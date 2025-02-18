use futures::channel::oneshot;
use futures::StreamExt;
use libp2p::multiaddr::Protocol;
use libp2p::multihash::Code;
use parking_lot::Mutex;
use std::sync::Arc;
use std::time::Duration;
use subspace_core_primitives::{crypto, PieceIndexHash, U256};
use subspace_networking::{BootstrappedNetworkingParameters, Config};

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let config_1 = Config {
        listen_on: vec!["/ip4/0.0.0.0/tcp/0".parse().unwrap()],
        allow_non_global_addresses_in_dht: true,
        ..Config::default()
    };
    let (node_1, mut node_runner_1) = subspace_networking::create(config_1).unwrap();

    println!("Node 1 ID is {}", node_1.id());

    let (node_1_address_sender, node_1_address_receiver) = oneshot::channel();
    let on_new_listener_handler = node_1.on_new_listener(Arc::new({
        let node_1_address_sender = Mutex::new(Some(node_1_address_sender));

        move |address| {
            if matches!(address.iter().next(), Some(Protocol::Ip4(_))) {
                if let Some(node_1_address_sender) = node_1_address_sender.lock().take() {
                    node_1_address_sender.send(address.clone()).unwrap();
                }
            }
        }
    }));

    tokio::spawn(async move {
        node_runner_1.run().await;
    });

    // Wait for first node to know its address
    let node_1_addr = node_1_address_receiver.await.unwrap();
    drop(on_new_listener_handler);

    let config_2 = Config {
        networking_parameters_registry: BootstrappedNetworkingParameters::new(vec![
            node_1_addr.with(Protocol::P2p(node_1.id().into()))
        ])
        .boxed(),
        listen_on: vec!["/ip4/0.0.0.0/tcp/0".parse().unwrap()],
        allow_non_global_addresses_in_dht: true,
        ..Config::default()
    };

    let (node_2, mut node_runner_2) = subspace_networking::create(config_2).unwrap();

    println!("Node 2 ID is {}", node_2.id());

    tokio::spawn(async move {
        node_runner_2.run().await;
    });

    tokio::time::sleep(Duration::from_secs(1)).await;

    let hashed_peer_id = PieceIndexHash::from(crypto::blake2b_256_hash(&node_1.id().to_bytes()));
    let key = libp2p::multihash::MultihashDigest::digest(
        &Code::Identity,
        &U256::from(hashed_peer_id).to_be_bytes(),
    );
    let peer_id = node_2
        .get_closest_peers(key)
        .await
        .unwrap()
        .next()
        .await
        .unwrap();
    assert_eq!(node_1.id(), peer_id);

    println!("Exiting..");
}
