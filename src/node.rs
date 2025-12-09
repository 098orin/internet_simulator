use tokio::sync::{RwLock, mpsc};
use std::sync::Arc;

use crate::{Address, Message};

#[derive(Clone)]
pub struct NetHandle {
    pub address: Address,
    pub label: Option<String>,
    pub tx_to_router: mpsc::Sender<Message>,
    pub rx_from_router: Arc<RwLock<mpsc::Receiver<Message>>>,
}

impl NetHandle {
    pub fn new(
        address: Address,
        label: Option<String>,
        tx_to_router: mpsc::Sender<Message>,
        rx_node: mpsc::Receiver<Message>,
    ) -> Self {
        Self {
            address,
            label,
            tx_to_router,
            rx_from_router: Arc::new(RwLock::new(rx_node)),
        }
    }

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

impl std::fmt::Display for NetHandle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.label {
            Some(label) => write!(f, "{}[{}]", label, self.address),
            None => write!(f, "[{}]", self.address),
        }
    }
}
