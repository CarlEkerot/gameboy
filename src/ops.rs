use cpu::CPU;
use std::collections::HashMap;
use serde_json;

#[derive(Debug)]
enum Parameter {
    A,
    B,
    C,
    D,
    E,
    H,
    L,
    HL,
    Immediate,
}

pub struct Instruction {
    code: u16,

}

// #[derive(Debug, Serialize, Deserialize)]
// #[derive(Debug)]
pub struct InstructionDefinition<'a> {
    name: &'a str,
    prefix: u8,
    code: u8,
    params: [Option<Parameter>; 2],
    cycles: usize,
}

/*
impl<'a> Instruction<'a> {
    pub fn parse(json: &str) -> Instruction {
        serde_json::from_str(json).unwrap()
    }

    pub fn execute(&self, cpu: &mut CPU) {
        match self.name {
            "ld" => Load::execute(&self, cpu),
            "mv" => Move::execute(&self, cpu),
            _ => panic!("Unknown op")
        }
    }
}
*/

// #[derive(Debug)]
pub struct InstructionSet<'a> {
    instructions: HashMap<u16, InstructionDefinition<'a>>,
}

impl<'a> InstructionSet<'a> {
    /*
    pub fn parse(json: &str) -> InstructionSet {
        let ins: Vec<Instruction> = serde_json::from_str(json).unwrap();
        InstructionSet {
            instructions: ins
                .into_iter()
                .map(|i| (i.code, i))
                .collect()
        }
    }

    pub fn lookup(&self, code: u16) -> &Instruction {
        self.instructions.get(&code).expect("Unknown opcode")
    }
    */
}

#[cfg(test)]
mod tests {
    use super::*;

    /*
    #[test]
    fn test_serialize_single() {
        let s = r#"
        {
            "name": "ld",
            "code": 10,
            "param1": 5,
            "param2": 6,
            "cycles": 4
        }
        "#;
        println!("{:?}", Instruction::parse(s))
    }

    #[test]
    fn test_serialize_multiple() {
        let s = r#"
        [
            {
                "name": "ld",
                "code": 10,
                "param1": 5,
                "param2": 6,
                "cycles": 4
            },
            {
                "name": "mov",
                "code": 11,
                "param1": 2,
                "param2": 3,
                "cycles": 8
            }
        ]
        "#;
        println!("{:?}", InstructionSet::parse(s))
    }
    */
    #[test]
    fn test_execute() {
        let i = InstructionDefinition {
            name: "LD",
            code: 0x06,
            params: [
                Some(Parameter::B),
                Some(Parameter::Immediate)
            ],
            cycles: 8
        };
    }
}
