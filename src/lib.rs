#[macro_use]
extern crate error_chain;

#[macro_use]
extern crate lazy_static;

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
mod test_helpers;