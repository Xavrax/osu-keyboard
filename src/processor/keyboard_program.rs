use crate::error_system::OsuKeyboardError;
use crate::processor::Processor;

pub struct TemporaryProcessor;

impl Processor for TemporaryProcessor {
    fn setup(&self) -> Result<(), OsuKeyboardError> {
        Ok(())
    }

    fn run(&self) -> Result<!, OsuKeyboardError> {
        Err(OsuKeyboardError::InitializationFailed)
    }
}
