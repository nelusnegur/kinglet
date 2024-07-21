use core::fmt::Arguments;
use core::fmt::Write;

pub struct Console<W: Write> {
    writer: W,
}

impl<W: Write> Console<W> {
    pub fn new(writer: W) -> Console<W> {
        Self { writer }
    }

    pub fn print(&mut self, args: Arguments) {
        self.writer.write_fmt(args).unwrap()
    }

    pub fn print_str(&mut self, str: &str) {
        self.writer.write_str(str).unwrap()
    }
}
