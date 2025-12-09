use internet_simulator::{Address, VirtualNetwork};

#[tokio::main]
async fn main() {
    let mut net = VirtualNetwork::new();

    let a = net.register_node(Address::from_octets(10, 0, 0, 1), Some("node-A".into()));
    let b = net.register_node(Address::from_octets(10, 0, 0, 2), Some("node-B".into()));

    tokio::spawn(async move {
        while let Some(msg) = a.recv().await {
            println!("A({:?}) <- {:?}", a.label, msg);
        }
    });

    tokio::spawn(async move {
        b.send(Address::from_octets(10, 0, 0, 1), b"PING".to_vec())
            .await;
    });

    tokio::signal::ctrl_c().await.unwrap();
}
