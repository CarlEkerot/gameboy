use cpu::CPU;
use memory::Memory;
use instructions::Instruction;
use definition::{Mnemonic, OpCode};
use instruction_set::INSTRUCTIONS;

fn mock_instruction(code: &OpCode) -> Instruction {
    Instruction {
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

pub fn execute_instruction(cpu: &mut CPU, code: OpCode, immediate: Option<u16>) {
    let i = Instruction {
        definition: INSTRUCTIONS.get(&code).unwrap(),
        immediate,
    };

    cpu.execute(&i).unwrap();
}
