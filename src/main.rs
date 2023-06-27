use clap::{App, Arg};


enum Command {
    MoveRight,
    MoveLeft,
    Increment,
    Decrement,
    Output,
    Input,
    JumpForward(usize),
    JumpBack(usize),
    Nothing,
}


enum RunningState {
    Running,
    Finished,
}


fn translate_into_commands(string: &String) -> Result<Vec<Command>, String> {
    let mut commands: Vec<Command>     = Vec::new();
    let mut pos_in_commands: Vec<usize> = Vec::new();

    let mut chars = string.chars();

    let mut current_cmd_ptr: isize = 0;

    loop {
        match chars.next() {
            Some(current_char) => {
                let current_cmd = match current_char {
                    '>' => Command::MoveRight,
                    '<' => Command::MoveLeft,
                    '+' => Command::Increment,
                    '-' => Command::Decrement,
                    '.' => Command::Output,
                    ',' => Command::Input,
                    '[' => {
                        pos_in_commands.push(current_cmd_ptr as usize);
                        Command::Nothing
                    },
                    ']' => {
                        if let Some(pos) = pos_in_commands.pop() {
                            commands[pos] = Command::JumpForward(current_cmd_ptr as usize);
                            Command::JumpBack(pos)
                        } else {
                            return Err(String::from("Syntax Error: '[' and ']' does not properly match."));
                        }
                    },
                    _   => {
                        Command::Nothing
                    },
                };
                current_cmd_ptr += 1;
                commands.push(current_cmd);
            },
            None => break,
        };
    };

    Ok(commands)
}


const ARRAY_SIZE: usize = 32768;
struct State {
    pointer: usize,
    array: [i32; ARRAY_SIZE],
    pc: usize,
    commands: Vec<Command>,
}


impl State {
    fn new(commands: Vec<Command>) -> State {
        State {
            pointer: 0,  // program counter
            array: [0; ARRAY_SIZE],
            pc: 0,
            commands
        }
    }

    fn execute_once(&mut self) -> Result<RunningState, String> {
        if self.pc >= self.commands.len() {
            return Ok(RunningState::Finished);
        }
        let current_cmd = &self.commands[self.pc];
        match *current_cmd {
            Command::MoveRight         => self.move_to_the_right()?,
            Command::MoveLeft          => self.move_to_the_left()?,
            Command::Increment         => self.increment()?,
            Command::Decrement         => self.decrement()?,
            Command::Input             => self.input()?,
            Command::Output            => self.output()?,
            Command::JumpForward(pos)  => self.jump_forward(pos)?,
            Command::JumpBack(pos)     => self.jump_back(pos)?,
            Command::Nothing           => self.pass()?,
        };
        self.pc += 1;
        Ok(RunningState::Running)
    }

    fn move_to_the_right(&mut self) -> Result<RunningState, String> {
        self.pointer += 1;
        if self.pointer >= self.array.len() {
            Err(String::from("Index Error: You have gone too far to the right!"))
        } else {
            Ok(RunningState::Running)
        }
    }

    fn move_to_the_left(&mut self) -> Result<RunningState, String> {
        if self.pointer == 0 {
            Err(String::from("Index Error: You have gone too far to the left!"))
        } else {
            self.pointer -= 1;
            Ok(RunningState::Running)
        }
    }

    fn increment(&mut self) -> Result<RunningState, String> {
        if self.array[self.pointer] + 1 > std::i32::MAX {
            Err(String::from("Overflow Error: The number in the cell is too large!"))
        } else {
            self.array[self.pointer] += 1;
            Ok(RunningState::Running)
        }
    }

    fn decrement(&mut self) -> Result<RunningState, String> {
        if self.array[self.pointer] - 1 < std::i32::MIN {
            Err(String::from("Overflow Error: The number in the cell is too small!"))
        } else {
            self.array[self.pointer] -= 1;
            Ok(RunningState::Running)
        }
    }

    fn output(&mut self) -> Result<RunningState, String> {
        let data = self.array[self.pointer];
        if data == data as i32 {
            if let Some(converted_char) = char::from_u32(data as u32) {
                print!("{}", converted_char);
            } else {
                return Err(String::from("Value Error: The value in the cell is not a valid Unicode character!"));
            }
            Ok(RunningState::Running)
        } else {
            Err(String::from("Value Error: The value in the cell is not a valid Unicode character!"))
        }
    }

    fn input(&mut self) -> Result<RunningState, String> {
        println!("Please enter a character; if you enter more than one character, only the first one will be taken:");
        use std::io;

        let mut input_buffer = String::new();
        if let Err(_) = io::stdin().read_line(&mut input_buffer) {
            return Err(String::from("IO Error: Unable to get character input!"));
        };

        if let Some(input_char) = input_buffer.chars().next() {
            self.array[self.pointer] = input_char as i32;
        } else {
            return Err(String::from("Value Error: Don't enter an empty string!"));
        };

        Ok(RunningState::Running)
    }

    fn jump_forward(&mut self, pos: usize) -> Result<RunningState, String> {
        if pos > self.array.len() {
            Err(String::from("Runtime Error: You have gone too far to the right!"))
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

    fn pass(&mut self) -> Result<RunningState, String> {
        Ok(RunningState::Running)
    }
}


fn main() {
    use std::fs;
    let args = App::new("brainfuck")
        .version("0.1")
        .about("A brainfuck interpreter written in rust.")
        .arg(Arg::with_name("mode")
             .help("Has 2 possible values: 'file' and 'cl'. 'file' stands for file mode, and 'cl' stands for command line mode.")
             .takes_value(true)
             .required(true))
        .arg(Arg::with_name("input")
             .help("In file mode, a file name should be provided. In cl mode, a string of source could should be provided in the command line.")
             .takes_value(true)
             .required(true))
        .get_matches();

    let mode = args.value_of("mode").unwrap();
    let input = args.value_of("input").unwrap();

    let mut cmd_string = String::new();

    if mode == "cl" {
        cmd_string = cmd_string + input;
    } else if mode == "file" {
        match fs::read_to_string(input) {
            Ok(string)  => {
                cmd_string = string;
            },
            Err(_)      => panic!("Failed to read from the file!"),
        };
    }

    match translate_into_commands(&cmd_string) {
        Ok(commands)    => {
            let mut state = State::new(commands);
            loop {
                match state.execute_once() {
                    Ok(RunningState::Running)   => {},
                    Ok(RunningState::Finished)  => {
                        break;
                    },
                    Err(info)                   => panic!("{}", info),
                }
            };
        },
        Err(info)       => panic!("{}", info),
    };
}
