pub enum DebuggingError {
    IndexOutOfBounds,
    MarkNotFound,
    JumpHistoryEmpty,
    BreakpointNotFound,
}

impl std::fmt::Display for DebuggingError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::IndexOutOfBounds => write!(f, "The index is out of bounds!"),
            Self::MarkNotFound => write!(f, "The mark does not exist!"),
            Self::JumpHistoryEmpty => write!(f, "There is no jump back destination!"),
            Self::BreakpointNotFound => write!(f, "The breakpoint does not exist!"),
        }
    }
}
