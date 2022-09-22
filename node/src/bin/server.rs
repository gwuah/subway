use futures::try_join;
use subway::{routing::Router, Config, Node};
use tokio::{signal, sync::broadcast, task};

#[tokio::main]
async fn main() {
    let (cancel, _) = broadcast::channel::<()>(10);
    let sig_cancel = cancel.clone();
    let node_cancel = cancel.clone();

    _ = task::spawn(async move {
        _ = signal::ctrl_c().await;
        _ = sig_cancel.send(());
    });

    let cfg = Config {
        route_table_name: "subway",
        rt_index: &77,
        network: "150.150.150.2/24",
        interface_name: "tun0",
        tunnel_entry: "0.0.0.0:5678",
        tunnel_exit: "172.18.0.10:5678",
    };

    let router = Router::new(cfg);

    let node = match Node::new(cfg, router, node_cancel).await {
        Ok(node) => node,
        Err(e) => panic!("failed to initialize node. err={}", e),
    };

    if let Err(e) = router.configure_nat_rules() {
        panic!("failed to configure NAT rules, non-negotiable{}", e)
    }

    let i2t = node.clone();
    let interface_to_tunnel = task::spawn(async move { i2t.read_interface_write_tunnel().await });

    let t2i = node.clone();
    let tunnel_to_interface = task::spawn(async move { t2i.read_tunnel_write_interface().await });

    println!("subways is running on this machine ...");

    if let Err(e) = try_join!(interface_to_tunnel, tunnel_to_interface) {
        println!("yawa dey! err={}", e)
    };

    return ();
}
