use std::env;
use yaisd_core::process_manager;

const HELP_STR: &'static str = "run without arguments will start the daemon!
available commands:
--help
--version";

fn main() {
    let current_version = env!("CARGO_PKG_VERSION");
    let args: Vec<String> = env::args().collect();
    let command = parse_command(&args);

    if let Some(c_str) = command {
        if c_str == "--version" {
            println!("{}", current_version)
        } else if c_str == "--help" {
            println!("{}", HELP_STR)
        } else {
            process_manager::run();
        }
    } else {
        process_manager::run();
    }
}

fn parse_command(args: &Vec<String>) -> Option<&String> {
    if args.len() < 2 {
        return None;
    } else {
        Some(&args[1])
    }
}
