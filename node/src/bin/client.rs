use subway::{Config, Node};
use tokio::{signal, sync::broadcast, task};

fn main() {
    let (cancel, _) = broadcast::channel::<()>(10);
    let _ = cancel.subscribe();

    let _ = task::spawn(async move {
        let _ = signal::ctrl_c().await;
        let _ = cancel.send(());
    });

    let cfg = Config {
        route_table_name: "subway",
        rt_index: &77,
        network: "150.150.150.1/24",
        interface_name: "tun0",
        tunnel_entry: "0.0.0.0:5678",
        tunnel_exit: "172.18.0.20:5678",
    };

    _ = Node::new(cfg);

    ();
}
