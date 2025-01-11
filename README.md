# brianFuck974

brianFuck974,
named after me.
My implementation of brainfuck in rust.
In this implementation,
initially 32 128-bit cells are used.
The cells are actually stored in a `Vec` of `i128`s,
so it could be extended whenever needed.

In order to run this interpreter,
you can use `cargo run`.
You can also `cargo run -- --help` for some hints on how to use this interpreter.
Suppose that your program is in the file `example.bf`,
you can run

```
cargo run -- example.bf
```

to run the program.

To install this interpreter on the system,
`cd` into the project directory and run

```
cargo install --path .
```

## Install & Uninstall

in the project directory.
After installation,
you can run `brianfuck974` to call this interpreter.
If you want to change the name of the command,
please refer to `Cargo.toml` and change the `name` under `[[bin]]` before installation.

If you want to uninstall this program,
please run

```
cargo uninstall brianFuck974
```

instead of

```
cargo uninstall brianfuck974
```

## Usage

This interpreter has 2 modes,
the first one is interpreter mode and the other one is debugger mode.

### Interpreter Mode

In order to run a brainfuck source file,
simply run
`brianfuck974 <path>`.

### Debugger Mode

The debugger mode is triggered by passing the `--debug` or `-d` flag.
For example,
to debug a brainfuck source file,
simply run
`brianfuck974 --debug <path>` or `brianfuck974 -d <path>`.

There are 32 debugger commands in total
(including a no-op that does not do anything),
here is a list of them:

