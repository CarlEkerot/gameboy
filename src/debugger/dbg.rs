use std::io::stdin;
use std::fs::File;

use cpu::CPU;
use memory::Memory;
use debugger::commands::*;
use std::rc::Rc;
use std::cell::RefCell;

pub struct Debugger<'a> {
    pub cpu: CPU,
    pub rom: &'a File,
    pub breakpoints: Vec<Breakpoint>,
    pub prev_cmd: String,
}

impl<'a> Debugger<'a> {
    pub fn new(rom: &'a mut File) -> Self {
        let mem = Rc::new(RefCell::new(Memory::default()));
        let bytes_read = mem.borrow_mut().load_rom(rom).unwrap();
        println!("Loaded {} byte rom", bytes_read);

        Debugger {
            cpu: CPU::new(Rc::clone(&mem)),
            rom,
            breakpoints: vec![],
            prev_cmd: String::from("n")
        }
    }

    pub fn add_breakpoint(&mut self, b: Breakpoint) {
        self.breakpoints.push(b)
    }

    pub fn remove_breakpoints(&mut self) {
        self.breakpoints.clear()
    }

    pub fn should_break(&self) -> bool {
        self.breakpoints.iter()
            .any(|ref b| b.0 == self.cpu.pc)
    }

    fn read_input(&self) -> String {
        let mut buffer = String::new();
        stdin().read_line(&mut buffer).unwrap();
        buffer
    }

    pub fn parse_command(&mut self, cmd: &str) {
        let mut parts = cmd.split_whitespace();

        match parts.next() {
            Some("b") => {
                let i = Breakpoint::parse(cmd);
                if let Some(breakpoint) = i {
                    breakpoint.execute(self);
                };
            },
            Some("i") => {
                let i = Info::parse(cmd);
                if let Some(info) = i {
                    info.execute(self);
                };
            },
            Some("x")  => {
                let i = Print::parse(cmd);
                if let Some(print) = i {
                    print.execute(self);
                };
            },
            Some("n")  => {
                let i = Next::parse(cmd);
                if let Some(next) = i {
                    next.execute(self);
                };
            },
            Some("c")  => {
                let i = Continue::parse(cmd);
                if let Some(cont) = i {
                    cont.execute(self);
                };
            },
            Some("l")  => {
                let i = List::parse(cmd);
                if let Some(list) = i {
                    list.execute(self);
                };
            },
            Some("restart")  => {
                let i = Restart::parse(cmd);
                if let Some(restart) = i {
                    restart.execute(self);
                };
            },
            Some("exit")  => {
                let i = Exit::parse(cmd);
                if let Some(exit) = i {
                    exit.execute(self);
                };
            },
            _ => println!("Invalid command!"),
        };

        self.prev_cmd = String::from(cmd);
    }

    pub fn start(&mut self) {
        loop {
            let cmd_string = self.read_input();
            if !cmd_string.trim().is_empty() {
                self.parse_command(cmd_string.as_str());
            } else {
                let prev = self.prev_cmd.clone();
                self.parse_command(&prev);
            }
        }
    }

    pub fn restart(&mut self) {
        // TODO: Implement
    }
}
