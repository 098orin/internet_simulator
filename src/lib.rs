use std::{collections::HashMap, sync::Arc};
use tokio::sync::{RwLock, mpsc};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Address(pub u32);

impl Address {
    pub fn from_octets(a: u8, b: u8, c: u8, d: u8) -> Self {
        Self(((a as u32) << 24) | ((b as u32) << 16) | ((c as u32) << 8) | (d as u32))
    }
}

/// ネットワーク上のメッセージ
#[derive(Debug, Clone)]
pub struct Message {
    pub src: Address,
    pub dst: Address,
    pub payload: Vec<u8>,
}

/// ノードから見えるネットワークインターフェース
#[derive(Clone)]
pub struct NetHandle {
    pub address: Address,
    pub label: Option<String>,
    sender: mpsc::Sender<Message>,
    receiver: Arc<RwLock<mpsc::Receiver<Message>>>,
}

impl NetHandle {
    pub async fn send(&self, dst: Address, payload: Vec<u8>) {
        let msg = Message {
            src: self.address,
            dst,
            payload,
        };
        let _ = self.sender.send(msg).await;
    }

    pub async fn recv(&self) -> Option<Message> {
        let mut rx = self.receiver.write().await;
        rx.recv().await
    }
}

/// 仮想ネットワーク本体
pub struct VirtualNetwork {
    nodes: HashMap<Address, mpsc::Sender<Message>>,
}

impl VirtualNetwork {
    pub fn new() -> Self {
        Self {
            nodes: HashMap::new(),
        }
    }

    pub fn register_node(&mut self, address: Address, label: Option<String>) -> NetHandle {
        let (tx_net, mut rx_net) = mpsc::channel::<Message>(128);
        let (tx_node, rx_node) = mpsc::channel::<Message>(128);

        self.nodes.insert(address, tx_node);

        // routing loop
        let nodes = self.nodes.clone();
        tokio::spawn(async move {
            while let Some(msg) = rx_net.recv().await {
                if let Some(dst_tx) = nodes.get(&msg.dst) {
                    let _ = dst_tx.send(msg).await;
                }
                // TODO: 遅延・パケットロスここに追加可能
            }
        });

        NetHandle {
            address,
            label,
            sender: tx_net,
            receiver: Arc::new(RwLock::new(rx_node)),
        }
    }
}
