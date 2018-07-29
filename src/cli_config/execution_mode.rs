use std::net::SocketAddr;

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
}