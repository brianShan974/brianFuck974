use std::str::Split;

use crate::executor::executor_state::Int;

use super::parse_error::DebuggerCommandParseError;

pub enum DebuggerCommand {
    /// A no-op command that does not do anything.
    NoOp,

    /// Prints the instruction at an index. The parameter is the index. If not provided, the
    /// program counter will be used.
    PrintInstruction(Option<usize>),

    /// Prints the content of the cell at an index. The parameter is the index. If not provided,
    /// the data pointer will be used.
    PrintCell(Option<usize>),

    /// Prints all instructions in the program.
    PrintAllInstructions,

    /// Prints all cells in the program.
    PrintAllCells,

    /// Lists 11 instructions around an index. For example, if the index is 10, this command will
    /// list the instructions from 5 to 15. If the index is not provided, the program counter will
    /// be used.
    ListInstruction(Option<usize>),

    /// Lists n (specified by the first parameter) instructions before and after an index (specified
    /// by the second parameter). For example, if `n = 5`, this command has the same effect as
    /// ListInstruction(index). If the index is not provided, the program counter will be used.
    LongListInstruction(usize, Option<usize>),

    /// Lists 11 instructions around an instruction given its name. For example, if the index is
    /// 10, this command will list the instructions from 5 to 15. If the index is not provided, the
    /// program counter will be used.
    ListMarkedInstruction(String),

    /// Lists n (specified by the first parameter) instructions before and after a marked
    /// instruction (specified by the second parameter). For example, if `n = 5`, this command
    /// has the same effect as ListInstruction(index). If the index is not provided, the program
    /// counter will be used.
    LongListMarkedInstruction(usize, String),

    /// Lists 11 cells around an index. For example, if the index is 10, this command will list the
    /// cells from 5 to 15. If the index is not provided, the data pointer will be used.
    ListCell(Option<usize>),

    /// Lists n (specified by the first parameter) cells before and after an index (specified by the
    /// second parameter). For example, if `n = 5`, this command works the same as ListCell(index).
    /// If the index is not provided, the data pointer will be used.
    LongListCell(usize, Option<usize>),

    /// Lists 11 cells around an index. For example, if the index is 10, this command will list the
    /// cells from 5 to 15. If the index is not provided, the data pointer will be used.
    ListMarkedCell(String),

    /// Lists n (specified by the first parameter) cells before and after an index (specified by the
    /// second parameter). For example, if `n = 5`, this command works the same as ListCell(index).
    /// If the index is not provided, the data pointer will be used.
    LongListMarkedCell(usize, String),

    /// Sets the value of a cell given its index. If the index is not provided, the data pointer
    /// will be used.
    SetCell(Int, Option<usize>),

    /// Sets the value of a cell given its name.
    SetMarkedCell(Int, String), //implemented

    /// Runs one of the 6 instructions, excluding the square brackets.
    RunInstruction(char),

    /// Runs a sequence of the 6 instructions, excluding the square brackets.
    RunInstructions(String),

    /// Marks an instruction by its index. The first parameter is the name and the second parameter
    /// is the index. If the index is not provided, the program counter will be used.
    Mark(String, Option<usize>),

    /// Marks an cell by its index. The first parameter is the name and the second parameter is the
    /// index. If the index is not provided, the data pointer will be used.
    MarkCell(String, Option<usize>),

    /// Sets the program counter to an instruction given its index.
    Jump(usize),

    /// Sets the program counter to a marked instruction given its name.
    JumpMark(String),

    /// Sets the data pointer to a cell given its index.
    JumpCell(usize),

    /// Sets the data pointer to a marked cell given its name.
    JumpMarkedCell(String),

    /// Sets the program counter to the value before the last Jump(Mark) command.
    JumpBack,

    /// Sets the data pointer to the value before the last Jump(Marked)Cell command.
    JumpBackCell,

    /// Sets a breakpoint at an instruction given its index. If the index is not provided, the
    /// program counter will be used.
    Breakpoint(Option<usize>),

    /// Sets a breakpoint at a marked instruction given its name.
    BreakpointMark(String),

    /// Removes a breakpoint at an instruction given its index. If the index is not provided, the
    /// program counter will be used.
    RemoveBreakpoint(Option<usize>),

    /// Removes a breakpoint at a marked instruction given its name.
    RemoveBreakpointMark(String),

    /// Runs the next command.
    Step,

    /// Runs until the next breakpoint is reached.
    ContinueToBreakpoint,

