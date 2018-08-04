use std::net::{SocketAddr, IpAddr, Ipv4Addr};
use std::fmt;

/// Determines in which mode file_syncer will be started.
pub enum ExecutionMode{
    Autonomous,
    Server(u16),
    Client(SocketAddr)
}

impl ExecutionMode {

    pub const AUTONOMOUS: &'static str = "A";
    pub const SERVER: &'static str = "S";
    pub const CLIENT: &'static str = "C";

    pub fn determine_mode(possible_mode: Option<&str>) -> Option<ExecutionMode> {
        let socket = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 8080);

        match possible_mode {
            Some(_) => Some(ExecutionMode::Autonomous), 
            None => None
        }
    }
}

impl fmt::Display for ExecutionMode {

    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let text = match self {
            ExecutionMode::Autonomous => ExecutionMode::AUTONOMOUS,
            ExecutionMode::Server(_) => ExecutionMode::SERVER,
            ExecutionMode::Client(_) => ExecutionMode::CLIENT 
        };

        write!(f, "{}", text)
    }

}
