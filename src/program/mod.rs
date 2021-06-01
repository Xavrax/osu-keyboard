use crate::error_system::OsuKeyboardError;

pub mod keyboard_program;

pub trait Program {
    fn setup(&self) -> Result<(), OsuKeyboardError>;
    fn run(&self) -> Result<!, OsuKeyboardError>;
}
