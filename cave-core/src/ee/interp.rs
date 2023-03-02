#[derive(Debug)]
pub enum StepMode {
    Forever,
    EndOfBlock,
    MaxBlocks(usize),
    MaxInstructions(usize),
}

#[derive(Debug)]
pub enum StepResult {
    Ok(usize),
    Exited(usize),
}

#[derive(Debug)]
pub enum StepError {
    DecodeError,
    Exception,
}

pub trait Interpreter {
    fn step(&mut self, mode: StepMode) -> Result<StepResult, StepError>;
}
