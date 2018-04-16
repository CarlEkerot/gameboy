use std::fmt;

pub type OpCode = u16;

#[derive(Debug, PartialEq)]
pub enum Mnemonic {
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
    INVALID,
}

#[derive(Debug, PartialEq)]
pub enum Operand {
    Register(usize),
    RegisterAddr(usize),
    RegisterPair(usize, usize),
    RegisterPairAddr(usize, usize),
    SP,
    Zero,
    NonZero,
    Carry,
    NonCarry,
    Immediate(usize),
    Address(usize),
    Offset(usize),
    SPOffset(usize),
    RSTOffset(usize),
    Bit(usize),
    None,
}

pub const IMMEDIATES: [Operand; 6] = [
    Operand::Immediate(8),
    Operand::Immediate(16),
    Operand::Address(8),
    Operand::Address(16),
    Operand::Offset(8),
    Operand::SPOffset(8),
];

impl Operand {
    pub fn as_string(&self, immediate: Option<u16>) -> String {
        match *self {
            Operand::Immediate(8) | Operand::Immediate(16) |
            Operand::Address(8) | Operand::Address(16) =>
                immediate.map(|i| format!("${:04x}", i)),
            Operand::Offset(8) => immediate.map(|i| {
                let signed = i as i8;
                if signed.is_negative() {
                    format!("-${:04x}", -signed)
                } else {
                    format!("${:04x}", signed)
                }
            }),
            Operand::SPOffset(8) => immediate.map(|i| {
                let signed = i as i8;
                if signed.is_negative() {
                    format!("(SP - ${:04x})", -signed)
                } else {
                    format!("(SP + ${:04x})", signed)
                }
            }),
            _ => Some(format!("{}", self)),
        }.unwrap_or_else(|| String::from("BAD"))
    }
}

impl fmt::Display for Operand {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let repr = match *self {
            Operand::Zero => String::from("Z"),
            Operand::NonZero => String::from("NZ"),
            Operand::Carry => String::from("($FF00+C)"),
            Operand::NonCarry => String::from("NC"),
            Operand::Bit(d) => format!("{}", d),
            _ => format!("{:?}", self),
        };
        write!(f, "{}", repr)
    }
}

#[derive(Debug)]
pub enum Flag {
    Unchanged,
    Set,
    Reset,
    Function,
}

#[derive(Debug)]
pub struct Definition {
    pub mnemonic: Mnemonic,
    pub code: OpCode,
    pub operands: [Operand; 2],
    pub cycles: [usize; 2],
    pub flags: [Flag; 4],
}

pub const INVALID: Definition = Definition {
    mnemonic: Mnemonic::INVALID,
    code: 0xffff,
    operands: [Operand::None, Operand::None],
    cycles: [0, 0],
    flags: [
        Flag::Unchanged,
        Flag::Unchanged,
        Flag::Unchanged,
        Flag::Unchanged,
    ]
};

#[derive(Debug)]
pub struct Instruction<'a> {
    pub addr: u16,
    pub definition: &'a Definition,
    pub immediate: Option<u16>,
}

impl<'a> fmt::Display for Instruction<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let a = self.definition.operands.get(0).unwrap_or(&Operand::None);
        let b = self.definition.operands.get(1).unwrap_or(&Operand::None);
        let a_str = a.as_string(self.immediate);
        let b_str = b.as_string(self.immediate);

        let s = match (a, b) {
            (&Operand::None, &Operand::None) =>
                format!("{:?}", self.definition.mnemonic),
            (_, &Operand::None) =>
                format!("{:?} {}", self.definition.mnemonic, a_str),
            (&Operand::None, _) =>
                format!("{:?} {}", self.definition.mnemonic, b_str),
            _ => {
                format!("{:?} {}, {}", self.definition.mnemonic, a_str, b_str)
            },
        };
        write!(f, "{}", s)
    }
}
