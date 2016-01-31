extern crate file_logger;
#[macro_use]
extern crate log;
extern crate regex;
use self::regex::Regex;
use self::file_logger::{LoggerBuilder, OnError};

fn main() {
    let regex = Regex::new("test").unwrap();
    // LoggerBuilder::new_file("test/test.log").unwrap()
    //     .tag(regex)
    //     .level(log::LogLevelFilter::Warn)
    //     .format("[{level}] {timestamp} {module_path}:{file}:{line} - {message}".parse().unwrap())
    //     .init().unwrap();
    LoggerBuilder::from_config_str(r#"
file = "test/test.log"
tag = "test"
level = "Warn"
format = "[{level}] {timestamp} {module_path}:{file}:{line} - {message}"
"#)
        .unwrap()
        .init()
        .unwrap();
    info!("test");
    info!("test");
    for i in 0..100 {
        warn!(target: "test", "test {}", i);
    }
}
