use debugger::Debugger;
use debugger::commands::Command;

pub struct Continue;

impl Command for Continue {
    type Item = Continue;
    fn parse(cmd: &str) -> Option<Self::Item> {
        Some(Continue)
    }

    fn execute(self, debugger: &mut Debugger) {
        while !debugger.should_break() {
            print!("${:04x}: ", debugger.cpu.pc);
            let instruction = debugger.cpu.execute_next();
            print!("{}\n", instruction);
        };
        let instruction = debugger.cpu.current_instruction().unwrap();
        println!("${:04x}: {}", debugger.cpu.pc, instruction);
    }
}
