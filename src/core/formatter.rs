//! Formatter core functionality


/// An output for formatter.

pub trait FormatterOutput {
    fn write_str(&mut self, s: &str);
    fn write_fmt(&mut self, args: core::fmt::Arguments<'_>);
}

impl FormatterOutput for String {
    fn write_str(&mut self, s: &str) {
        self.push_str(s);
    }

    fn write_fmt(&mut self, args: core::fmt::Arguments<'_>) {
        core::fmt::write(self, args).unwrap();
    }
}

