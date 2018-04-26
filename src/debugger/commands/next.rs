use debugger::Debugger;
use debugger::commands::Command;

pub struct Next;

impl Command for Next {
    type Item = Next;
    fn parse(cmd: &str) -> Option<Self::Item> {
        Some(Next)
    }

    fn execute(self, debugger: &mut Debugger) {
        print!("${:04x}: ", debugger.cpu.pc);
        let instruction = debugger.cpu.execute_next();
        print!("{}\n", instruction);
    }
}
