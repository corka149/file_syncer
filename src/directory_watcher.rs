use std::collections::HashMap;
use std::path::Path;

use error::PathError;

pub struct DirectoryWatcher<'a> {
    /// Indicates how often should the directory be checked.
    ms_interval: u32,
    file_register: HashMap<String, u32>,
    dir: &'a Path,
}

impl<'a> DirectoryWatcher<'a> {
    /// Creates a new directory watcher. It must be defined an check interval in milliseconds
    /// and a path to the directory which should be watched.
    pub fn new(ms_interval: u32, path: &str) -> Result<DirectoryWatcher, PathError> {
        let path = Path::new(path);
        if !path.is_dir() {
            return Err(PathError);
        }

        Ok(DirectoryWatcher {
            ms_interval,
            file_register: HashMap::new(),
            dir: Path::new(path),
        })
    }

    pub fn is_file_open(&self) -> bool {
        false
    }
}

#[cfg(test)]
mod tests {

    use super::DirectoryWatcher;

    #[test]
    fn test_new_directorywatcher_path_ok() {
        let test = DirectoryWatcher::new(1000, "./test.d");
        assert!(test.is_ok());
    }

    #[test]
    fn test_new_directorywatcher_error_path() {
        let test = DirectoryWatcher::new(1000, "./test_.d");
        assert!(test.is_err());
    }
}
