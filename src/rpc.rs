use crate::mempool::Mempool;
use crate::primitives::{Address, Block, Transaction};
use jsonrpsee::{
    core::{async_trait, RpcResult},
    proc_macros::rpc,
    ws_server::{WsServerBuilder, WsServerHandle},
};
use std::net::SocketAddr;
use std::sync::{Arc, Mutex};
use crate::chain::Chain;

#[rpc(server)]
pub trait QraslRpc {
    #[method(name = "qrasl_getLatestBlock")]
    async fn get_latest_block(&self) -> RpcResult<Option<Block>>;

    #[method(name = "qrasl_getBalance")]
    async fn get_balance(&self, account: Address) -> RpcResult<u128>;

    #[method(name = "qrasl_getNftOwner")]
    async fn get_nft_owner(&self, collection_id: u64, token_id: u64) -> RpcResult<Option<Address>>;

    #[method(name = "qrasl_submitTransaction")]
    async fn submit_transaction(&self, tx: Transaction) -> RpcResult<()>;
}

use libp2p::gossipsub::IdentTopic as Topic;
use libp2p::swarm::Swarm;

pub struct QraslRpcServer {
    chain: Arc<Mutex<Chain>>,
    mempool: Arc<Mutex<Mempool>>,
    swarm: Swarm<crate::network::QraslBehaviour>,
    tx_topic: Topic,
}

impl QraslRpcServer {
    pub fn new(
        chain: Arc<Mutex<Chain>>,
        mempool: Arc<Mutex<Mempool>>,
        swarm: Swarm<crate::network::QraslBehaviour>,
        tx_topic: Topic,
    ) -> Self {
        Self {
            chain,
            mempool,
            swarm,
            tx_topic,
        }
    }
}

#[async_trait]
impl QraslRpcServer for QraslRpcServer {
    async fn get_latest_block(&self) -> RpcResult<Option<Block>> {
        let chain = self.chain.lock().unwrap();
        let tip_hash = chain.db.get(b"tip").unwrap().unwrap();
        let block_bytes = chain.db.get(&tip_hash).unwrap().unwrap();
        let block: Block = bincode::deserialize(&block_bytes).unwrap();
        Ok(Some(block))
    }

    async fn get_balance(&self, account: Address) -> RpcResult<u128> {
        let chain = self.chain.lock().unwrap();
        let balance = chain
            .db
            .get(&bincode::serialize(&account).unwrap())
            .unwrap()
            .map(|v| bincode::deserialize(&v).unwrap())
            .unwrap_or(0);
        Ok(balance)
    }

    async fn get_nft_owner(&self, collection_id: u64, token_id: u64) -> RpcResult<Option<Address>> {
        let chain = self.chain.lock().unwrap();
        let nft_id = crate::nft::NftId {
            collection_id,
            token_id,
        };
        if let Some(nft_bytes) = chain.db.get(&bincode::serialize(&nft_id).unwrap()).unwrap() {
            let nft: crate::nft::NonFungibleToken = bincode::deserialize(&nft_bytes).unwrap();
            Ok(Some(nft.owner))
        } else {
            Ok(None)
        }
    }

    async fn submit_transaction(&self, tx: Transaction) -> RpcResult<()> {
        let mut mempool = self.mempool.lock().unwrap();
        mempool
            .add_transaction(tx.clone())
            .map_err(|e| jsonrpsee::core::Error::Custom(e.to_string()))?;

        let tx_json = serde_json::to_string(&tx).unwrap();
        self.swarm
            .behaviour_mut()
            .gossipsub
            .publish(self.tx_topic.clone(), tx_json.as_bytes())
            .map_err(|e| jsonrpsee::core::Error::Custom(e.to_string()))?;

        Ok(())
    }
}

use libp2p::gossipsub::IdentTopic as Topic;
use libp2p::swarm::Swarm;

pub async fn run_rpc_server(
    chain: Arc<Mutex<Chain>>,
    mempool: Arc<Mutex<Mempool>>,
    mut swarm: Swarm<crate::network::QraslBehaviour>,
    tx_topic: Topic,
) -> Result<SocketAddr, anyhow::Error> {
    let server = WsServerBuilder::default().build("127.0.0.1:0").await?;
    let addr = server.local_addr()?;
    let handle = server.start(QraslRpcServer::new(chain, mempool, swarm, tx_topic).into_rpc())?;

    tokio::spawn(handle.stopped());

    Ok(addr)
}
