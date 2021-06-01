#![no_std]
#![no_main]
#![feature(never_type)]

mod error_system;
mod processor;
mod program;

extern crate panic_halt;

#[arduino_uno::entry]
fn main() -> ! {
    program::run().unwrap_or_else(|err| error_system::report(err))
}
