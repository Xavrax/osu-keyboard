#![no_std]
#![no_main]
#![feature(never_type)]

mod error_system;
mod program;

extern crate panic_halt;

use crate::program::keyboard_program::TemporaryProgram;
use crate::program::Program;
use arduino_uno::hal::port::mode::Output;
use arduino_uno::hal::port::portb::PB5;
use arduino_uno::prelude::*;
use crate::error_system::OsuKeyboardError;

#[arduino_uno::entry]
fn main() -> ! {
    if let Some(peripherals) = arduino_uno::Peripherals::take() {
        let program = TemporaryProgram;

        if let Err(err) = program.setup() {
            return error_system::report(err);
        }

        program
            .run()
            .unwrap_or_else(|err| error_system::report(err))
    } else {
        error_system::report(OsuKeyboardError::Unknown)
    }
}
