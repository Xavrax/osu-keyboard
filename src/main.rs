#![no_std]
#![no_main]
#![feature(never_type)]

mod error_system;
mod processor;
mod program;

extern crate panic_halt;

#[arduino_uno::entry]
fn main() -> ! {
    let peripherals =
        arduino_uno::Peripherals::take().expect("Cannot take Peripherals and handle error!");

    program::run(peripherals).unwrap_or_else(|err| error_system::report(err))
}
