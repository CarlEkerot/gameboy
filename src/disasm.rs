use std::io::{Read, Bytes};
use std::fs::File;
use instructions::{
    Operand,
    OpCode,
    Instruction,
    Definition,
    IMMEDIATES,
    INVALID,
};
use constants::EXTENDED_INSTRUCTION;
use instruction_set::INSTRUCTIONS;
use errors::*;

#[derive(Debug)]
struct Program {
    bytes: Bytes<File>,
    offset: usize,
}

impl Program {
    pub fn new(file: File) -> Program {
        Program {
            bytes: file.bytes(),
            offset: 0,
        }
    }

    fn next_byte(&mut self) -> Option<u8> {
        self.offset += 1;
        match self.bytes.next() {
            None => None,
            Some(Err(e)) => panic!("{}", e),
            Some(Ok(res)) => Some(res),
        }
    }

    fn next_byte_checked(&mut self) -> Result<u8> {
        self.next_byte().chain_err(|| "Failed to read byte")
    }

    fn next_short(&mut self) -> Result<u16> {
        self.next_byte()
            .and_then(|l| self.next_byte().map(|h| (h, l)))
            .map(|(h, l)| (h as u16) << 8 | (l as u16))
            .chain_err(|| "Failed to parse short")
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

    fn parse_immediate(&mut self, definition: &Definition) -> Result<Option<u16>> {
        match definition.operands.iter().find(|&o| IMMEDIATES.contains(o)) {
            // TODO: Take care of signedness!
            Some(&Operand::Immediate(8)) | Some(&Operand::Address(8)) |
            Some(&Operand::Offset(8)) | Some(&Operand::SPOffset(8)) =>
                self.next_byte_checked().map(|b| Some(b as u16)),
            Some(&Operand::Immediate(16)) | Some(&Operand::Address(16)) =>
                self.next_short().map(Some),
            _ => Ok(None),
        }
    }

    // Not currently applicable to Iterator trait due to bad lifetime support
    // for associated types.
    pub fn next(&mut self) -> Option<Instruction> {
        let addr = self.offset as u16;
        self.parse_opcode()
            .and_then(|c| INSTRUCTIONS.get(&c).or(Some(&INVALID)))
            .and_then(|d| self.parse_immediate(&d).ok().map(|i| (d, i)))
            .map(|(definition, immediate)| {
                Instruction {
                    addr,
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
        let f = File::open("/home/kalle/temp/test3.rom").unwrap();
        let mut p = Program::new(f);
        println!("Next: {:?}", p.next());
        println!("Next: {:?}", p.next());
    }

    #[test]
    fn test_load_boot_rom() {
        let f = File::open("/home/kalle/temp/boot.gb").unwrap();
        let mut p = Program::new(f);

        while let Some(i) = p.next() {
            println!("{:04x}: {}", i.addr, i);
        };
    }
}