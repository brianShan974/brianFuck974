pub mod command_line_args;

use clap::Parser;

use bfdbg::start::{debug, execute};
use command_line_args::Args;

fn main() {
    let args = Args::parse();

    let cmd_string = match std::fs::read_to_string(args.path) {
        Ok(string) => string,
        Err(_) => panic!("Failed to read from the file!"),
    };

    if args.debug {
        debug(&cmd_string);
    } else {
        execute(&cmd_string);
    }
}
