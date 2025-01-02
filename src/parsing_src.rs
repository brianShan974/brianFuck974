use crate::Command;

pub fn translate_into_commands(string: &str) -> Result<Vec<Command>, String> {
    let mut commands: Vec<Command> = Vec::new();
    let mut pos_in_commands: Vec<usize> = Vec::new();

    let mut current_cmd_ptr: usize = 0;

    for current_char in string.chars() {
        let current_cmd = match current_char {
            '>' => Command::MoveRight,
            '<' => Command::MoveLeft,
            '+' => Command::Increment,
            '-' => Command::Decrement,
            '.' => Command::Output,
            ',' => Command::Input,
            '[' => {
                pos_in_commands.push(current_cmd_ptr);
                Command::JumpForward(0)
            }
            ']' => {
                if let Some(pos) = pos_in_commands.pop() {
                    commands[pos] = Command::JumpForward(current_cmd_ptr);
                    Command::JumpBack(pos)
                } else {
                    return Err(String::from(
                        "Syntax Error: '[' and ']' does not properly match.",
                    ));
                }
            }
            _ => {
                continue;
            }
        };
        current_cmd_ptr += 1;
        commands.push(current_cmd);
    }

    Ok(commands)
}