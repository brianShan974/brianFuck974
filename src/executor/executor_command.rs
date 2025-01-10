#[derive(Clone)]
pub enum ExecutorCommand {
    MoveRight,
    MoveLeft,
    Increment,
    Decrement,
    Output,
    Input,
    JumpForward(usize),
    JumpBack(usize),
}

impl std::fmt::Display for ExecutorCommand {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::MoveRight => write!(f, "MoveRight"),
            Self::MoveLeft => write!(f, "MoveLeft"),
            Self::Increment => write!(f, "Increment"),
            Self::Decrement => write!(f, "Decrement"),
            Self::Output => write!(f, "Output"),
            Self::Input => write!(f, "Input"),
            Self::JumpForward(pos) => write!(f, "JumpForward({})", pos),
            Self::JumpBack(pos) => write!(f, "JumpBack({})", pos),
        }
    }
}
