use std::fmt;
use std::io;

#[derive(Debug)]
pub enum SubwayError {
    BindUdp(io::Error),
    CreateTun(io::Error),
    IOError(io::Error),
}

impl std::error::Error for SubwayError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        use SubwayError::*;
        match self {
            BindUdp(e) => Some(e),
            CreateTun(e) => Some(e),
            IOError(e) => Some(e),
        }
    }
}

impl fmt::Display for SubwayError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use SubwayError::*;
        match self {
            BindUdp(_) => "Failed to bind UDP socket locally".fmt(f),
            CreateTun(_) => "Failed to create tun/tap interface".fmt(f),
            IOError(e) => write!(f, "{}", e.to_string()),
        }
    }
}
