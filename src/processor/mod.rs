use crate::error_system::OsuKeyboardError;

pub mod keyboard_processor;

pub trait Processor {
    fn setup(&self) -> Result<(), OsuKeyboardError>;
    fn run(&self) -> Result<!, OsuKeyboardError>;
}
