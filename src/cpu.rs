// http://marc.rawer.de/Gameboy/Docs/GBCPUman.pdf
use memory::Memory;
use instructions::{Instruction, Mnemonic, Operand};
use errors::*;
use constants::*;

// Allow dead code for now...
#[allow(dead_code)]
pub struct CPU {
    reg: [u8; 8],
    sp: u16,
    pc: u16,
    flag: u8,
    ram: Memory,
    cycles: usize,
}

impl CPU {
    pub fn new(ram: Memory) -> CPU {
        CPU {
            reg: [0; 8],
            sp: 0xfffe,
            pc: 0x100,
            flag: 0,
            ram,
            cycles: 0,
        }
    }

    pub fn execute(&mut self, instruction: &Instruction) -> Result<()> {
        // NOTE: When other cycle count?
        self.cycles += instruction.definition.cycles[0];
        match instruction.definition.mnemonic {
            Mnemonic::LD => Load::execute(instruction, self),
            _ => Ok(())
        }
    }
}

trait Execute {
    fn execute(instruction: &Instruction, cpu: &mut CPU) -> Result<()>;
}

struct Load;
impl Execute for Load {
    fn execute(instruction: &Instruction, cpu: &mut CPU) -> Result<()> {
        let dst = instruction.definition.operands.get(0)
            .chain_err(|| "Missing destination operand")?;
        let src = instruction.definition.operands.get(1)
            .chain_err(|| "Missing source operand")?;

        match (dst, src) {
            (&Operand::Register(r), &Operand::Immediate(BYTE)) => {
                cpu.reg[r] = instruction.immediate.map(|d| d as u8)
                    .chain_err(|| "Missing immediate")?
            },
            (&Operand::Register(r1), &Operand::Register(r2)) => {
                cpu.reg[r1] = cpu.reg[r2];
            },
            (&Operand::Register(r), &Operand::RegisterPair(h, l)) => {
                let addr = ((cpu.reg[h] as usize) << 8) | (cpu.reg[l] as usize);
                cpu.reg[r] = cpu.ram.load(addr);
            },
            (&Operand::Register(r), &Operand::Address(SHORT)) => {
                let addr = instruction.immediate.map(|d| d as usize)
                    .chain_err(|| "Missing immediate")?;
                cpu.reg[r] = cpu.ram.load(addr);
            },
            (&Operand::Register(r), &Operand::COffset(BYTE)) => {
                let addr = OFFSET_BASE + cpu.reg[REG_C] as usize;
                cpu.reg[r] = cpu.ram.load(addr);
            },
            (&Operand::SP, &Operand::RegisterPair(h, l)) => {
                cpu.sp = ((cpu.reg[h] as u16) << 8) | (cpu.reg[l] as u16);
            },
            (&Operand::SP, &Operand::Immediate(SHORT)) => {
                cpu.sp = instruction.immediate.chain_err(|| "Missing immediate")?;
            },
            (&Operand::RegisterPair(h, l), &Operand::Immediate(SHORT)) => {
                let val = instruction.immediate.chain_err(|| "Missing immediate")?;
                cpu.reg[h] = (val >> 8) as u8;
                cpu.reg[l] = (val & 0xff) as u8;
            },
            (&Operand::RegisterPair(h, l), &Operand::Immediate(BYTE)) => {
                let addr = ((cpu.reg[h] as usize) << 8) | (cpu.reg[l] as usize);
                let val = instruction.immediate.map(|d| d as u8).chain_err(|| "Missing immediate")?;
                cpu.ram.store(addr, val);
            },
            (&Operand::RegisterPair(h, l), &Operand::Register(r)) => {
                let addr = ((cpu.reg[h] as usize) << 8) | (cpu.reg[l] as usize);
                cpu.ram.store(addr, cpu.reg[r]);
            },
            (&Operand::RegisterPair(h, l), &Operand::SPOffset(BYTE)) => {
                // TODO: Was here!
            },
            (&Operand::Address(SHORT), &Operand::SP) => (),
            (&Operand::Address(SHORT), &Operand::Register(r)) => (),
            (&Operand::COffset(BYTE), &Operand::Register(r)) => (),
            _ => {
            },
        };
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use instruction_set::INSTRUCTIONS;
    use instructions::Instruction;
    use instructions::Operand;
    use instructions::OpCode;

    fn mock_instruction(code: &OpCode) -> Instruction {
        Instruction {
            addr: 0,
            definition: INSTRUCTIONS.get(code).unwrap(),
            immediate: Some(5),
        }
    }

    #[test]
    fn execute_loads() {
        let m = Memory::default();
        let mut cpu = CPU::new(m);
        let itr = INSTRUCTIONS.iter()
            .filter(|&(code, d)| d.mnemonic == Mnemonic::LD);
        for (code, _) in itr {
            cpu.execute(&mock_instruction(&code)).expect("FAILURE");
        }
    }

    #[test]
    fn build_opreand() {
        let s = Operand::Register(REG_A);
    }
}