extern crate log;
extern crate time;
use self::log::*;
use std::fs::File;
use std::io::Write;
use std::sync::Mutex;
use std::marker::Send;

pub struct Logger<W> {
    level: Option<LogLevel>,
    file: Mutex<W>
}

impl <W:Write+Send>Logger<W> {
    fn new(level: Option<LogLevel>, file: W) -> Self {
        Logger {
            level: level,
            file: Mutex::new(file)
        }
    }
}

impl <W:Write+Send>Log for Logger<W> {
    fn enabled(&self, metadata: &LogMetadata) -> bool {
        match self.level {
            Some(l) => metadata.level() <= l,
            None => true
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

pub fn init<W:'static + Write+Send>(level: Option<LogLevel>, file: W) -> Result<(), SetLoggerError> {
    log::set_logger(move |max_log_level| {
        max_log_level.set(LogLevelFilter::Trace);
        Box::new(Logger::new(level, file))
    })
}
