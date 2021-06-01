use crate::error_system::OsuKeyboardError;
use crate::processor::keyboard_processor::{KeyboardProcessor, TemporaryProcessor};
use crate::processor::Processor;
use arduino_uno::hal::port::mode::Output;
use arduino_uno::hal::port::portb::PB5;
use arduino_uno::prelude::*;
use arduino_uno::Peripherals;

pub fn run(peripherals: Peripherals) -> Result<!, OsuKeyboardError> {
    let processor = KeyboardProcessor::new(peripherals);

    processor.setup()?;

    processor.run()
}
