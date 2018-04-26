use std::process;

use debugger::Debugger;
use debugger::commands::Command;

pub struct Exit;

impl Command for Exit {
    type Item = Exit;
    fn parse(cmd: &str) -> Option<Self::Item> {
        Some(Exit)
    }

    fn execute(self, debugger: &mut Debugger) {
        println!("Exiting...");
        process::exit(0);
    }
}
