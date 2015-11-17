extern crate file_logger;
#[macro_use]
extern crate log;
extern crate regex;
use self::regex::Regex;
use self::file_logger::LoggerBuilder;

fn main() {
    let regex = Regex::new("test").unwrap();
    LoggerBuilder::new_file("./test.log").unwrap()
        .filter(regex)
        .level(log::LogLevelFilter::Warn)
        .init().unwrap();
    info!("test");
    info!("test");
    for i in 0..100 {
        warn!(target: "test", "test {}", i);
    }
}