    /// Quits the debugger.
    Quit,
}

impl TryFrom<String> for DebuggerCommand {
    type Error = DebuggerCommandParseError;

    fn try_from(input: String) -> Result<Self, Self::Error> {
        let input = input.trim().to_lowercase();
        let mut input = input.split(' ');

        if let Some(initial) = input.next() {
            match initial {
                "" => Ok(Self::NoOp),
                "pi" | "print_instruction" => {
                    let index = parse_optional_usize(&mut input)?;
                    Ok(Self::PrintInstruction(index))
                }
                "pc" | "print_cell" => {
                    let index = parse_optional_usize(&mut input)?;
                    Ok(Self::PrintCell(index))
                }
                "pai" | "print_all_instructions" => {
                    if input.next().is_some() {
                        Err(DebuggerCommandParseError::InvalidCommandFormat)
                    } else {
                        Ok(Self::PrintAllInstructions)
                    }
                }
                "pac" | "print_all_cells" => {
                    if input.next().is_some() {
                        Err(DebuggerCommandParseError::InvalidCommandFormat)
                    } else {
                        Ok(Self::PrintAllCells)
                    }
                }
                "li" | "list_instruction" => {
                    let index = parse_optional_usize(&mut input)?;
                    Ok(Self::ListInstruction(index))
                }
                "lli" | "long_list_instruction" => {
                    let length = parse_usize_value(&mut input, false)?;
                    let index = parse_optional_usize(&mut input)?;
                    Ok(Self::LongListInstruction(length, index))
                }
                "lmi" | "list_marked_instruction" => {
                    let mark = parse_string_value(&mut input, true)?;
                    Ok(Self::ListMarkedInstruction(mark))
                }
                "llmi" | "long_list_marked_instruction" => {
                    let length = parse_usize_value(&mut input, false)?;
                    let mark = parse_string_value(&mut input, true)?;
                    Ok(Self::LongListMarkedInstruction(length, mark))
                }
                "lc" | "list_cell" => {
                    let index = parse_optional_usize(&mut input)?;
                    Ok(Self::ListCell(index))
                }
                "llc" | "long_list_cell" => {
                    let length = parse_usize_value(&mut input, false)?;
                    let index = parse_optional_usize(&mut input)?;
                    Ok(Self::LongListCell(length, index))
                }
                "lmc" | "list_marked_cell" => {
                    let mark = parse_string_value(&mut input, true)?;
                    Ok(Self::ListMarkedCell(mark))
                }
                "llmc" | "long_list_marked_cell" => {
                    let length = parse_usize_value(&mut input, false)?;
                    let mark = parse_string_value(&mut input, true)?;
                    Ok(Self::LongListMarkedCell(length, mark))
                }
                "sc" | "set_cell" => {
                    let value = parse_int_value(&mut input)?;
                    let index = parse_optional_usize(&mut input)?;
                    Ok(Self::SetCell(value, index))
                }
                "smc" | "set_marked_cell" => {
                    let value = parse_int_value(&mut input)?;
                    let mark = parse_string_value(&mut input, true)?;
                    Ok(Self::SetMarkedCell(value, mark))
                }
                "ri" | "run_instruction" => {
                    let instruction = parse_char_value(&mut input)?;
                    Ok(Self::RunInstruction(instruction))
                }
                "ris" | "run_instructions" => {
                    let instructions = parse_string_value(&mut input, true)?;
                    Ok(Self::RunInstructions(instructions))
                }
                "m" | "mark" => {
                    let mark = parse_string_value(&mut input, false)?;
                    let index = parse_optional_usize(&mut input)?;
                    Ok(Self::Mark(mark, index))
                }
                "mc" | "mark_cell" => {
                    let mark = parse_string_value(&mut input, false)?;
                    let index = parse_optional_usize(&mut input)?;
                    Ok(Self::MarkCell(mark, index))
                }
                "j" | "jump" => {
                    let index = parse_usize_value(&mut input, true)?;
                    Ok(Self::Jump(index))
                }
                "jm" | "jump_mark" => {
                    let mark = parse_string_value(&mut input, true)?;
                    Ok(Self::JumpMark(mark))
                }
                "jc" | "jump_cell" => {
                    let index = parse_usize_value(&mut input, true)?;
                    Ok(Self::JumpCell(index))
                }
                "jmc" | "jump_marked_cell" => {
                    let mark = parse_string_value(&mut input, true)?;
                    Ok(Self::JumpMarkedCell(mark))
                }
                "jb" | "jump_back" => {
                    if input.next().is_some() {
                        Err(DebuggerCommandParseError::InvalidCommandFormat)
                    } else {
                        Ok(Self::JumpBack)
                    }
                }
                "jbc" | "jump_back_cell" => {
                    if input.next().is_some() {
                        Err(DebuggerCommandParseError::InvalidCommandFormat)
                    } else {
                        Ok(Self::JumpBackCell)
                    }
                }
                "b" | "breakpoint" => {
                    let index = parse_optional_usize(&mut input)?;
                    Ok(Self::Breakpoint(index))
                }
                "bm" | "breakpoint_mark" => {
                    let mark = parse_string_value(&mut input, true)?;
                    Ok(Self::BreakpointMark(mark))
                }
                "rb" | "remove_breakpoint" => {
                    let index = parse_optional_usize(&mut input)?;
                    Ok(Self::RemoveBreakpoint(index))
                }
                "rbm" | "remove_breakpoint_mark" => {
                    let mark = parse_string_value(&mut input, true)?;
                    Ok(Self::RemoveBreakpointMark(mark))
                }
                "s" | "step" => {
                    if input.next().is_some() {
                        Err(DebuggerCommandParseError::InvalidCommandFormat)
                    } else {
                        Ok(Self::Step)
                    }
                }
                "ctb" | "continue_to_breakpoint" => {
                    if input.next().is_some() {
                        Err(DebuggerCommandParseError::InvalidCommandFormat)
                    } else {
                        Ok(Self::ContinueToBreakpoint)
                    }
                }
                "q" | "quit" => {
                    if input.next().is_some() {
                        Err(DebuggerCommandParseError::InvalidCommandFormat)
                    } else {
                        Ok(Self::Quit)
                    }
                }
                _ => Err(Self::Error::InvalidCommandFormat),
            }
        } else {
            Ok(Self::NoOp)
        }
    }
}

