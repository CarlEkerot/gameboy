use debugger::Debugger;
use debugger::commands::Command;

pub struct Next;

impl Command for Next {
    type Item = Next;
    fn parse(cmd: &str) -> Option<Self::Item> {
        Some(Next)
    }

    fn execute(self, debugger: &mut Debugger) {
        debugger.cpu.execute_next();
        let instruction = debugger.cpu.current_instruction().unwrap();
        println!("${:04x}: {}", debugger.cpu.pc, instruction);
    }
}
