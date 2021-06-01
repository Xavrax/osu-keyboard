use crate::error_system::OsuKeyboardError;
use crate::processor::Processor;
use arduino_uno::Peripherals;

pub struct KeyboardProcessor {
    peripherals: Peripherals,
}

impl KeyboardProcessor {
    pub fn new(peripherals: Peripherals) -> Self {
        Self { peripherals }
    }
}

impl Processor for KeyboardProcessor {
    fn setup(&self) -> Result<(), OsuKeyboardError> {
        Ok(())
    }

    fn run(&self) -> Result<!, OsuKeyboardError> {
        Err(OsuKeyboardError::InitializationFailed)
    }
}
