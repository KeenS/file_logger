extern crate file_logger;
#[macro_use]
extern crate log;
use std::fs::File;

fn main() {
    let file = File::create("./test.log").unwrap();
    let _ = file_logger::init(None, file).unwrap();
    info!("test");
    info!("test");
    for i in 0..100 {
        warn!("test {}", i);
    }
}
