mod command;
mod command_line_args;
mod execution_state;
mod parsing_src;
mod running_state;

use clap::Parser;
use command::Command;
use command_line_args::Args;
use execution_state::ExecutionState;
use parsing_src::translate_into_commands;
use running_state::RunningState;

fn main() {
    let args = Args::parse();

    let cmd_string = match std::fs::read_to_string(args.path) {
        Ok(string) => string,
        Err(_) => panic!("Failed to read from the file!"),
    };

    match translate_into_commands(&cmd_string) {
        Ok(commands) => {
            let mut state = ExecutionState::new(commands);
            loop {
                match state.execute_once() {
                    Ok(RunningState::Running) => {}
                    Ok(RunningState::Finished) => {
                        break;
                    }
                    Err(info) => {
                        println!("{}", info);
                        break;
                    }
                }
            }
        }
        Err(info) => println!("{}", info),
    };
}
