#[macro_use]
extern crate error_chain;

#[macro_use]
extern crate lazy_static;

extern crate sdl2;

mod errors {
    error_chain!{}
}

pub mod constants;
pub mod memory;
pub mod cpu;
pub mod disasm;
pub mod instructions;
pub mod instruction_set;
pub mod operations;
pub mod lcd;

mod test_helpers;
