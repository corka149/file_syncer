pub mod execution_mode;

use clap::Arg;
use self::execution_mode::ExecutionMode;

const PATH: &str = "path";
const MODE: &str = "mode";
const COMMAND: &str = "command";
const file_filter: &str = "filter";

const APP_NAME: &str = env!("CARGO_PKG_NAME");
const APP_VERSION: &str = env!("CARGO_PKG_VERSION");


pub struct ExtractedArgs<'a> {
    pub path: &'a str,
    pub mode: execution_mode::ExecutionMode,
    pub command: &'a str,
    pub file_filter: &'a str
}

// Lifetime 'a must outlive 'b
pub struct CliConfig<'b, 'a: 'b> {
    cli_args: Vec<Arg<'a, 'b>>
}

impl<'b, 'a: 'b> CliConfig<'a, 'b>{

    pub fn create_cli_config() -> CliConfig<'a, 'b> {
        let mut config: Vec<Arg> = Vec::new();
        config.push(CliConfig::create_path_conf());
        config.push(CliConfig::create_mode_conf());
        CliConfig{
            cli_args: config
        }
    }

    fn create_path_conf() -> Arg<'a, 'b> {
        Arg::with_name(PATH)
            .help("Specifies the directory which should be observered.")
            .long(PATH)
            .short("F")
            .required(true)
    }

    fn create_mode_conf() -> Arg<'a, 'b> {
        Arg::with_name(MODE)
            .help("Mode in which file_syncer runs.")
            .long(MODE)
            .short("M")
            .default_value(ExecutionMode::AUTONOMOUS)
    }
}

