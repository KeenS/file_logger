extern crate log;
extern crate time;
extern crate nom;
use self::time::Tm;
use self::log::LogRecord;
use self::nom::IResult;
use std::vec::Vec;
use std::io;
use std::str::FromStr;
use std::str::from_utf8;
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
    pub fn format<W: io::Write>(&self,
                                f: &mut W,
                                record: &LogRecord,
                                datetime: &Tm)
                                -> Result<(), Error> {
        let location = record.location();
        match self {
            &FormatSpecifier::Str(ref s) => try!(write!(f, "{}", s)),
            &FormatSpecifier::Level => try!(write!(f, "{}", record.level())),
            // TODO: don't use `unwrap()`
            &FormatSpecifier::Timestamp(ref s) =>
                try!(write!(f, "{}", try!(time::strftime(s, datetime)))),
            &FormatSpecifier::ModulePath => try!(write!(f, "{}", location.module_path())),
            &FormatSpecifier::File => try!(write!(f, "{}", location.file())),
            &FormatSpecifier::Line => try!(write!(f, "{}", location.line())),
            &FormatSpecifier::Message => try!(write!(f, "{}", record.args())),
        }
        Ok(())
    }
}

pub struct Formatter {
    f: Vec<FormatSpecifier>,
}

impl Formatter {
    pub fn new() -> Self {
        Formatter { f: Vec::new() }
    }

    pub fn default() -> Self {
        let mut f = Self::new();
        f.push(FormatSpecifier::Str("[".to_owned()));
        f.push(FormatSpecifier::Level);
        f.push(FormatSpecifier::Str("] ".to_owned()));
        f.push(FormatSpecifier::Timestamp("%F %T%z".to_owned()));
        f.push(FormatSpecifier::Str(" ".to_owned()));
        f.push(FormatSpecifier::ModulePath);
        f.push(FormatSpecifier::Str(":".to_owned()));
        f.push(FormatSpecifier::File);
        f.push(FormatSpecifier::Str(":".to_owned()));
        f.push(FormatSpecifier::Line);
        f.push(FormatSpecifier::Str(" - ".to_owned()));
        f.push(FormatSpecifier::Message);
        f
    }
    pub fn push(&mut self, f: FormatSpecifier) {
        self.f.push(f);
    }

    pub fn format<W: io::Write>(&self,
                                mut w: &mut W,
                                record: &LogRecord,
                                datetime: &Tm)
                                -> Result<(), Error> {
        for f in &self.f {
            try!(f.format(w, record, datetime));
        }
        try!(write!(w, "\n"));
        Ok(())
    }
}


named!(parse_str < Vec<FormatSpecifier> >, many0!(alt!(_str|_tag)));
named!(_str <FormatSpecifier>, map_res!(many1!(is_not!("{")), |vs| {
    let mut vec = Vec::new();
    for v in vs {
        vec.extend(v);
    }
    from_utf8(&vec).map(|s| FormatSpecifier::Str(s.to_string()))   
}));
named!(_tag <FormatSpecifier>,
       delimited!(char!('{'),
                  alt!(
                      complete!(tag!("level"))       => {|_| FormatSpecifier::Level} |
                      complete!(tag!("message"))     => {|_| FormatSpecifier::Message} |
                      complete!(tag!("file"))        => {|_| FormatSpecifier::File} |
                      complete!(tag!("module_path")) => {|_| FormatSpecifier::ModulePath} |
                      complete!(tag!("line"))        => {|_| FormatSpecifier::Line} |
                      complete!(tag!("timestamp"))   => {|_| FormatSpecifier::Timestamp("%F %T%z".to_string())}),
                  char!('}')));

impl FromStr for Formatter {
    type Err = FormatError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match parse_str(s.as_bytes()) {
            IResult::Done(rest, v) => {
                let len = rest.len();
                if len == 0 {
                    Ok(Formatter { f: v })
                } else {
                    println!("{}", from_utf8(rest).unwrap());
                    Err(FormatError {
                        format: s.to_string(),
                        position: s.len() - len,
                    })
                }
            }
            IResult::Incomplete(_) => Err(FormatError {
                format: s.to_string(),
                position: 0,
            }),
            IResult::Error(_) => Err(FormatError {
                format: s.to_string(),
                position: 1,
            }),
        }
    }
}
