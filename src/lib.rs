extern crate clap;

pub mod directory_watcher;
pub mod error;
mod cli_config;

use cli_config::CliConfig;
use clap::{App, Arg, ArgMatches};

pub fn run_syncer() {
    let args = CliConfig::create_cli_config();
    let matches = App::new("Keep files in sync on two different maschines.")
        .args(&args.cli_args)
        .get_matches();
    println!("{:?}", matches);
}
