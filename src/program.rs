use crate::error_system::OsuKeyboardError;
use crate::processor::keyboard_program::TemporaryProcessor;
use crate::processor::Processor;
use arduino_uno::hal::port::mode::Output;
use arduino_uno::hal::port::portb::PB5;
use arduino_uno::prelude::*;

pub fn run() -> Result<!, OsuKeyboardError> {
    let processor = TemporaryProcessor;

    processor.setup()?;

    processor.run()
}
