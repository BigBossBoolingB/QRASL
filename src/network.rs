use libp2p::{
    gossipsub::{
        Gossipsub, GossipsubEvent, GossipsubMessage, IdentTopic as Topic, MessageAuthenticity,
        ValidationMode,
    },
    mdns::{Mdns, MdnsEvent},
    swarm::{NetworkBehaviourEventProcess, SwarmBuilder},
    NetworkBehaviour, PeerId, Swarm,
};
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use tokio::sync::mpsc;

#[derive(NetworkBehaviour)]
#[behaviour(event_process = true)]
pub struct QraslBehaviour {
    pub gossipsub: Gossipsub,
    pub mdns: Mdns,
    #[behaviour(ignore)]
    pub block_tx: mpsc::UnboundedSender<String>,
    #[behaviour(ignore)]
    pub tx_tx: mpsc::UnboundedSender<String>,
}

impl NetworkBehaviourEventProcess<GossipsubEvent> for QraslBehaviour {
    fn inject_event(&mut self, event: GossipsubEvent) {
        if let GossipsubEvent::Message {
            propagation_source: _,
            message_id: _,
            message,
        } = event
        {
            let msg_str = String::from_utf8_lossy(&message.data);
            if message.topic == Topic::new("new-blocks").hash() {
                if let Err(e) = self.block_tx.send(msg_str.to_string()) {
                    eprintln!("Error sending block to main loop: {}", e);
                }
            } else if message.topic == Topic::new("unconfirmed-transactions").hash() {
                if let Err(e) = self.tx_tx.send(msg_str.to_string()) {
                    eprintln!("Error sending tx to main loop: {}", e);
                }
            }
        }
    }
}

impl NetworkBehaviourEventProcess<MdnsEvent> for QraslBehaviour {
    fn inject_event(&mut self, event: MdnsEvent) {
        match event {
            MdnsEvent::Discovered(list) => {
                for (peer, _) in list {
                    self.gossipsub.add_explicit_peer(&peer);
                }
            }
            MdnsEvent::Expired(list) => {
                for (peer, _) in list {
                    if !self.mdns.has_node(&peer) {
                        self.gossipsub.remove_explicit_peer(&peer);
                    }
                }
            }
        }
    }
}

pub async fn create_swarm(
    block_tx: mpsc::UnboundedSender<String>,
    tx_tx: mpsc::UnboundedSender<String>,
) -> Result<Swarm<QraslBehaviour>, Box<dyn std::error::Error>> {
    let local_key = libp2p::identity::Keypair::generate_ed25519();
    let local_peer_id = PeerId::from(local_key.public());
    println!("Local peer id: {:?}", local_peer_id);

    let transport = libp2p::development_transport(local_key.clone()).await?;

    let message_id_fn = |message: &GossipsubMessage| {
        let mut s = DefaultHasher::new();
        message.data.hash(&mut s);
        s.finish().to_string()
    };

    let gossipsub_config = libp2p::gossipsub::GossipsubConfigBuilder::default()
        .heartbeat_interval(std::time::Duration::from_secs(10))
        .validation_mode(ValidationMode::Strict)
        .message_id_fn(message_id_fn)
        .build()
        .expect("Valid config");

    let mut gossipsub: Gossipsub = Gossipsub::new(MessageAuthenticity::Signed(local_key), gossipsub_config)
        .expect("Correct configuration");

    let topic = Topic::new("new-blocks");
    gossipsub.subscribe(&topic)?;

    let mdns = Mdns::new(Default::default()).await?;
    let behaviour = QraslBehaviour {
        gossipsub,
        mdns,
        tx,
    };
    let mut swarm = SwarmBuilder::new(transport, behaviour, local_peer_id)
        .executor(Box::new(|fut| {
            tokio::spawn(fut);
        }))
        .build();

    swarm.listen_on("/ip4/0.0.0.0/tcp/0".parse()?)?;

    Ok(swarm)
}
