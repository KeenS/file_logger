extern crate log;
extern crate time;
extern crate regex;
use self::log::*;
use self::regex::Regex;
use std::io::Write;
use std::sync::Mutex;
use std::marker::Send;
use std::boxed::Box;

pub struct Logger<W> {
    level: LogLevelFilter,
    file: Mutex<W>,
    filter: Option<Regex>
}


impl <W:Write+Send>Logger<W> {
    fn new(level: LogLevelFilter, filter: Option<Regex>, file: W) -> Self {
        Logger {
            level: level,
            file: Mutex::new(file),
            filter: filter
        }
    }
}

impl <W:Write+Send>Log for Logger<W> {
    fn enabled(&self, metadata: &LogMetadata) -> bool {
        if metadata.level() <= self.level {
            match self.filter {
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
        let now  = time::strftime("%F %T%z", &time::now()).unwrap();
        let location = record.location();
        let _ = writeln!(self.file.lock().unwrap(), "[{level}] {timestamp} {module_path}:{file}:{line} - {message}",
                         level = record.level(),
                         timestamp = now,
                         module_path = location.module_path(),
                         file = location.file(),
                         line = location.line(),
                         message = record.args());
    }
}


pub struct LoggerBuilder<W> {
    level: LogLevelFilter,
    file: W,
    filter: Option<Regex>
}


impl <W: 'static+Write+Send>LoggerBuilder<W> {
    pub fn new(w: W) -> Self {
        LoggerBuilder {
            level: LogLevelFilter::Off,
            file: w,
            filter: None
        }
    }

    pub fn filter(mut self, r: Regex) -> Self {
        self.filter = Some(r);
        self
    }

    pub fn level(mut self, l: LogLevelFilter) -> Self {
        self.level = l;
        self
    }

    pub fn build(self) -> Logger<W> {
        let LoggerBuilder{level, filter, file} = self;
        Logger::new(level, filter, file)
    }

    pub fn init(self) -> Result<(), SetLoggerError> {        
        log::set_logger(move |max_log_level| {
            max_log_level.set(self.level);
            Box::new(self.build())
        })
    }
}
