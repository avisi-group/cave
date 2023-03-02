use super::CpuState;
use std::marker::PhantomData;

#[derive(Debug)]
pub enum RunMode {
    Forever,
    EndOfBlock,
    MaxBlocks(usize),
    MaxInstructions(usize),
}

#[derive(Debug)]
pub enum RunResult {
    Ok,
    Exited,
}

#[derive(Debug)]
pub enum RunError {
    GenericError,
    UndefinedInstruction,
}

pub enum ExecutionResult {
    Continue,
    Exit,
    Abort,
}

pub trait InstructionExecutor<R> {
    fn execute(state: &mut CpuState<R>) -> ExecutionResult;
}

pub struct Interpreter<R, E: InstructionExecutor<R>> {
    regstate_type: PhantomData<R>,
    executor_type: PhantomData<E>,
}

impl<R, E: InstructionExecutor<R>> Interpreter<R, E> {
    pub fn new() -> Self {
        Self {
            regstate_type: PhantomData,
            executor_type: PhantomData,
        }
    }

    pub fn run(mode: RunMode, state: &mut CpuState<R>) -> Result<(RunResult, usize), RunError> {
        match mode {
            RunMode::Forever => loop {
                match E::execute(state) {
                    ExecutionResult::Continue => {}
                    ExecutionResult::Exit => {
                        return Ok((RunResult::Exited, 0));
                    }
                    _ => {
                        return Err(RunError::GenericError);
                    }
                }
            },
            _ => Err(RunError::GenericError),
        }
    }
}
