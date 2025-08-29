use derive_more::Display;

#[derive(Display)]
pub enum DebuggingError {
    #[display("The index is out of bounds!")]
    IndexOutOfBounds,
    #[display("The mark does not exist!")]
    MarkNotFound,
    #[display("There is no jump back destination!")]
    JumpHistoryEmpty,
    #[display("The breakpoint does not exist!")]
    BreakpointNotFound,
    #[display("The instruction is invalid!")]
    InvalidInstruction,
}
