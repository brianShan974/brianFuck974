use std::io::{self, Read};

use crate::executor::{executing_state::ExecutionState, executor_command::ExecutorCommand};

use super::executing_error::ExecutionError;

pub type Int = i128;

pub type ExecutionResult = Result<ExecutionState, ExecutionError>;

const INITIAL_SIZE: usize = 32;

pub struct ExecutorState {
    pointer: usize,
    array: Vec<Int>,
    pc: usize,
    commands: Vec<ExecutorCommand>,
}

impl ExecutorState {
    pub fn new(commands: Vec<ExecutorCommand>) -> ExecutorState {
        let mut array = Vec::with_capacity(INITIAL_SIZE);
        array.push(0);

        ExecutorState {
            pointer: 0,
            array,
            pc: 0, // program counter
            commands,
        }
    }

    pub fn execute_once(&mut self) -> ExecutionResult {
        if self.pc >= self.commands.len() {
            return Ok(ExecutionState::Finished);
        }

        let current_cmd = self.commands[self.pc].clone();

        self.execute_command(current_cmd)?;
        self.increment_pc();

        Ok(ExecutionState::Running)
    }

    pub fn execute_command(&mut self, command: ExecutorCommand) -> Result<usize, ExecutionError> {
        self.check_state_valid()?;

        match command {
            ExecutorCommand::MoveRight => self.move_to_the_right()?,
            ExecutorCommand::MoveLeft => self.move_to_the_left()?,
            ExecutorCommand::Increment => self.increment()?,
            ExecutorCommand::Decrement => self.decrement()?,
            ExecutorCommand::Input => self.input()?,
            ExecutorCommand::Output => self.output()?,
            ExecutorCommand::JumpForward(pos) => self.jump_forward(pos)?,
            ExecutorCommand::JumpBack(pos) => self.jump_back(pos)?,
        };

        Ok(self.pc)
    }

    pub fn increment_pc(&mut self) {
        self.pc += 1;
    }

    pub fn get_cells(&self) -> &Vec<Int> {
        &self.array
    }

    pub fn get_commands(&self) -> &Vec<ExecutorCommand> {
        &self.commands
    }

    pub fn get_pc(&self) -> usize {
        self.pc
    }

    pub fn get_pointer(&self) -> usize {
        self.pointer
    }

    pub fn get_cell(&self, index: usize) -> Option<Int> {
        self.array.get(index).copied()
    }

    pub fn get_command(&self, index: usize) -> Option<ExecutorCommand> {
        self.commands.get(index).cloned()
    }

    pub fn get_array_len(&self) -> usize {
        self.array.len()
    }

    pub fn get_commands_len(&self) -> usize {
        self.commands.len()
    }

    pub fn set_pc(&mut self, index: usize) -> bool {
        if self.validate_command_index(index) {
            self.pc = index;
            true
        } else {
            false
        }
    }

    pub fn set_pointer(&mut self, index: usize) -> bool {
        if self.validate_cell_index(index) {
            self.pointer = index;
            true
        } else {
            false
        }
    }

    pub fn set_cell_value(&mut self, index: usize, value: Int) -> bool {
        if self.validate_cell_index(index) {
            self.array[index] = value;
            true
        } else {
            false
        }
    }

    pub fn validate_command_index(&self, index: usize) -> bool {
        (0..self.commands.len()).contains(&index)
    }

    pub fn validate_cell_index(&self, index: usize) -> bool {
        (0..self.array.len()).contains(&index)
    }

    fn check_state_valid(&self) -> Result<(), ExecutionError> {
        if self.pc >= self.commands.len() {
            Err(ExecutionError::ProgramCounterOutOfRange)
        } else if self.pointer >= self.array.len() {
            Err(ExecutionError::TooFarRight)
        } else {
            Ok(())
        }
    }

    fn move_to_the_right(&mut self) -> ExecutionResult {
        self.pointer += 1;

        if self.pointer >= self.array.len() {
            self.array.push(0);
        }

        Ok(ExecutionState::Running)
    }

    fn move_to_the_left(&mut self) -> ExecutionResult {
        if self.pointer == 0 {
            Err(ExecutionError::TooFarLeft)
        } else {
            self.pointer -= 1;
            Ok(ExecutionState::Running)
        }
    }

    fn increment(&mut self) -> ExecutionResult {
        if self.array[self.pointer] == Int::MAX {
            Err(ExecutionError::Overflow)
        } else {
            self.array[self.pointer] += 1;
            Ok(ExecutionState::Running)
        }
    }

    fn decrement(&mut self) -> ExecutionResult {
        if self.array[self.pointer] == Int::MIN {
            Err(ExecutionError::Underflow)
        } else {
            self.array[self.pointer] -= 1;
            Ok(ExecutionState::Running)
        }
    }

    fn output(&mut self) -> ExecutionResult {
        let data = self.array[self.pointer];

        if let Some(converted_char) = char::from_u32(data as u32) {
            print!("{}", converted_char);
        } else {
            return Err(ExecutionError::InvalidCharacter);
        }

        Ok(ExecutionState::Running)
    }

    fn input(&mut self) -> ExecutionResult {
        let input_char = io::stdin()
            .bytes()
            .next()
            .ok_or(ExecutionError::InputError)?
            .map_err(|_err| ExecutionError::InputError)?;

        self.array[self.pointer] = input_char as Int;

        Ok(ExecutionState::Running)
    }

    fn jump_forward(&mut self, pos: usize) -> ExecutionResult {
        if pos > self.commands.len() {
            Err(ExecutionError::TooFarRight)
        } else {
            if self.array[self.pointer] == 0 {
                self.pc = pos;
            }
            Ok(ExecutionState::Running)
        }
    }

    fn jump_back(&mut self, pos: usize) -> ExecutionResult {
        if self.array[self.pointer] != 0 {
            self.pc = pos;
        }

        Ok(ExecutionState::Running)
    }
}
