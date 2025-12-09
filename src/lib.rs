use std::{collections::HashMap, sync::Arc};
use tokio::sync::{RwLock, mpsc};

/// ネットワーク上のメッセージ
#[derive(Debug, Clone)]
pub struct Message {
    pub src: Address,
    pub dst: Address,
    pub payload: String,
}

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

/// ノードから見えるネットワークインターフェース
#[derive(Clone)]
pub struct NetHandle {
    pub address: Address,
    pub label: Option<String>,
    pub tx_to_router: mpsc::Sender<Message>,
    pub rx_from_router: Arc<RwLock<mpsc::Receiver<Message>>>,
}

impl NetHandle {
    pub async fn send(&self, dst: Address, payload: impl Into<String>) {
        let msg = Message {
            src: self.address,
            dst,
            payload: payload.into(),
        };
        let _ = self.tx_to_router.send(msg).await;
    }

    pub async fn recv(&self) -> Option<Message> {
        let mut lock = self.rx_from_router.write().await;
        lock.recv().await
    }
}

/// 仮想ネットワーク本体
pub struct VirtualNetwork {
    nodes: Arc<RwLock<HashMap<Address, mpsc::Sender<Message>>>>,
    tx_router: mpsc::Sender<Message>,
}

impl VirtualNetwork {
    pub fn new() -> Self {
        let (tx_router, mut rx_router) = mpsc::channel::<Message>(128);
        let nodes = Arc::new(RwLock::new(HashMap::<Address, mpsc::Sender<Message>>::new()));

        // ルータースレッドは常に1つ
        {
            let nodes = nodes.clone();
            tokio::spawn(async move {
                while let Some(msg) = rx_router.recv().await {
                    let table = nodes.read().await;

                    if let Some(dst_tx) = table.get(&msg.dst) {
                        // 宛先ノードへ届ける
                        let _ = dst_tx.send(msg).await;
                    }
                    // 遅延とか実装できる
                }
            });
        }

        Self { nodes, tx_router }
    }

    pub async fn register_node(&mut self, address: Address, label: Option<String>) -> NetHandle {
        // ノードの受信チャンネル
        let (tx_node, rx_node) = mpsc::channel(128);

        // ルーターテーブルに登録（即反映！）
        {
            let mut table = self.nodes.write().await;
            table.insert(address, tx_node);
        }

        // 全ノード共通の tx_router を clone
        NetHandle {
            address,
            label,
            tx_to_router: self.tx_router.clone(),
            rx_from_router: Arc::new(RwLock::new(rx_node)),
        }
    }
}
