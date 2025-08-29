use derive_more::Display;

#[derive(Clone, Display)]
pub enum ExecutorCommand {
    MoveRight,
    MoveLeft,
    Increment,
    Decrement,
    Output,
    Input,
    #[display("JumpForward({})", _0)]
    JumpForward(usize),
    #[display("JumpBack({})", _0)]
    JumpBack(usize),
}
