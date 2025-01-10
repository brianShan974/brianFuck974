use std::collections::{HashMap, HashSet};

use super::{debugging_error::DebuggingError, debugging_state::DebuggingState};

use crate::executor::{
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
    pub fn new(commands: Vec<ExecutorCommand>) -> Self {
        Self {
            state: ExecutorState::new(commands),
            i_marks: HashMap::new(),
            i_marked_indices: HashMap::new(),
            c_marks: HashMap::new(),
            c_marked_indices: HashMap::new(),
            breakpoints: HashSet::new(),
            jump_history: Vec::new(),
            jump_cell_history: Vec::new(),
        }
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
        if (0..128).contains(&value) {
            println!("The value is a valid character <{}>.", value as u8 as char);
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

    fn list_marked_instruction(&self, name: String) -> DebuggingResult {
        self.long_list_marked_instruction(5, name)
    }

    fn long_list_marked_instruction(&self, length: usize, name: String) -> DebuggingResult {
        if let Some(index) = self.i_marks.get(&name) {
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

    fn list_marked_cell(&self, name: String) -> DebuggingResult {
        self.long_list_marked_cell(5, name)
    }

    fn long_list_marked_cell(&self, length: usize, name: String) -> DebuggingResult {
        if let Some(index) = self.c_marks.get(&name) {
            self.long_list_cell(length, Some(*index))
        } else {
            Err(DebuggingError::MarkNotFound)
        }
    }

    fn mark(&mut self, name: String, index: Option<usize>) -> DebuggingResult {
        let index = if let Some(i) = index {
            i
        } else {
            self.state.get_pc()
        };

        if self.state.validate_command_index(index) {
            if let Some(old_index) = self.i_marks.insert(name.clone(), index) {
                self.i_marked_indices.remove(&old_index);
            }
            self.i_marked_indices.insert(index, name);
            Ok(DebuggingState::Running)
        } else {
            Err(DebuggingError::IndexOutOfBounds)
        }
    }

    fn mark_cell(&mut self, name: String, index: Option<usize>) -> DebuggingResult {
        let index = if let Some(i) = index {
            i
        } else {
            self.state.get_pointer()
        };

        if self.state.validate_cell_index(index) {
            if let Some(old_index) = self.c_marks.insert(name.clone(), index) {
                self.c_marked_indices.remove(&old_index);
            }
            self.c_marked_indices.insert(index, name);
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

    fn jump_mark(&mut self, name: String) -> DebuggingResult {
        if let Some(index) = self.i_marks.get(&name) {
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

    fn jump_marked_cell(&mut self, name: String) -> DebuggingResult {
        if let Some(index) = self.c_marks.get(&name) {
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

    fn breakpoint_mark(&mut self, name: String) -> DebuggingResult {
        if let Some(index) = self.i_marks.get(&name) {
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

    fn remove_breakpoint_mark(&mut self, name: String) -> DebuggingResult {
        if let Some(index) = self.i_marks.get(&name) {
            self.remove_breakpoint(Some(*index))
        } else {
            Err(DebuggingError::MarkNotFound)
        }
    }

    fn step(&mut self) -> DebuggingResult {
        unimplemented!()
    }

    fn continue_to_breakpoint(&mut self) -> DebuggingResult {
        unimplemented!()
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
        println!();
    }

    fn p_cell(&self, index: usize, cell: &Int) {
        print!("Position: {}, Value: {}", index, cell);
        if let Some(mark) = self.c_marked_indices.get(&index) {
            println!(", Mark: {}", mark);
        }
    }
}
