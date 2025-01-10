mod command_line_args;
mod debugger;
mod executor;
mod parsing_src;
mod start;

use clap::Parser;

use command_line_args::Args;
use start::{debug, execute};

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
