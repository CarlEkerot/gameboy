use debugger::Debugger;
use debugger::commands::Command;

pub struct List;

impl Command for List {
    type Item = List;
    fn parse(_cmd: &str) -> Option<Self::Item> {
        Some(List)
    }

    fn execute(self, debugger: &mut Debugger) {
        print!("${:04x}: ", debugger.cpu.pc);
        let instruction = debugger.cpu.current_instruction().unwrap();
        print!("{}\n", instruction);
    }
}
