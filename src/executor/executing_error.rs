pub enum ExecutionError {
    ProgramCounterOutOfRange,
    TooFarLeft,
    TooFarRight,
    Overflow,
    Underflow,
    InvalidCharacter,
    InputError,
}

impl std::fmt::Display for ExecutionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ProgramCounterOutOfRange => {
                write!(f, "Index Error: The program counter is out of range.")
            }
            Self::TooFarLeft => write!(f, "Index Error: You have gone too far to the left!"),
            Self::TooFarRight => write!(f, "Index Error: You have gone too far to the right!"),
            Self::Overflow => write!(
                f,
                "Overflow Error: The number in the cell has reached its maximum!"
            ),
            Self::Underflow => write!(
                f,
                "Overflow Error: The number in the cell has reached its minimum!"
            ),
            Self::InvalidCharacter => write!(
                f,
                "Value Error: The value in the cell is not a valid Unicode character!"
            ),
            Self::InputError => write!(f, "IO Error: Unable to get character input!"),
        }
    }
}
