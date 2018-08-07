use std::net::SocketAddr;
use std::fmt;

/// Tests if two &str do equal
#[macro_export]
macro_rules! do_str_equal {
    ($first:expr, $second:expr) =>   {
        // Expands to an expression
        {
            let mut equals = true;
            let mut chars = $second.chars();
            for x in $first.chars() {
                if let Some(y) = chars.next() {
                    if x != y {
                        equals = false;
                    }
                }
            }
            equals
        }
    }
}

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

    pub fn determine_mode(possible_mode: Option<&str>, possible_host: Option<&str>,
        possible_port: Option<&str>) -> Option<ExecutionMode> {

        match possible_mode {
            Some(mode) => {
                if do_str_equal!(ExecutionMode::CLIENT, mode) {
                    ExecutionMode::create_client(possible_host)
                } else if do_str_equal!(ExecutionMode::SERVER, mode) {
                    ExecutionMode::create_server(possible_port)
                } else if do_str_equal!(ExecutionMode::AUTONOMOUS, mode) {                    
                    Some(ExecutionMode::Autonomous)
                } else {
                    println!("Warning! Unknown mode detected '{}'. Switched to default '{}'.", 
                        mode, ExecutionMode::AUTONOMOUS);
                    Some(ExecutionMode::Autonomous)
                }
            }, 
            None => None
        }
    }

    fn create_client(possible_host: Option<&str>) -> Option<ExecutionMode> {
        let ip_socket = ExecutionMode::extract_ip_socket(possible_host);
        match ip_socket {
            Some(socket) => Some(ExecutionMode::Client(socket)),
            None => None
        }
    }

    fn extract_ip_socket(possible_host: Option<&str>) -> Option<SocketAddr> {
        // let socket = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 8080);
        match possible_host {
            Some(host) => {
                let host = host.parse::<SocketAddr>();
                match host {
                    Ok(socket_addr) => Some(socket_addr),
                    Err(e) =>  {
                        eprintln!("{:?}", e);
                        None
                    }
                }
            },
            None => None
        }
    }

    fn create_server(possible_port: Option<&str>) -> Option<ExecutionMode> {
        let port = ExecutionMode::extract_port(possible_port);
        match port {
            Some(port) => Some(ExecutionMode::Server(port)),
            None => None
        }   
    }

    fn extract_port(possible_port: Option<&str>) -> Option<u16> {
        match possible_port {
            Some(val) => {
                let port = val.parse::<u16>();
                match port {
                    Ok(port) =>  Some(port),
                    Err(e) =>  {
                        eprintln!("{:?}", e);
                        None
                    }
                }
            },
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

#[cfg(test)]
mod tests {

    #[test]
    fn test_do_str_equal_true() {
        assert!(do_str_equal!("yes", "yes"));
    }

    #[test]
    fn test_do_str_equal_false() {
        assert!(!do_str_equal!("yes", "no"));
    }
    
    #[test]
    fn test_do_str_equal_similar_case_different_str_false() {
        assert!(!do_str_equal!("yes", "yEs"));
    }
    
    #[test]
    fn test_do_str_equal_similar_str_false() {
        assert!(!do_str_equal!("yes", " yes"));
    }

}