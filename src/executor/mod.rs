use super::cli_config::extracted_args::ExtractedArgs;
use super::cli_config::execution_mode::ExecutionMode;

mod autonomous_executor;

pub fn execute_mode(args: ExtractedArgs) {

    match args.mode {
        Some(ref mode) => {
            let result = match mode {
                ExecutionMode::Autonomous => autonomous_executor::run(&args),
                ExecutionMode::Server(_) => {
                    println!("Not implemented yet!");
                    Ok(())
                },
                ExecutionMode::Client(_) => {
                    println!("Not implemented yet!");
                    Ok(())
                }
            };
            if let Err(e) = result {
                eprintln!("{}", e);
            }
        },
        None => eprintln!("No valid mode found! Program exits now!")

    }

}