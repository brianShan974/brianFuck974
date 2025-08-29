use derive_more::Display;

#[derive(Display)]
pub enum ExecutionError {
    #[display("Index Error: The program counter is out of range.")]
    ProgramCounterOutOfRange,
    #[display("Index Error: You have gone too far to the left!")]
    TooFarLeft,
    #[display("Index Error: You have gone too far to the right!")]
    TooFarRight,
    #[display("Overflow Error: The number in the cell has reached its maximum!")]
    Overflow,
    #[display("Overflow Error: The number in the cell has reached its minimum!")]
    Underflow,
    #[display("Value Error: The value in the cell is not a valid Unicode character!")]
    InvalidCharacter,
    #[display("IO Error: Unable to get character input!")]
    InputError,
}
