use std::{
    char,
    collections::{HashMap, HashSet},
    io::{self, Write},
};

use super::{
    debugger_command::DebuggerCommand, debugging_error::DebuggingError,
    debugging_state::DebuggingState, parse_error::DebuggerCommandParseError,
};

use crate::executor::{
    executing_state::ExecutionState,
    executor_command::ExecutorCommand,
    executor_state::{ExecutorState, Int},
};

type DebuggingResult = Result<DebuggingState, DebuggingError>;

pub struct DebuggerState {
    state: ExecutorState,
    i_marks: HashMap<String, usize>,
    i_marked_indices: HashMap<usize, String>,
    c_marks: HashMap<String, usize>,
    c_marked_indices: HashMap<usize, String>,
    breakpoints: HashSet<usize>,
    jump_history: Vec<usize>,
    jump_cell_history: Vec<usize>,
}

impl DebuggerState {
    pub fn new(commands: Vec<ExecutorCommand>, breakpoints: HashSet<usize>) -> Self {
        Self {
            state: ExecutorState::new(commands),
            i_marks: HashMap::new(),
            i_marked_indices: HashMap::new(),
            c_marks: HashMap::new(),
            c_marked_indices: HashMap::new(),
            breakpoints,
            jump_history: Vec::new(),
            jump_cell_history: Vec::new(),
        }
    }

    pub fn execute_debugger_command(
        &mut self,
        command: String,
    ) -> Result<DebuggingResult, DebuggerCommandParseError> {
        let command = DebuggerCommand::try_from(command)?;

        use DebuggerCommand as DC;
        let result = match command {
            DC::NoOp => Ok(DebuggingState::Running),
            DC::PrintInstruction(index) => self.print_instruction(index),
            DC::PrintCell(index) => self.print_cell(index),
            DC::PrintAllInstructions => self.print_all_instructions(),
            DC::PrintAllCells => self.print_all_cells(),
            DC::ListInstruction(index) => self.list_instruction(index),
            DC::LongListInstruction(length, index) => self.long_list_instruction(length, index),
            DC::ListMarkedInstruction(mark) => self.list_marked_instruction(mark),
            DC::LongListMarkedInstruction(length, mark) => {
                self.long_list_marked_instruction(length, mark)
            }
            DC::ListCell(index) => self.list_cell(index),
            DC::LongListCell(length, index) => self.long_list_cell(length, index),
            DC::ListMarkedCell(mark) => self.list_marked_cell(mark),
            DC::LongListMarkedCell(length, mark) => self.long_list_marked_cell(length, mark),
            DC::SetCell(value, index) => self.set_cell(value, index),
            DC::SetMarkedCell(value, mark) => self.set_marked_cell(value, mark),
            DC::RunInstruction(instruction) => self.run_instruction(instruction),
            DC::RunInstructions(instructions) => self.run_instructions(instructions),
            DC::Mark(mark, index) => self.mark(mark, index),
            DC::MarkCell(mark, index) => self.mark_cell(mark, index),
            DC::Jump(index) => self.jump(index),
            DC::JumpMark(mark) => self.jump_mark(mark),
            DC::JumpCell(index) => self.jump_cell(index),
            DC::JumpMarkedCell(mark) => self.jump_marked_cell(mark),
            DC::JumpBack => self.jump_back(),
            DC::JumpBackCell => self.jump_back_cell(),
            DC::Breakpoint(index) => self.breakpoint(index),
            DC::BreakpointMark(mark) => self.breakpoint_mark(mark),
            DC::RemoveBreakpoint(index) => self.remove_breakpoint(index),
            DC::RemoveBreakpointMark(mark) => self.remove_breakpoint_mark(mark),
            DC::Step => self.step(),
            DC::ContinueToBreakpoint => self.continue_to_breakpoint(),
            DC::Quit => self.quit(),
        };

        Ok(result)
    }

    fn print_instruction(&self, index: Option<usize>) -> DebuggingResult {
        let index = if let Some(i) = index {
            i
        } else {
            self.state.get_pc()
        };

        let command = self
            .state
            .get_command(index)
            .ok_or(DebuggingError::IndexOutOfBounds)?;

        println!("The command at index {} is {}.", index, command);

        if let Some(mark) = self.i_marked_indices.get(&index) {
            println!("It is marked as <{}>.", mark);
        }

        if self.breakpoints.contains(&index) {
            println!("It is a breakpoint.");
        }

        Ok(DebuggingState::Running)
    }

    fn print_cell(&self, index: Option<usize>) -> DebuggingResult {
        let index = if let Some(i) = index {
            i
        } else {
            self.state.get_pointer()
        };

        let value = self
            .state
            .get_cell(index)
            .ok_or(DebuggingError::IndexOutOfBounds)?;

        println!("The cell at index {} has the value {}.", index, value);
        if let Some(c) = char::from_u32(value as u32) {
            println!("The value is a valid character <{}>.", c);
        }

        if let Some(mark) = self.c_marked_indices.get(&index) {
            println!("The cell is marked as <{}>.", mark);
        }

        Ok(DebuggingState::Running)
    }

