enum DebuggerCommand {
    /// Prints the instruction at an index. The parameter is the index. If not provided, the
    /// program counter will be used.
    PrintInstruction(Option<usize>), // implemented

    /// Prints the content of the cell at an index. The parameter is the index. If not provided,
    /// the data pointer will be used.
    PrintCell(Option<usize>), // implemented

    /// Prints all instructions in the program.
    PrintAllInstructions, // implemented

    /// Prints all cells in the program.
    PrintAllCells, // implemented

    /// Lists 11 instructions around an index. For example, if the index is 10, this command will
    /// list the instructions from 5 to 15. If the index is not provided, the program counter will
    /// be used.
    ListInstruction(Option<usize>), // implemented

    /// Lists n (specified by the first parameter) instructions before and after an index (specified
    /// by the second parameter). For example, if `n = 5`, this command has the same effect as
    /// ListInstruction(index). If the index is not provided, the program counter will be used.
    LongListInstruction(usize, Option<usize>), // implemented

    /// Lists 11 instructions around an instruction given its name. For example, if the index is
    /// 10, this command will list the instructions from 5 to 15. If the index is not provided, the
    /// program counter will be used.
    ListMarkedInstruction(String), // implemented

    /// Lists n (specified by the first parameter) instructions before and after a marked
    /// instruction (specified by the second parameter). For example, if `n = 5`, this command
    /// has the same effect as ListInstruction(index). If the index is not provided, the program
    /// counter will be used.
    LongListMarkedInstruction(usize, String), // implemented

    /// Lists 11 cells around an index. For example, if the index is 10, this command will list the
    /// cells from 5 to 15. If the index is not provided, the data pointer will be used.
    ListCell(Option<usize>), // implemented

    /// Lists n (specified by the first parameter) cells before and after an index (specified by the
    /// second parameter). For example, if `n = 5`, this command works the same as ListCell(index).
    /// If the index is not provided, the data pointer will be used.
    LongListCell(usize, Option<usize>), // implemented

    /// Lists 11 cells around an index. For example, if the index is 10, this command will list the
    /// cells from 5 to 15. If the index is not provided, the data pointer will be used.
    ListMarkedCell(String), // implemented

    /// Lists n (specified by the first parameter) cells before and after an index (specified by the
    /// second parameter). For example, if `n = 5`, this command works the same as ListCell(index).
    /// If the index is not provided, the data pointer will be used.
    LongListMarkedCell(usize, String), // implemented

    /// Marks an instruction by its index. The first parameter is the name and the second parameter
    /// is the index. If the index is not provided, the program counter will be used.
    Mark(String, Option<usize>), // implemented

    /// Marks an cell by its index. The first parameter is the name and the second parameter is the
    /// index. If the index is not provided, the data pointer will be used.
    MarkCell(String, Option<usize>), // implemented

    /// Sets the program counter to an instruction given its index.
    Jump(usize), // implemented

    /// Sets the program counter to a marked instruction given its name.
    JumpMark(String), // implemented

    /// Sets the data pointer to a cell given its index.
    JumpCell(usize), // implemented

    /// Sets the data pointer to a marked cell given its name.
    JumpMarkedCell(String), // implemented

    /// Sets the program counter to the value before the last Jump(Mark) command.
    JumpBack, // implemented

    /// Sets the data pointer to the value before the last Jump(Marked)Cell command.
    JumpBackCell, // implemented

    /// Sets a breakpoint at an instruction given its index. If the index is not provided, the
    /// program counter will be used.
    Breakpoint(Option<usize>), // implemented

    /// Sets a breakpoint at a marked instruction given its name.
    BreakpointMark(String), // implemented

    /// Removes a breakpoint at an instruction given its index. If the index is not provided, the
    /// program counter will be used.
    RemoveBreakpoint(Option<usize>), // implemented

    /// Removes a breakpoint at a marked instruction given its name.
    RemoveBreakpointMark(String), // implemented

    /// Runs the next command.
    Step,

    /// Runs until the next breakpoint is reached.
    ContinueToBreakpoint,

    /// Quits the debugger.
    Quit, // implemented
}
