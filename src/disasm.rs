use std::collections::HashMap;
use std::io::{Cursor, Read, Bytes};
use std::fs::File;

const EXTENDED_INSTRUCTION: u16 = 0xcb;

type OpCode = u16;

#[derive(Debug)]
enum Operation {
    LD,
    LDD,
    LDI,
    LDH,
    PUSH,
    POP,
    ADD,
    ADC,
    SUB,
    SBC,
    AND,
    OR,
    XOR,
    CP,
    INC,
    DEC,
    SWAP,
    DAA,
    CPL,
    CCF,
    SCF,
    NOP,
    HALT,
    STOP,
    DI,
    EI,
    RLCA,
    RLA,
    RRCA,
    RRA,
    RLC,
    RL,
    RRC,
    RR,
    SLA,
    SRA,
    SRL,
    BIT,
    SET,
    RES,
    JP,
    JR,
    CALL,
    RST,
    RET,
    RETI,
}

#[derive(Debug)]
enum Operand {
    B,
    C,
    D,
    E,
    H,
    L,
    A,
    BC,
    DE,
    HL,
    SP,
    AF,
    Immediate8,
    Immediate16,
    Address8,
    Address16,
}

#[derive(Debug)]
enum FlagAction {
    Unchanged,
    Set,
    Reset,
    Function,
}

#[derive(Debug)]
struct InstructionDefinition {
    operation: Operation,
    code: OpCode,
    num_operands: usize,
    operands: [Operand; 2],
    cycles: u8,
    flags: [FlagAction; 4],
}

#[derive(Debug)]
struct Instruction<'a> {
    definition: &'a InstructionDefinition,
    immediate: u16,
}

type InstructionSet = HashMap<OpCode, InstructionDefinition>;

#[derive(Debug)]
struct Program<'a> {
    data: Bytes<File>,
    instruction_set: &'a InstructionSet,
}

impl<'a> Program<'a> {
    pub fn new(file: File, instruction_set: &'a InstructionSet) -> Program<'a> {
        Program {
            data: file.bytes(),
            instruction_set,
        }
    }

    fn next_byte(&mut self) -> Option<u8> {
        match self.data.next() {
            None => return None,
            Some(Err(e)) => panic!("{}", e),
            Some(Ok(res)) => Some(res),
        }
    }

    fn next_short(&mut self) -> Option<u16> {
        let lo = self.next_byte().map(|b| b as u16);
        let hi = self.next_byte().map(|b| b as u16);

        match (hi, lo) {
            (Some(h), Some(l)) => Some((h << 8) | l),
            _ => None,
        }
    }

    fn parse_opcode(&mut self) -> Option<OpCode> {
        let first = self.next_byte().map(|b| b as u16);

        first.map(|b| if b == EXTENDED_INSTRUCTION {
            // Extended instruction
            let second = self.next_byte().unwrap() as u16;
            b << 8 | second
        } else {
            b
        })
    }

    fn parse_immediate(&mut self, definition: &InstructionDefinition) -> Option<u16> {
        let v: Vec<_> = definition.operands.iter().flat_map(|o| match *o {
            Operand::Immediate8 | Operand::Address8 => self.next_byte().map(|b| b as u16),
            Operand::Immediate16 | Operand::Address16 => self.next_short(),
            _ => None
        }).collect();
        v.get(0).map(|d| d.clone())
    }

    // Not currently applicable to Iterator trait due to bad lifetime support
    // for associated types.
    fn next(&mut self) -> Option<Instruction> {
        self.parse_opcode()
            .and_then(|c| self.instruction_set.get(&c))
            .and_then(|d| self.parse_immediate(&d).map(|i| (d, i)))
            .map(|(definition, immediate)| {
                Instruction {
                    definition,
                    immediate,
                }
            })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        let i = InstructionDefinition {
            operation: Operation::LD,
            code: 0x02,
            num_operands: 0,
            operands: [Operand::BC, Operand::A],
            cycles: 8,
            flags: [
                FlagAction::Unchanged,
                FlagAction::Unchanged,
                FlagAction::Unchanged,
                FlagAction::Unchanged,
            ]
        };

        let i2 = InstructionDefinition {
            operation: Operation::LD,
            code: 0xfa,
            num_operands: 2,
            operands: [Operand::A, Operand::Address16],
            cycles: 16,
            flags: [
                FlagAction::Unchanged,
                FlagAction::Unchanged,
                FlagAction::Unchanged,
                FlagAction::Unchanged,
            ]
        };

        let mut map = HashMap::new();
        map.insert(0x02, i);
        map.insert(0xfa, i2);

        let mut f = File::open("/home/kalle/temp/test3.rom").unwrap();
        let mut p = Program::new(f, &map);
        println!("Next: {:?}", p.next());
    }
}