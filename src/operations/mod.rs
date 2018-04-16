mod add;
mod inc;
mod dec;
mod load;
mod ldi;
mod ldd;
mod nop;
mod rla;
mod rlca;

pub use self::add::Add;
pub use self::inc::Increase;
pub use self::dec::Decrease;
pub use self::load::Load;
pub use self::ldi::LoadIncrease;
pub use self::ldd::LoadDecrease;
pub use self::nop::Nop;
pub use self::rla::RotateALeft;
pub use self::rlca::RotateALeftCarry;

use instructions::Instruction;
use cpu::CPU;
use errors::*;

pub trait Execute {
    fn execute(instruction: &Instruction, cpu: &mut CPU) -> Result<()>;
}
