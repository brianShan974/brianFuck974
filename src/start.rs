use crate::{
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
            let mut state = ExecutorState::new(commands, breakpoints);
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
