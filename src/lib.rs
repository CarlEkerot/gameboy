#[macro_use]
extern crate error_chain;

extern crate sdl2;

mod errors {
    error_chain!{}
}

pub mod constants;
pub mod memory;
pub mod cpu;
pub mod definition;
pub mod instructions;
pub mod instruction_set;
pub mod interrupts;
pub mod operations;
pub mod lcd;
pub mod debugger;
pub mod emulator;

mod test_helpers;
