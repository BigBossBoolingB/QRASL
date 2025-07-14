use crate::primitives::{Address, Block};
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
}

pub struct QraslRpcServer {
    chain: Arc<Mutex<Chain>>,
}

impl QraslRpcServer {
    pub fn new(chain: Arc<Mutex<Chain>>) -> Self {
        Self { chain }
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
}

pub async fn run_rpc_server(chain: Arc<Mutex<Chain>>) -> Result<SocketAddr, anyhow::Error> {
    let server = WsServerBuilder::default().build("127.0.0.1:0").await?;
    let addr = server.local_addr()?;
    let handle = server.start(QraslRpcServer::new(chain).into_rpc())?;

    tokio::spawn(handle.stopped());

    Ok(addr)
}