    fn print_all_instructions(&self) -> DebuggingResult {
        let instructions = self.state.get_commands();

        for (index, command) in instructions.iter().enumerate() {
            self.p_instruction(index, command);
        }

        Ok(DebuggingState::Running)
    }

    fn print_all_cells(&self) -> DebuggingResult {
        let cells = self.state.get_cells();

        for (index, cell) in cells.iter().enumerate() {
            self.p_cell(index, cell);
        }

        Ok(DebuggingState::Running)
    }

    fn list_instruction(&self, index: Option<usize>) -> DebuggingResult {
        self.long_list_instruction(5, index)
    }

    fn long_list_instruction(&self, length: usize, index: Option<usize>) -> DebuggingResult {
        let index = if let Some(i) = index {
            i
        } else {
            self.state.get_pc()
        };

        if !self.state.validate_command_index(index) {
            return Err(DebuggingError::IndexOutOfBounds);
        }

        let commands_len = self.state.get_commands_len();

        let lower_bound = index.saturating_sub(length);
        let upper_bound = if index + length >= commands_len {
            commands_len - 1
        } else {
            index + length
        };

        for i in lower_bound..=upper_bound {
            self.p_instruction(i, &self.state.get_command(i).unwrap());
        }

        Ok(DebuggingState::Running)
    }

    fn list_marked_instruction(&self, mark: String) -> DebuggingResult {
        self.long_list_marked_instruction(5, mark)
    }

    fn long_list_marked_instruction(&self, length: usize, mark: String) -> DebuggingResult {
        if let Some(index) = self.i_marks.get(&mark) {
            self.long_list_instruction(length, Some(*index))
        } else {
            Err(DebuggingError::MarkNotFound)
        }
    }

    fn list_cell(&self, index: Option<usize>) -> DebuggingResult {
        self.long_list_cell(5, index)
    }

    fn long_list_cell(&self, length: usize, index: Option<usize>) -> DebuggingResult {
        let index = if let Some(i) = index {
            i
        } else {
            self.state.get_pointer()
        };

        if !self.state.validate_cell_index(index) {
            return Err(DebuggingError::IndexOutOfBounds);
        }

        let array_len = self.state.get_array_len();

        let lower_bound = index.saturating_sub(length);
        let upper_bound = if index + length >= array_len {
            array_len - 1
        } else {
            index + length
        };

        for i in lower_bound..=upper_bound {
            self.p_cell(i, &self.state.get_cell(i).unwrap());
        }

        Ok(DebuggingState::Running)
    }

    fn list_marked_cell(&self, mark: String) -> DebuggingResult {
        self.long_list_marked_cell(5, mark)
    }

    fn long_list_marked_cell(&self, length: usize, mark: String) -> DebuggingResult {
        if let Some(index) = self.c_marks.get(&mark) {
            self.long_list_cell(length, Some(*index))
        } else {
            Err(DebuggingError::MarkNotFound)
        }
    }

    fn set_cell(&mut self, value: Int, index: Option<usize>) -> DebuggingResult {
        let index = if let Some(i) = index {
            i
        } else {
            self.state.get_pointer()
        };

        if self.state.set_cell_value(index, value) {
            Ok(DebuggingState::Running)
        } else {
            Err(DebuggingError::IndexOutOfBounds)
        }
    }

    fn set_marked_cell(&mut self, value: Int, mark: String) -> DebuggingResult {
        if let Some(index) = self.c_marks.get(&mark) {
            self.set_cell(value, Some(*index))
        } else {
            Err(DebuggingError::MarkNotFound)
        }
    }

    fn run_instruction(&mut self, instruction: char) -> DebuggingResult {
        let command = match instruction {
            '+' => ExecutorCommand::Increment,
            '-' => ExecutorCommand::Decrement,
            '>' => ExecutorCommand::MoveRight,
            '<' => ExecutorCommand::MoveLeft,
            '.' => ExecutorCommand::Output,
            ',' => ExecutorCommand::Input,
            _ => return Err(DebuggingError::InvalidInstruction),
        };

        match self.state.execute_command(command) {
            Ok(_) => Ok(DebuggingState::Running),
            Err(err) => {
                println!("{}", err);
                Ok(DebuggingState::Finished)
            }
        }
    }

    fn run_instructions(&mut self, instructions: String) -> DebuggingResult {
        for instruction in instructions.chars() {
            self.run_instruction(instruction)?;
        }

        Ok(DebuggingState::Running)
    }

    fn mark(&mut self, mark: String, index: Option<usize>) -> DebuggingResult {
        let index = if let Some(i) = index {
            i
        } else {
            self.state.get_pc()
        };

        if self.state.validate_command_index(index) {
            if let Some(old_index) = self.i_marks.insert(mark.clone(), index) {
                self.i_marked_indices.remove(&old_index);
            }
            self.i_marked_indices.insert(index, mark);
            Ok(DebuggingState::Running)
        } else {
            Err(DebuggingError::IndexOutOfBounds)
        }
    }

