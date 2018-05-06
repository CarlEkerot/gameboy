use std::process;

use debugger::Debugger;
use debugger::commands::Command;

pub struct Exit;

impl Command for Exit {
    type Item = Exit;
    fn parse(_cmd: &str) -> Option<Self::Item> {
        Some(Exit)
    }

    fn execute(self, _debugger: &mut Debugger) {
        println!("Exiting...");
        process::exit(0);
    }
}
