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
use std::io;

use format::Formatter;

pub enum OnError {
    Panic,
    Ignore,
}

pub struct Logger<W> {
    level: LogLevelFilter,
    file: Mutex<W>,
    tag: Option<Regex>,
    formatter: Formatter,
    on_error: OnError,
}


impl <W:Write+Send>Logger<W> {
    fn new(level: LogLevelFilter, tag: Option<Regex>, file: W, formatter: Formatter, on_error: OnError) -> Self {
        Logger {
            level: level,
            file: Mutex::new(file),
            tag: tag,
            formatter: formatter,
            on_error: on_error,
        }
    }
}

impl <W:Write+Send>Log for Logger<W> {
    fn enabled(&self, metadata: &LogMetadata) -> bool {
        if metadata.level() <= self.level {
            match self.tag {
                Some(ref f) => f.is_match(metadata.target()),
                None => true
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
        match self.formatter.format(&mut *file, record, &now) {
            Ok(_) => (),
            Err(_) => match self.on_error {
                OnError::Ignore => (),
                OnError::Panic => panic!("Could not write log")
            }
        }
    }
}


pub struct LoggerBuilder<W> {
    level: LogLevelFilter,
    file: W,
    tag: Option<Regex>,
    formatter: Formatter,
    on_error: OnError,
}


impl <W: 'static+Write+Send>LoggerBuilder<W> {
    pub fn file(w: W) -> Self {
        LoggerBuilder {
            level: LogLevelFilter::Off,
            file: w,
            tag: None,
            formatter: Formatter::default(),
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

    pub fn formatter(mut self, f: Formatter) -> Self {
        self.formatter = f;
        self
    }

    pub fn on_error(mut self, e: OnError) -> Self {
        self.on_error = e;
        self
    }

    pub fn build(self) -> Logger<W> {
        let LoggerBuilder{level, tag, file, formatter, on_error} = self;
        Logger::new(level, tag, file, formatter, on_error)
    }

    pub fn init(self) -> Result<(), SetLoggerError> {        
        log::set_logger(move |max_log_level| {
            max_log_level.set(self.level);
            Box::new(self.build())
        })
    }
}

impl LoggerBuilder<File> {
    pub fn new_file<P: AsRef<Path>>(p: P) -> io::Result<Self> {
        let file = try!(File::create(p));
        Ok(Self::file(file))
    }

    pub fn append_file<P: AsRef<Path>>(p: P) -> io::Result<Self> {
        let file = OpenOptions::new().write(true).create(true).append(true).open(p.as_ref());
        let file = try!(file);
        Ok(Self::file(file))
    }    
}
