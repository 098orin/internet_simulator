# Internet Simulator for rust
A simple network simulator written in Rust.

## Example
```rs
use internet_simulator::{Net, Address};
use tokio::time::{sleep, Duration};

#[tokio::main]
async fn main() {
    let net = Net::new();

    let a = net.register_node(Address(1), Some("A".into())).await;
    let b = net.register_node(Address(2), Some("B".into())).await;

    // A → B 
    a.send(Address(2), "hello!").await;

    // B receives
    if let Some(msg) = b.recv().await {
        println!("B received: {:?}", msg);
    }
}
```
