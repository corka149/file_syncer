use super::execution_mode::{self, ExecutionMode};
use std::fmt;


pub struct ExtractedArgs {
    pub path: String,
    pub mode: Option<execution_mode::ExecutionMode>,
    pub command: Option<String>,
    pub file_filter: Option<String>
}

impl fmt::Display for ExtractedArgs {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mode = match self.mode {
            // ref because we want to take ownership
            Some(ref val) => val,
            None => &ExecutionMode::Autonomous
        };
        let command = match self.command {
            Some(ref val) => val,
            None => "none"
        };
        let filter = match self.file_filter {
            Some(ref val) => val,
            None => "none"
        };

        write!(f, "(path '{}', mode '{}', command '{}', file filter '{}')", 
            self.path, mode, command, filter)
    }
}

