use crate::error_system::OsuKeyboardError;
use crate::program::Program;

pub struct TemporaryProgram;

impl Program for TemporaryProgram {
    fn setup(&self) -> Result<(), OsuKeyboardError> {
        Ok(())
    }

    fn run(&self) -> Result<!, OsuKeyboardError> {
        Err(OsuKeyboardError::InitializationFailed)
    }
}
