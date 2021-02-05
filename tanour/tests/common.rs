use log::LevelFilter;
use std::fs::File;
use std::io::Read;
use tanour::types::{Address, Bytes};

pub fn start_logger() {
    simple_logger::SimpleLogger::new()
        .with_level(LevelFilter::Debug)
        .with_module_level("tanour", LevelFilter::Trace)
        .init()
        .unwrap();
}

pub fn read_file(file_path: &str) -> Bytes {
    let mut file = match File::open(file_path) {
        Ok(file) => file,
        Err(err) => panic!(err.to_string()),
    };
    let mut code = Vec::new();
    if let Err(err) = file.read_to_end(&mut code) {
        panic!(err.to_string());
    };

    code
}

pub fn address_from_hex(s : &str) -> Address{
    Address::from_slice(&hex::decode(s).unwrap())
}
