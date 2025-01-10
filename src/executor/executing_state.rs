pub type ExecutionResult = Result<ExecutionState, String>;

pub enum ExecutionState {
    Running,
    Finished,
}
