use debugger::Debugger;
use debugger::commands::Command;

pub struct Restart;

impl Command for Restart {
    type Item = Restart;
    fn parse(_cmd: &str) -> Option<Self::Item> {
        Some(Restart)
    }

    fn execute(self, debugger: &mut Debugger) {
        println!("Restarting...");
        debugger.cpu.reset();
    }
}
