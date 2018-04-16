mod inc;
mod dec;
mod load;
mod ldi;
mod ldd;

pub use self::inc::Increase;
pub use self::dec::Decrease;
pub use self::load::Load;
pub use self::ldi::LoadIncrease;
pub use self::ldd::LoadDecrease;

use instructions::Instruction;
use cpu::CPU;
use errors::*;

pub trait Execute {
    fn execute(instruction: &Instruction, cpu: &mut CPU) -> Result<()>;
}
