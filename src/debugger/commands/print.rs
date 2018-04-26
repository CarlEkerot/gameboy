use debugger::Debugger;
use debugger::commands::{Breakpoint, Command};

enum PrintType {
    PrintByte,
}

pub struct Print(PrintType, u16);

impl Command for Print {
    type Item = Print;
    fn parse(cmd: &str) -> Option<Self::Item> {
        let mut split = cmd.split_whitespace();

        // Drop `x`
        split.next();


        // TODO: Add support for more print types
        split.next()
            .and_then(Breakpoint::parse_number)
            .map(|a| Print(PrintType::PrintByte, a as u16))
    }

    fn execute(self, debugger: &mut Debugger) {
        match self.0 {
            PrintType::PrintByte => {
                let val = debugger.cpu.ram.load(self.1 as usize);
                println!("${:04x}: 0x{:02x}", self.1, val)
            },
        }
    }
}
