use ::cli_config::extracted_args::ExtractedArgs;
use ::directory_watcher::DirectoryWatcher;
use ::error::PathError;

pub fn run(args: &ExtractedArgs) -> Result<(), PathError> {
    let mut watcher = DirectoryWatcher::new(5, &args.path)?;
    
    while let Ok(changed_files) = watcher.emitted_changed_files() {
        for path in changed_files {
            println!("{:?}", path);
        }
    }
    
    Ok(())
}