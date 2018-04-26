mod breakpoint;
mod cont;
mod exit;
mod info;
mod list;
mod next;
mod print;
mod restart;

pub use self::breakpoint::Breakpoint;
pub use self::cont::Continue;
pub use self::exit::Exit;
pub use self::info::Info;
pub use self::list::List;
pub use self::next::Next;
pub use self::print::Print;
pub use self::restart::Restart;

use debugger::Debugger;

pub trait Command {
    type Item;
    fn parse(input: &str) -> Option<Self::Item>;
    fn execute(self, debugger: &mut Debugger);

    fn parse_number(s: &str) -> Option<u64> {
        if s.starts_with("0x") {
            u64::from_str_radix(&s[2..], 16).ok()
        } else {
            s.parse::<u64>().ok()
        }
    }
}
