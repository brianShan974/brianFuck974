use crate::executor::executor_state::Int;

enum DebuggerCommand {
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
