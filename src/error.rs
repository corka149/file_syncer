use std::error;
use std::fmt;

#[derive(Debug, Clone)]
pub struct PathError;

impl fmt::Display for PathError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "exptected path: Provided string does not define a path to a directory!")
    }
}

impl error::Error for PathError {
    fn description(&self) -> &str {
        "exptected path: Provided string does not define a path to a directory!"
    }

    fn cause(&self) -> Option<&error::Error> {
        None
    }
}

