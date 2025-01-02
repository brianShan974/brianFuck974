use std::io;
use std::io::Read;

use crate::Command;
use crate::RunningState;

type Int = i128;
const INITIAL_SIZE: usize = 32;

pub struct ExecutionState {
    pointer: usize,
    array: Vec<i128>,
    pc: usize,
    commands: Vec<Command>,
}

impl ExecutionState {
    pub fn new(commands: Vec<Command>) -> ExecutionState {
        let mut array = Vec::with_capacity(INITIAL_SIZE);
        array.push(0);

        ExecutionState {
            pointer: 0,
            array,
            pc: 0, // program counter
            commands,
        }
    }

    pub fn execute_once(&mut self) -> Result<RunningState, String> {
        if self.pc >= self.commands.len() {
            return Ok(RunningState::Finished);
        }
        let current_cmd = &self.commands[self.pc];
        match *current_cmd {
            Command::MoveRight => self.move_to_the_right()?,
            Command::MoveLeft => self.move_to_the_left()?,
            Command::Increment => self.increment()?,
            Command::Decrement => self.decrement()?,
            Command::Input => self.input()?,
            Command::Output => self.output()?,
            Command::JumpForward(pos) => self.jump_forward(pos)?,
            Command::JumpBack(pos) => self.jump_back(pos)?,
        };
        self.pc += 1;
        Ok(RunningState::Running)
    }

    fn move_to_the_right(&mut self) -> Result<RunningState, String> {
        self.pointer += 1;

        if self.pointer >= self.array.len() {
            self.array.push(0);
        }

        Ok(RunningState::Running)
    }

    fn move_to_the_left(&mut self) -> Result<RunningState, String> {
        if self.pointer == 0 {
            Err(String::from(
                "Index Error: You have gone too far to the left!",
            ))
        } else {
            self.pointer -= 1;
            Ok(RunningState::Running)
        }
    }

    fn increment(&mut self) -> Result<RunningState, String> {
        if self.array[self.pointer] + 1 < self.array[self.pointer] {
            Err(String::from(
                "Overflow Error: The number in the cell is too large!",
            ))
        } else {
            self.array[self.pointer] += 1;
            Ok(RunningState::Running)
        }
    }

    fn decrement(&mut self) -> Result<RunningState, String> {
        if self.array[self.pointer] - 1 > self.array[self.pointer] {
            Err(String::from(
                "Overflow Error: The number in the cell is too small!",
            ))
        } else {
            self.array[self.pointer] -= 1;
            Ok(RunningState::Running)
        }
    }

    fn output(&mut self) -> Result<RunningState, String> {
        let data = self.array[self.pointer];

        if let Some(converted_char) = char::from_u32(data as u32) {
            print!("{}", converted_char);
        } else {
            return Err(String::from(
                "Value Error: The value in the cell is not a valid Unicode character!",
            ));
        }

        Ok(RunningState::Running)
    }

    fn input(&mut self) -> Result<RunningState, String> {
        let input_char = io::stdin()
            .bytes()
            .next()
            .ok_or("IO Error: Unable to get character input!")?
            .map_err(|_err| "IO Error: Unable to get character input!")?;

        self.array[self.pointer] = input_char as Int;

        Ok(RunningState::Running)
    }

    fn jump_forward(&mut self, pos: usize) -> Result<RunningState, String> {
        if pos > self.commands.len() {
            Err(String::from(
                "Runtime Error: You have gone too far to the right!",
            ))
        } else {
            if self.array[self.pointer] == 0 {
                self.pc = pos;
            }
            Ok(RunningState::Running)
        }
    }

    fn jump_back(&mut self, pos: usize) -> Result<RunningState, String> {
        if self.array[self.pointer] != 0 {
            self.pc = pos;
        }

        Ok(RunningState::Running)
    }
}
