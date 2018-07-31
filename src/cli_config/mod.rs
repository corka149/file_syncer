pub mod execution_mode;

use clap::Arg;
use self::execution_mode::ExecutionMode;

const PATH: &str = "path";
const MODE: &str = "mode";
const COMMAND: &str = "command";
const FILE_FILTER: &str = "filter";

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
    pub cli_args: Vec<Arg<'a, 'b>>
}

impl<'b, 'a: 'b> CliConfig<'a, 'b>{

    pub fn create_cli_config() -> CliConfig<'a, 'b> {
        let mut config: Vec<Arg> = Vec::new();
        config.push(CliConfig::create_path_conf());
        config.push(CliConfig::create_mode_conf());
        config.push(CliConfig::create_command_conf());
        config.push(CliConfig::create_filter_conf());
        CliConfig{
            cli_args: config
        }
    }

    fn create_path_conf() -> Arg<'a, 'b> {
        Arg::with_name(PATH)
            .help("Specifies the directory which should be observered.")
            .required(true)
            .index(1)
    }

    fn create_mode_conf() -> Arg<'a, 'b> {
        Arg::with_name(MODE)
            .help("Mode in which file_syncer runs.")
            .long(MODE)
            .short("M")
            .default_value(ExecutionMode::AUTONOMOUS)
            .takes_value(true)
    }

    fn create_command_conf() -> Arg<'a, 'b> {
        Arg::with_name(COMMAND)
            .help("The provided command will be executed:
    o Server mode - before sending
    o Client mode - after receiving
    o Autonomous mode - after change detected")
            .long(COMMAND)
            .short("C")
            .takes_value(true)
    }

    fn create_filter_conf() -> Arg<'a, 'b> {
        Arg::with_name(FILE_FILTER)
            .help("The filter will applied before sending or requesting on each file name")
            .long(FILE_FILTER)
            .short("F")
            .takes_value(true)
    }
}

