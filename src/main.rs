mod command;
mod execution_state;
mod parsing_src;
mod running_state;

use clap::{App, Arg};

use command::Command;
use execution_state::ExecutionState;
use parsing_src::translate_into_commands;
use running_state::RunningState;

fn main() {
    use std::fs;
    let args = App::new("brainfuck")
        .version("0.1")
        .about("A brainfuck interpreter written in rust.")
        .arg(Arg::with_name("mode")
             .help("Has 2 possible values: 'f' and 'c'. 'f' stands for file mode, and 'c' stands for command line mode.")
             .takes_value(true)
             .required(true))
        .arg(Arg::with_name("input")
             .help("In file mode, a file name should be provided. In command line mode, a string of source could should be provided in the command line.")
             .takes_value(true)
             .required(true))
        .get_matches();

    let mode = args.value_of("mode").unwrap();
    let input = args.value_of("input").unwrap();

    let mut cmd_string = String::new();

    if mode == "c" {
        cmd_string += input;
    } else if mode == "f" {
        match fs::read_to_string(input) {
            Ok(string) => {
                cmd_string = string;
            }
            Err(_) => panic!("Failed to read from the file!"),
        };
    }

    match translate_into_commands(&cmd_string) {
        Ok(commands) => {
            let mut state = ExecutionState::new(commands);
            loop {
                match state.execute_once() {
                    Ok(RunningState::Running) => {}
                    Ok(RunningState::Finished) => {
                        break;
                    }
                    Err(info) => panic!("{}", info),
                }
            }
        }
        Err(info) => panic!("{}", info),
    };
}
