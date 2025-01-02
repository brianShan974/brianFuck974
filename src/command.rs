pub enum Command {
    MoveRight,
    MoveLeft,
    Increment,
    Decrement,
    Output,
    Input,
    JumpForward(usize),
    JumpBack(usize),
}
