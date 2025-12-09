use internet_simulator::{Address, VirtualNetwork};

#[tokio::main]
async fn main() {
    let mut net = VirtualNetwork::new();

    let a = net.register_node(Address(1), Some("A".into())).await;
    let b = net.register_node(Address(2), Some("B".into())).await;

    // A -> B にメッセージ送信
    tokio::spawn({
        let a = a.clone();
        async move {
            a.send(Address(2), "Hello B!").await;
        }
    });

    // B が受信
    if let Some(msg) = b.recv().await {
        println!("[{}] got: {}", b.address, msg.payload);
    }
}
