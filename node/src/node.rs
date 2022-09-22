use std::sync::Arc;
use tokio::net::UdpSocket;

use crate::config::Config;
use crate::errors::SubwayError;
use crate::SUBWAY_PACKET_MARK;

use tokio_tun::{Tun, TunBuilder};

use std::os::unix::io::AsRawFd;

#[allow(dead_code)]
pub struct Node {
    cfg: Config<'static>,
    socket: Arc<UdpSocket>,
    tun: Tun,
}

impl Node {
    pub async fn new(cfg: Config<'static>) -> Result<Node, SubwayError> {
        let tun = TunBuilder::new()
            .name(cfg.interface_name)
            .tap(false)
            .packet_info(true)
            .try_build()
            .expect("failed to create virtual tunnel");

        let socket = Arc::new(
            UdpSocket::bind(cfg.tunnel_entry)
                .await
                .map_err(SubwayError::BindUdp)?,
        );

        let result = unsafe {
            libc::setsockopt(
                socket.as_raw_fd(),
                libc::SOL_SOCKET,
                libc::SO_MARK,
                &SUBWAY_PACKET_MARK as *const u32 as *const libc::c_void,
                std::mem::size_of_val(&SUBWAY_PACKET_MARK) as _,
            )
        };
        if result == -1 {
            panic!("failed to set subway fwmark on socket")
        }

        return Ok(Node { cfg, tun, socket });
    }
}
