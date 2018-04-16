use cpu::CPU;
use memory::Memory;
use instructions::{Instruction, Mnemonic, OpCode};
use instruction_set::INSTRUCTIONS;

fn mock_instruction(code: &OpCode) -> Instruction {
    Instruction {
        addr: 0x100,
        definition: INSTRUCTIONS.get(code).unwrap(),
        immediate: Some(5),
    }
}

pub fn execute_all(mnemonic: Mnemonic) {
    let m = Memory::default();
    let mut cpu = CPU::new(m);
    let itr = INSTRUCTIONS.iter()
        .filter(|&(_, d)| d.mnemonic == mnemonic);
    for (code, _) in itr {
        cpu.execute(&mock_instruction(&code)).expect("FAILURE");
    }
}