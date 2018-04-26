use std::fmt;
use cpu::CPU;
use debugger::Debugger;
use debugger::commands::Command;

pub struct Breakpoint(pub u16);

impl fmt::Display for Breakpoint {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "${:04x}", self.0)
    }
}

impl Command for Breakpoint {
    type Item = Breakpoint;
    fn parse(cmd: &str) -> Option<Self::Item> {
        let mut split = cmd.split_whitespace();

        // Drop `b`
        split.next();

        split.next().and_then(|addr_str| {
            Breakpoint::parse_number(addr_str)
        }).map(|addr| Breakpoint(addr as u16))
    }

    fn execute(self, debugger: &mut Debugger) {
        println!("Added breakpoint at ${:04x}", &self.0);
        debugger.add_breakpoint(self);
    }
}
