pub mod execution_mode;
pub mod extracted_args;

use clap::{Arg, ArgMatches};

use self::execution_mode::ExecutionMode;
use super::error::PathError;
use self::extracted_args::ExtractedArgs;

const PATH: &str = "path";
const MODE: &str = "mode";
const COMMAND: &str = "command";
const FILE_FILTER: &str = "filter";
const PORT: &str = "port";
const HOST: &str = "host";


// Lifetime 'a must outlive 'b
pub struct CliConfig<'b, 'a: 'b> {
    cli_args: Vec<Arg<'a, 'b>>
}

impl<'b, 'a: 'b> CliConfig<'a, 'b>{

    pub fn create_cli_config() -> CliConfig<'a, 'b> {
        let mut config: Vec<Arg> = Vec::new();
        config.push(CliConfig::create_path_conf());
        config.push(CliConfig::create_mode_conf());
        config.push(CliConfig::create_command_conf());
        config.push(CliConfig::create_filter_conf());
        config.push(CliConfig::create_port_conf());
        config.push(CliConfig::create_host_conf());
        CliConfig{
            cli_args: config
        }
    }

    pub fn get_cli_args(&'a self) -> &'a [Arg<'a , 'b>] {
        &self.cli_args
    }

    pub fn extract_args(matches: ArgMatches) -> Result<ExtractedArgs, PathError> {
        let path = match matches.value_of(PATH) {
            Some(val) => val,
            None =>  return Err(PathError)
        };
        let mode = matches.value_of(MODE);
        let mode = ExecutionMode::determine_mode(mode);

        Ok(ExtractedArgs{
            path: String::from(path),
            mode: mode,
            command: None,
            file_filter: None
        })
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
            .help("The filter will applied before sending or requesting on each file name.")
            .long(FILE_FILTER)
            .short("F")
            .takes_value(true)
    }

    fn create_port_conf() -> Arg<'a, 'b> {
        Arg::with_name(PORT)
            .help("The port nummer which will be used for hosting in server mode.")
            .long(PORT)
            .short("P")
            .required_if(MODE, "S")
            .takes_value(true)
    }

    fn create_host_conf() -> Arg<'a, 'b> {
        Arg::with_name(HOST)
            .help("Specifies to which host the client should connect in client mode. E.g.: 192.168.0.1:8000")
            .long(HOST)
            .short("H")
            .required_if(MODE, "C")
            .takes_value(true)
    }
}

