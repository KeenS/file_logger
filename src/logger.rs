extern crate toml;
extern crate log;
extern crate time;
extern crate regex;
use self::log::*;
use self::regex::Regex;
use std::io::Write;
use std::sync::Mutex;
use std::marker::Send;
use std::boxed::Box;
use std::fs::File;
use std::fs::OpenOptions;
use std::path::Path;

use format::Formatter;
use error::Error;


pub enum OnError {
    Panic,
    Ignore,
}

pub struct Logger<W> {
    level: LogLevelFilter,
    file: Mutex<W>,
    tag: Option<Regex>,
    format: Formatter,
    on_error: OnError,
}


impl <W:Write+Send>Logger<W> {
    fn new(level: LogLevelFilter,
           tag: Option<Regex>,
           file: W,
           format: Formatter,
           on_error: OnError)
           -> Self {
        Logger {
            level: level,
            file: Mutex::new(file),
            tag: tag,
            format: format,
            on_error: on_error,
        }
    }
}

impl <W:Write+Send>Log for Logger<W> {
    fn enabled(&self, metadata: &LogMetadata) -> bool {
        if metadata.level() <= self.level {
            match self.tag {
                Some(ref f) => f.is_match(metadata.target()),
                None => true,
            }
        } else {
            true
        }
    }

    fn log(&self, record: &LogRecord) {
        if !Log::enabled(self, record.metadata()) {
            return;
        }
        let mut file = self.file.lock().unwrap();
        let now = time::now();
        match self.format.format(&mut *file, record, &now) {
            Ok(_) => (),
            Err(ref e) => match self.on_error {
                OnError::Ignore => (),
                OnError::Panic => panic!("{}", e),
            },
        }
    }
}


pub struct LoggerBuilder<W> {
    level: LogLevelFilter,
    file: W,
    tag: Option<Regex>,
    format: Formatter,
    on_error: OnError,
}


impl <W: 'static+Write+Send>LoggerBuilder<W> {
    pub fn file(w: W) -> Self {
        LoggerBuilder {
            level: LogLevelFilter::Off,
            file: w,
            tag: None,
            format: Formatter::default(),
            on_error: OnError::Panic,
        }
    }

    pub fn tag(mut self, r: Regex) -> Self {
        self.tag = Some(r);
        self
    }

    pub fn level(mut self, l: LogLevelFilter) -> Self {
        self.level = l;
        self
    }

    pub fn format(mut self, f: Formatter) -> Self {
        self.format = f;
        self
    }

    pub fn on_error(mut self, e: OnError) -> Self {
        self.on_error = e;
        self
    }

    pub fn build(self) -> Logger<W> {
        let LoggerBuilder{level, tag, file, format, on_error} = self;
        Logger::new(level, tag, file, format, on_error)
    }

    pub fn init(self) -> Result<(), SetLoggerError> {
        log::set_logger(move |max_log_level| {
            max_log_level.set(self.level);
            Box::new(self.build())
        })
    }

}

impl LoggerBuilder<File> {
    pub fn new_file<P: AsRef<Path>>(p: P) -> Result<Self, Error> {
        let file = try!(File::create(p));
        Ok(Self::file(file))
    }

    pub fn append_file<P: AsRef<Path>>(p: P) -> Result<Self, Error> {
        let file = OpenOptions::new().write(true).create(true).append(true).open(p.as_ref());
        let file = try!(file);
        Ok(Self::file(file))
    }
    pub fn from_config_str(s: &str) -> Result<Self, Error> {
        let v = toml::Parser::new(s).parse().unwrap();
        Self::from_config_toml(v)
    }

    pub fn from_config_toml(v: toml::Table) -> Result<Self, Error> {
        let f = match v.get("file").and_then(|s| s.as_str()) {
            Some(s) => match v.get("append").and_then(|b| b.as_bool()) {
                Some(true) => try!(Self::append_file(s)),
                Some(false) |
                None => try!(Self::new_file(s)),
            },
            None => return Err(Error::Config),
        };
        let f = match v.get("tag").and_then(|s| s.as_str()) {
            Some(s) => {
                let r = try!(Regex::new(s).map_err(|_| Error::Config));
                f.tag(r)
            }
            None => f,
        };
        let f = match v.get("level").and_then(|s| s.as_str()) {
            Some(s) => {
                let l = match s {
                    "Off" => LogLevelFilter::Off,
                    "Error" => LogLevelFilter::Error,
                    "Warn" => LogLevelFilter::Warn,
                    "Info" => LogLevelFilter::Info,
                    "Debug" => LogLevelFilter::Debug,
                    "Trace" => LogLevelFilter::Trace,
                    _ => return Err(Error::Config),
                };
                f.level(l)
            }
            None => f,
        };
        let f = match v.get("format").and_then(|s| s.as_str()) {
            Some(s) => f.format(try!(s.parse())),
            None => f,
        };
        Ok(f)
    }
}