| Command Name              | Short Trigger | Parameter                             | Functionality                                                                                                                                                                                                                                                                              |
| ------------------------- | ------------- | ------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| NoOp                      |               |                                       | A no-op command that does not do anything.                                                                                                                                                                                                                                                 |
| PrintInstruction          | `pi`          | `index: Option<usize>`                | Prints the instruction at an index. The parameter is the index. If not provided, the program counter will be used.                                                                                                                                                                         |
| PrintCell                 | `pc`          | `index: Option<usize>`                | Prints the content of the cell at an index. The parameter is the index. If not provided, the data pointer will be used.                                                                                                                                                                    |
| PrintAllInstructions      | `pai`         |                                       | Prints all instructions in the program.                                                                                                                                                                                                                                                    |
| PrintAllCells             | `pac`         |                                       | Prints all cells in the program.                                                                                                                                                                                                                                                           |
| ListInstruction           | `li`          | `index: Option<usize>`                | Lists 11 instructions around an index. For example, if the index is 10, this command will list the instructions from 5 to 15. If the index is not provided, the program counter will be used.                                                                                              |
| LongListInstruction       | `lli`         | `length: usize, index: Option<usize>` | Lists n (specified by the first parameter) instructions before and after an index (specified by the second parameter). For example, if `n = 5`, this command works the same as `ListInstruction(index)`. If the index is not provided, the program counter will be used.                   |
| ListMarkedInstruction     | `lmi`         | `mark: String`                        | Lists 11 instructions around an instruction given its name. For example, if the index is 10, this command will list the instructions from 5 to 15. If the index is not provided, the program counter will be used.                                                                         |
| LongListMarkedInstruction | `llmi`        | `length: usize, mark: String`         | Lists n (specified by the first parameter) instructions before and after a marked instruction (specified by the second parameter). For example, if `n = 5`, this command works the same as `ListMarkedInstruction(index)`. If the index is not provided, the program counter will be used. |
| ListCell                  | `lc`          | `index: Option<usize>`                | Lists 11 cells around an index. For example, if the index is 10, this command will list the cells from 5 to 15. If the index is not provided, the data pointer will be used.                                                                                                               |
| LongListCell              | `llc`         | `length: usize, index: Option<usize>` | Lists n (specified by the first parameter) cells before and after an index (specified by the second parameter). For example, if `n = 5`, this command works the same as `ListCell(index)`. If the index is not provided, the data pointer will be used.                                    |
| ListMarkedCell            | `lmc`         | `mark: String`                        | Lists 11 cells around an index. For example, if the index is 10, this command will list the cells from 5 to 15. If the index is not provided, the data pointer will be used.                                                                                                               |
| LongListMarkedCell        | `llmc`        | `length: usize, mark: String`         | Lists n (specified by the first parameter) cells before and after an index (specified by the second parameter). For example, if `n = 5`, this command works the same as `ListMarkedCell(index)`. If the index is not provided, the data pointer will be used.                              |
| SetCell                   | `sc`          | `value: Int, index: Option<usize>`    | Sets the value of a cell given its index. If the index is not provided, the data pointer will be used.                                                                                                                                                                                     |
| SetMarkedCell             | `smc`         | `value: Int, mark: String`            | Sets the value of a cell given its name.                                                                                                                                                                                                                                                   |
| RunInstruction            | `ri`          | `instruction: char`                   | Runs one of the 6 instructions, excluding the square brackets.                                                                                                                                                                                                                             |
| RunInstructions           | `ris`         | `instructions: String`                | Runs a sequence of the 6 instructions, excluding the square brackets.                                                                                                                                                                                                                      |
| Mark                      | `m`           | `mark: String, index: Option<usize>`  | Marks an instruction by its index. The first parameter is the name and the second parameter is the index. If the index is not provided, the program counter will be used.                                                                                                                  |
| MarkCell                  | `mc`          | `mark: String, index: Option<usize>`  | Marks an cell by its index. The first parameter is the name and the second parameter is the index. If the index is not provided, the data pointer will be used.                                                                                                                            |
| Jump                      | `j`           | `index: usize`                        | Sets the program counter to an instruction given its index.                                                                                                                                                                                                                                |
| JumpMark                  | `jm`          | `mark: String`                        | Sets the program counter to a marked instruction given its name.                                                                                                                                                                                                                           |
| JumpCell                  | `jc`          | `index: usize`                        | Sets the data pointer to a cell given its index.                                                                                                                                                                                                                                           |
| JumpMarkedCell            | `jmc`         | `mark: String`                        | Sets the data pointer to a marked cell given its name.                                                                                                                                                                                                                                     |
| JumpBack                  | `jb`          |                                       | Sets the program counter to the value before the last Jump(Mark) command.                                                                                                                                                                                                                  |
| JumpBackCell              | `jbc`         |                                       | Sets the data pointer to the value before the last Jump(Marked)Cell command.                                                                                                                                                                                                               |
| Breakpoint                | `b`           | `index: Option<usize>`                | Sets a breakpoint at an instruction given its index. If the index is not provided, the program counter will be used.                                                                                                                                                                       |
| BreakpointMark            | `bm`          | `mark: String`                        | Sets a breakpoint at a marked instruction given its name.                                                                                                                                                                                                                                  |
| RemoveBreakpoint          | `rb`          | `index: Option<usize>`                | Removes a breakpoint at an instruction given its index. If the index is not provided, the program counter will be used.                                                                                                                                                                    |
| RemoveBreakpointMark      | `rbm`         | `mark: String`                        | Removes a breakpoint at a marked instruction given its name.                                                                                                                                                                                                                               |
| Step                      | `s`           |                                       | Runs the next command.                                                                                                                                                                                                                                                                     |
| ContinueToBreakpoint      | `ctb`         |                                       | Runs until the next breakpoint is reached.                                                                                                                                                                                                                                                 |
| Quit                      | `q`           |                                       | Quits the debugger.                                                                                                                                                                                                                                                                        |

In addition,
each command also has a long trigger.
It is the command in snake case.
For example,
the long trigger of `LongListMarkedInstruction` is `long_list_marked_instruction`.

In order to use a command,
enter the trigger
(either short or long),
followed by its arguments,
separated by a single space.

For example,
in order to use `LongListCell` to list the content of 10 cells before and after the cell pointed to by the data pointer,
enter
`llc 10`
into the debugger.
In this case,
the `length` is passed as `10` and the `index` is not passed.
According to the table above,
the current data pointer will be used.
If you don't want to use the current data pointer,
you can pass the `index` as usual.
For example,
you can do `llc 10 49`.

When passing a mark,
remember that it should not contain any whitespace.
