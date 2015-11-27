extern crate log;
extern crate time;
use self::log::LogRecord;
use std::vec::Vec;
use std::io::Write;
use std::io::Error;

#[derive(Debug)]
pub enum FormatterEnum {
    Str(String),
    Level,
    Timestamp(String),
    ModulePath,
    File,
    Line,
    Message,
}

impl FormatterEnum {
    pub fn format<W: Write>(&self, f:&mut  W, record: &LogRecord) -> Result<(), Error>{
        let location = record.location();
        match self {
            &FormatterEnum::Str(ref s) => write!(f, "{}", s),
            &FormatterEnum::Level => write!(f, "{}", record.level()),
            &FormatterEnum::Timestamp(ref s) => write!(f, "{}", time::strftime(s, &time::now()).unwrap()),
            &FormatterEnum::ModulePath => write!(f, "{}", location.module_path()),
            &FormatterEnum::File => write!(f, "{}", location.file()),
            &FormatterEnum::Line => write!(f, "{}", location.line()),
            &FormatterEnum::Message => write!(f, "{}", record.args())
        }
    }
}

pub struct Formatter {
    f: Vec<FormatterEnum>
}

impl Formatter {
    pub fn new() -> Self {
        Formatter {f: Vec::new()}
    }

    pub fn default() -> Self {
        let mut f = Self::new();
        f.push(FormatterEnum::Str("[".to_string()));
        f.push(FormatterEnum::Level);
        f.push(FormatterEnum::Str("] ".to_string()));
        f.push(FormatterEnum::Timestamp("%F %T%z".to_string()));
        f.push(FormatterEnum::Str(" ".to_string()));
        f.push(FormatterEnum::ModulePath);
        f.push(FormatterEnum::Str(":".to_string()));
        f.push(FormatterEnum::File);
        f.push(FormatterEnum::Str(":".to_string()));
        f.push(FormatterEnum::Line);
        f.push(FormatterEnum::Str(" - ".to_string()));
        f.push(FormatterEnum::Message);
        f
    }
    pub fn push(&mut self, f: FormatterEnum) {
        self.f.push(f);
    }

    pub fn format<W:Write>(&self, mut w:  &mut W, record: &LogRecord) -> Result<(), Error>{
        let v: &[FormatterEnum] = &self.f;
        for f in v {
            try!(f.format(w, record));
        };
        write!(w, "\n")
    }
}
