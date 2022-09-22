pub mod config;
pub mod errors;
pub mod ipv4;
pub mod node;
pub mod routing;

pub use config::Config;
pub use node::Node;

pub const SUBWAY_PACKET_MARK: u32 = 0x50000;
pub const MAX_FRAME_SIZE: usize = u16::MAX as usize;
