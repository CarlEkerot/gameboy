use cpu::CPU;
use memory::Memory;
use instructions::Instruction;
use definition::{Mnemonic, OpCode};
use instruction_set::get_definition;
use definition::Definition;

fn mock_instruction(definition: &'static Definition) -> Instruction {
    Instruction {
        definition,
        immediate: Some(5),
    }
}

pub fn execute_all(mnemonic: Mnemonic) {
    let m = Memory::default();
    let mut cpu = CPU::new(m);
    let itr = (0..512).map(get_definition)
        .filter(|&d| d.mnemonic == mnemonic);
    for d in itr {
        cpu.execute(&mock_instruction(&d)).expect("FAILURE");
    }
}

pub fn execute_instruction(cpu: &mut CPU, code: OpCode, immediate: Option<u16>) {
    let i = Instruction {
        definition: get_definition(code),
        immediate,
    };

    cpu.execute(&i).unwrap();
}
