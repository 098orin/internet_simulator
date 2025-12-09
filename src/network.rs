use std::collections::HashMap;

use tokio::sync::{RwLock, mpsc};
use std::sync::Arc;

use crate::node::NetHandle;
use crate::message::Message;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Address(pub u32);

impl Address {
    pub fn from_octets(a: u8, b: u8, c: u8, d: u8) -> Self {
        Self(((a as u32) << 24) | ((b as u32) << 16) | ((c as u32) << 8) | (d as u32))
    }

    pub fn octets(&self) -> (u8, u8, u8, u8) {
        (
            (self.0 >> 24) as u8,
            (self.0 >> 16) as u8,
            (self.0 >> 8) as u8,
            (self.0) as u8,
        )
    }
}

impl std::fmt::Display for Address {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let (a, b, c, d) = self.octets();
        write!(f, "{}.{}.{}.{}", a, b, c, d)
    }
}

pub struct Network {
    nodes: Arc<RwLock<HashMap<Address, mpsc::Sender<Message>>>>,
    tx_router: mpsc::Sender<Message>,
}

impl Network {
    pub fn new() -> Self {
        let (tx_router, mut rx_router) = mpsc::channel::<Message>(128);
        let nodes = Arc::new(RwLock::new(HashMap::<Address, mpsc::Sender<Message>>::new()));
        let nodes_clone = nodes.clone();

        // ルーターループ開始（非公開）
        tokio::spawn(async move {
            while let Some(msg) = rx_router.recv().await {
                let map = nodes_clone.read().await;
                if let Some(dst) = map.get(&msg.dst) {
                    let _ = dst.send(msg).await;
                }
            }
        });

        Self { nodes, tx_router }
    }

    pub async fn register_node(
        &self,
        address: Address,
        label: Option<String>,
    ) -> NetHandle {
        let (tx_node, rx_node) = mpsc::channel(128);

        {
            let mut map = self.nodes.write().await;
            map.insert(address, tx_node);
        }

        NetHandle::new(address, label, self.tx_router.clone(), rx_node)
    }
}
