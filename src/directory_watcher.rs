use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::{thread, time, time::SystemTime};
use std::error::Error;
use std::fs::{ReadDir, Metadata};

use error::PathError;

pub struct DirectoryWatcher<'a> {
    /// Indicates how often should the directory be checked.
    secs_interval: u64,
    file_register: HashMap<PathBuf, SystemTime>,
    dir: &'a Path,
}

impl<'a> DirectoryWatcher<'a> {
    /// Creates a new directory watcher. It must be defined an check interval in milliseconds
    /// and a path to the directory which should be watched.
    pub fn new(secs_interval: u64, path: &str) -> Result<DirectoryWatcher, PathError> {
        let path = Path::new(path);
        if !path.is_dir() {
            return Err(PathError);
        }

        Ok(DirectoryWatcher {
            secs_interval,
            file_register: HashMap::new(),
            dir: Path::new(path),
        })
    }
    
    /// Will wait for the setted interval and then emitt the files that were added oder changed.
    pub fn emitted_changed_files(&mut self) -> Result<Vec<PathBuf>, Box<Error>> {
        let millis = time::Duration::from_secs(self.secs_interval);
        let mut changed_files: Vec<PathBuf> = Vec::new();

        thread::sleep(millis);
        let path_cont: ReadDir = self.dir.read_dir()?;
        for dir_entry in path_cont {
            if let Ok(dir_entry) = dir_entry {
                let meta_data: Metadata = dir_entry.metadata()?;
                if meta_data.is_file() && self.is_new_file(&dir_entry.path()) {
                    let modified_time = meta_data.modified()?;
                    self.add_new_file(dir_entry.path(), modified_time);
                    changed_files.push(dir_entry.path());
                }
            }
        }

        Ok(changed_files)
    }

    /// Adds a file to the register if it was not in the file_register before.
    fn add_new_file(&mut self, path: PathBuf, modified_time: SystemTime) {
        if self.is_new_file(&path) {
            self.register_file(path, modified_time);
        }
    }

    /// Checks if the file was already registered before.
    fn is_new_file(&self, path: &PathBuf) -> bool {
        return !self.file_register.contains_key(path);
    }

    /// Registers file when it can be converted to a valid &str.
    fn register_file(&mut self, path: PathBuf, modified_time: SystemTime) {
        self.file_register.insert(path, modified_time);
    }
}

#[cfg(test)]
mod tests {

    use super::DirectoryWatcher;
    use std::fs::File;
    use std::io::prelude::*;
    use std::fs::remove_file;

    #[test]
    fn test_new_directorywatcher_path_ok() {
        let test = DirectoryWatcher::new(5, "./test.d");
        assert!(test.is_ok());
    }

    #[test]
    fn test_new_directorywatcher_error_path() {
        let test = DirectoryWatcher::new(5, "./test_.d");
        assert!(test.is_err());
    }

    #[test]
    fn test_emitted_changed_files_only_one_file_added(){
        let mut test = DirectoryWatcher::new(5, "./test.d/1").unwrap();
        for i in 0..5 {
            let files = test.emitted_changed_files().unwrap();
            if i == 0 {
                assert_eq!(1, files.len());
            } else {
                assert_eq!(0, files.len());
            }
        }
    }

    #[test]
    fn test_emitted_changed_files_one_file_added_and_add_another_one_afterwards(){

        let mut test = DirectoryWatcher::new(5, "./test.d/2").unwrap();
        try_delete_file();
        for i in 0..5 {
            let files = test.emitted_changed_files().unwrap();
            
            match i {
                0 => {
                        assert_eq!(1, files.len());                    
                        try_create_file();
                    },
                1 => assert_eq!(1, files.len()),
                _ => assert_eq!(0, files.len(), "{:?}", files)
            }
        }
        try_delete_file();
    }

    fn try_create_file() {
        let mut file = File::create("test.d/2/test.txt").unwrap();
        let _ = file.write_all(b"A simple text").unwrap();
    }

    fn try_delete_file() {
        let _ = remove_file("test.d/2/test.txt");
    }
}