    fn mark_cell(&mut self, mark: String, index: Option<usize>) -> DebuggingResult {
        let index = if let Some(i) = index {
            i
        } else {
            self.state.get_pointer()
        };

        if self.state.validate_cell_index(index) {
            if let Some(old_index) = self.c_marks.insert(mark.clone(), index) {
                self.c_marked_indices.remove(&old_index);
            }
            self.c_marked_indices.insert(index, mark);
            Ok(DebuggingState::Running)
        } else {
            Err(DebuggingError::IndexOutOfBounds)
        }
    }

    fn jump(&mut self, index: usize) -> DebuggingResult {
        let current_pc = self.state.get_pc();

        if self.state.set_pc(index) {
            self.jump_history.push(current_pc);
            Ok(DebuggingState::Running)
        } else {
            Err(DebuggingError::IndexOutOfBounds)
        }
    }

    fn jump_mark(&mut self, mark: String) -> DebuggingResult {
        if let Some(index) = self.i_marks.get(&mark) {
            self.jump(*index)
        } else {
            Err(DebuggingError::MarkNotFound)
        }
    }

    fn jump_cell(&mut self, index: usize) -> DebuggingResult {
        let current_pointer = self.state.get_pointer();

        if self.state.set_pointer(index) {
            self.jump_cell_history.push(current_pointer);
            Ok(DebuggingState::Running)
        } else {
            Err(DebuggingError::IndexOutOfBounds)
        }
    }

    fn jump_marked_cell(&mut self, mark: String) -> DebuggingResult {
        if let Some(index) = self.c_marks.get(&mark) {
            self.jump_cell(*index)
        } else {
            Err(DebuggingError::MarkNotFound)
        }
    }

    fn jump_back(&mut self) -> DebuggingResult {
        if self.jump_history.is_empty() {
            Err(DebuggingError::JumpHistoryEmpty)
        } else {
            self.state.set_pc(self.jump_history.pop().unwrap());
            Ok(DebuggingState::Running)
        }
    }

    fn jump_back_cell(&mut self) -> DebuggingResult {
        if self.jump_cell_history.is_empty() {
            Err(DebuggingError::JumpHistoryEmpty)
        } else {
            self.state
                .set_pointer(self.jump_cell_history.pop().unwrap());
            Ok(DebuggingState::Running)
        }
    }

    fn breakpoint(&mut self, index: Option<usize>) -> DebuggingResult {
        let index = if let Some(i) = index {
            i
        } else {
            self.state.get_pc()
        };

        if self.state.validate_command_index(index) {
            self.breakpoints.insert(index);
            Ok(DebuggingState::Running)
        } else {
            Err(DebuggingError::IndexOutOfBounds)
        }
    }

    fn breakpoint_mark(&mut self, mark: String) -> DebuggingResult {
        if let Some(index) = self.i_marks.get(&mark) {
            self.breakpoint(Some(*index))
        } else {
            Err(DebuggingError::MarkNotFound)
        }
    }

    fn remove_breakpoint(&mut self, index: Option<usize>) -> DebuggingResult {
        let index = if let Some(i) = index {
            i
        } else {
            self.state.get_pc()
        };

        if self.breakpoints.contains(&index) {
            self.breakpoints.remove(&index);
            Ok(DebuggingState::Running)
        } else {
            Err(DebuggingError::BreakpointNotFound)
        }
    }

    fn remove_breakpoint_mark(&mut self, mark: String) -> DebuggingResult {
        if let Some(index) = self.i_marks.get(&mark) {
            self.remove_breakpoint(Some(*index))
        } else {
            Err(DebuggingError::MarkNotFound)
        }
    }

    fn step(&mut self) -> DebuggingResult {
        let result = match self.state.execute_once() {
            Ok(ExecutionState::Running) => Ok(DebuggingState::Running),
            Ok(ExecutionState::Finished) => Ok(DebuggingState::Finished),
            Err(err) => {
                println!("{}", err);
                Ok(DebuggingState::Finished)
            }
        };

        if self.print_instruction(None).is_err() {
            return result;
        }

        result
    }

    fn continue_to_breakpoint(&mut self) -> DebuggingResult {
        loop {
            match self.step() {
                Ok(DebuggingState::Finished) => return Ok(DebuggingState::Finished),
                Err(err) => {
                    println!("{}", err);
                    return Ok(DebuggingState::Finished);
                }
                _ => {}
            }

            if self.breakpoints.contains(&self.state.get_pc()) {
                break Ok(DebuggingState::Paused);
            }
        }
    }

    fn quit(&self) -> DebuggingResult {
        Ok(DebuggingState::Finished)
    }

    fn p_instruction(&self, index: usize, command: &ExecutorCommand) {
        print!("Position: {}, Value: {}", index, command);
        if let Some(mark) = self.i_marked_indices.get(&index) {
            print!(", Mark: {}", mark);
        }
        if self.breakpoints.contains(&index) {
            print!(", Breakpoint: true");
        }
        io::stdout().flush().unwrap();
        println!();
    }

    fn p_cell(&self, index: usize, cell: &Int) {
        print!("Position: {}, Value: {}", index, cell);
        if let Some(mark) = self.c_marked_indices.get(&index) {
            print!(", Mark: {}", mark);
        }
        io::stdout().flush().unwrap();
        println!();
    }
}
