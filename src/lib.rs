//! # Internet Simulator Library
//!
//! This crate provides a lightweight virtual network simulator designed
//! for experimenting with distributed systems, routing algorithms,
//! and peer-to-peer protocols.
//!
//! The simulator creates virtual nodes that communicate with each other
//! through an in-memory router. Each node can send and receive messages
//! asynchronously without relying on real network sockets.
//!
//! ## Key Features
//! - Virtual message passing between `Address`es  
//! - Asynchronous routing using `tokio`  
//! - Easy creation and registration of nodes  
//! - Extensible design suitable for DHTs or overlay networks  
//!
//! ## Example
//! ```rust
//! use internet_simulator::{Network, Address};
//! 
//! #[tokio::main]
//! async fn main() {
//!     let net = Network::new();
//!
//!     let a = net.register_node(Address(1), Some("A".into())).await;
//!     let b = net.register_node(Address(2), Some("B".into())).await;
//!
//!     a.send(Address(2), "hello").await;
//!
//!     if let Some(msg) = b.recv().await {
//!         println!("B received: {:?}", msg);
//!     }
//! }
//! ```

pub mod network;
pub mod node;
pub mod message;

/// Virtual network implementation and address type.
pub use network::Network;

/// Handle for interacting with a node (sending/receiving messages).
pub use node::NetHandle;

/// Structure representing a routed message.
pub use message::Message;

/// Unique numerical address for a virtual node.
pub use network::Address;