fn parse_int_value(input: &mut Split<'_, char>) -> Result<Int, DebuggerCommandParseError> {
    if let Some(value) = input.next() {
        if let Ok(value) = value.parse() {
            Ok(value)
        } else {
            Err(DebuggerCommandParseError::InvalidParameter)
        }
    } else {
        Err(DebuggerCommandParseError::InvalidCommandFormat)
    }
}

fn parse_usize_value(
    input: &mut Split<'_, char>,
    last: bool,
) -> Result<usize, DebuggerCommandParseError> {
    if let Some(value) = input.next() {
        if last && input.next().is_some() {
            return Err(DebuggerCommandParseError::InvalidCommandFormat);
        }

        if let Ok(value) = value.parse() {
            Ok(value)
        } else {
            Err(DebuggerCommandParseError::InvalidParameter)
        }
    } else {
        Err(DebuggerCommandParseError::InvalidCommandFormat)
    }
}

fn parse_string_value(
    input: &mut Split<'_, char>,
    last: bool,
) -> Result<String, DebuggerCommandParseError> {
    let mark = input.next();
    if let Some(mark) = mark {
        if last && input.next().is_some() {
            return Err(DebuggerCommandParseError::InvalidCommandFormat);
        }

        Ok(mark.to_string())
    } else {
        Err(DebuggerCommandParseError::InvalidCommandFormat)
    }
}

fn parse_char_value(input: &mut Split<'_, char>) -> Result<char, DebuggerCommandParseError> {
    let mark = input.next();
    if let Some(mark) = mark {
        if input.next().is_some() {
            return Err(DebuggerCommandParseError::InvalidCommandFormat);
        }

        if mark.len() != 1 {
            return Err(DebuggerCommandParseError::InvalidParameter);
        }

        Ok(mark.chars().next().unwrap())
    } else {
        Err(DebuggerCommandParseError::InvalidCommandFormat)
    }
}

fn parse_optional_usize(
    input: &mut Split<'_, char>,
) -> Result<Option<usize>, DebuggerCommandParseError> {
    let index = input.next();
    if let Some(index) = index {
        if input.next().is_some() {
            return Err(DebuggerCommandParseError::InvalidCommandFormat);
        }

        if let Ok(index) = index.parse() {
            Ok(Some(index))
        } else {
            Err(DebuggerCommandParseError::InvalidParameter)
        }
    } else {
        Ok(None)
    }
}
