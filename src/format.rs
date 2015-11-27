extern crate log;
extern crate time;
use self::time::Tm;
use self::log::LogRecord;
use std::vec::Vec;
use std::io;
use error::*;

#[derive(Debug)]
pub enum FormatSpecifier {
    Str(String),
    Level,
    Timestamp(String),
    ModulePath,
    File,
    Line,
    Message,
}

impl FormatSpecifier {
    pub fn format<W: io::Write>(&self, f:&mut  W, record: &LogRecord, datetime: &Tm) -> Result<(), Error>{
        let location = record.location();
        match self {
            &FormatSpecifier::Str(ref s) => try!(write!(f, "{}", s)),
            &FormatSpecifier::Level => try!(write!(f, "{}", record.level())),
            // TODO: don't use `unwrap()`
            &FormatSpecifier::Timestamp(ref s) => try!(write!(f, "{}", try!(time::strftime(s, datetime)))),
            &FormatSpecifier::ModulePath => try!(write!(f, "{}", location.module_path())),
            &FormatSpecifier::File => try!(write!(f, "{}", location.file())),
            &FormatSpecifier::Line => try!(write!(f, "{}", location.line())),
            &FormatSpecifier::Message => try!(write!(f, "{}", record.args()))
        }
        Ok(())
    }
}

pub struct Formatter {
    f: Vec<FormatSpecifier>
}

impl Formatter {
    pub fn new() -> Self {
        Formatter {f: Vec::new()}
    }

    pub fn default() -> Self {
        let mut f = Self::new();
        f.push(FormatSpecifier::Str("[".to_string()));
        f.push(FormatSpecifier::Level);
        f.push(FormatSpecifier::Str("] ".to_string()));
        f.push(FormatSpecifier::Timestamp("%F %T%z".to_string()));
        f.push(FormatSpecifier::Str(" ".to_string()));
        f.push(FormatSpecifier::ModulePath);
        f.push(FormatSpecifier::Str(":".to_string()));
        f.push(FormatSpecifier::File);
        f.push(FormatSpecifier::Str(":".to_string()));
        f.push(FormatSpecifier::Line);
        f.push(FormatSpecifier::Str(" - ".to_string()));
        f.push(FormatSpecifier::Message);
        f
    }
    pub fn push(&mut self, f: FormatSpecifier) {
        self.f.push(f);
    }

    pub fn format<W: io::Write>(&self, mut w:  &mut W, record: &LogRecord, datetime: &Tm) -> Result<(), Error>{
        for f in &self.f {
            try!(f.format(w, record, datetime));
        };
        try!(write!(w, "\n"));
        Ok(())
    }
}
