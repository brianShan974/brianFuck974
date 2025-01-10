use std::io::{self, Write};

use crate::{
    debugger::{
        debugger_state::DebuggerState, debugging_state::DebuggingState,
        parse_error::DebuggerCommandParseError,
    },
    executor::{executing_state::ExecutionState, executor_state::ExecutorState},
    parsing_src::translate_into_commands,
};

pub fn execute(cmd: &str) {
    match translate_into_commands(cmd, false) {
        Ok((commands, _)) => {
            let mut state = ExecutorState::new(commands);
            loop {
                match state.execute_once() {
                    Ok(ExecutionState::Running) => {}
                    Ok(ExecutionState::Finished) => {
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

pub fn debug(cmd: &str) {
    match translate_into_commands(cmd, true) {
        Ok((commands, breakpoints)) => {
            let mut debugger = DebuggerState::new(commands, breakpoints);
            loop {
                print!("Please enter the next command: ");
                io::stdout().flush().unwrap();
                let mut input_line = String::new();
                if io::stdin().read_line(&mut input_line).is_err() {
                    println!("IO Error: Failed to get command input!");
                    break;
                }

                match debugger.execute_debugger_command(input_line) {
                    Ok(result) => match result {
                        Ok(DebuggingState::Finished) => {
                            println!("Quitting the debugger. Bye!");
                            break;
                        }
                        Err(err) => println!("{}", err),
                        _ => {}
                    },
                    Err(DebuggerCommandParseError::InvalidParameter) => {
                        println!("Invalid parameter!")
                    }
                    Err(DebuggerCommandParseError::InvalidCommandFormat) => {
                        println!("Invalid command format!")
                    }
                }
                println!();
            }
        }
        Err(info) => println!("{}", info),
    };
}
