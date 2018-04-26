use debugger::Debugger;
use debugger::commands::Command;

enum InfoType {
    Breakpoints,
    Registers,
}

pub struct Info(InfoType);

impl Info {
    fn print_breakpoints(&self, debugger: &Debugger) {
        println!("{} breakpoints:", debugger.breakpoints.len());
        for b in &debugger.breakpoints {
            println!("    {}", b);
        }
    }

    fn print_registers(&self, debugger: &Debugger) {
        let c = &debugger.cpu;
        println!(r#"Registers:
    A: {:02x} F: {:02x} B: {:02x} C: {:02x}
    D: {:02x} E: {:02x} H: {:02x} L: {:02x}
    SP: {:04x} PC: {:04x} Flags: {:08b}"#,
                 c.reg[0], c.reg[1], c.reg[2], c.reg[3],
                 c.reg[4], c.reg[5], c.reg[6], c.reg[7],
                 c.sp, c.pc, c.flag)
    }
}

impl Command for Info {
    type Item = Info;
    fn parse(cmd: &str) -> Option<Self::Item> {
        let mut split = cmd.split_whitespace();

        // Drop `i`
        split.next();

        split.next().and_then(|t| {
            match t {
                "r" | "registers" => Some(InfoType::Registers),
                "b" | "breakpoints" => Some(InfoType::Breakpoints),
                _ => None
            }
        }).map(Info)
    }

    fn execute(self, debugger: &mut Debugger) {
        match self.0 {
            InfoType::Breakpoints => self.print_breakpoints(debugger),
            InfoType::Registers => self.print_registers(debugger),
        }
    }
}
