extern crate serde_json;

use std::fs::File;
use std::io::Read;
use std::error::Error;
use std::path::Path;
use std::str;

use serde_json::{Value};


fn main() {
    let json_buffer = load_file("./compilers/z80.json").into_vec();
    let json_str = String::from_utf8(json_buffer).unwrap_or(String::from("{}"));

    let cpu_json: Value = serde_json::from_str(&json_str).unwrap();

    println!("{}", cpu_json);
}


fn load_file(path: &str) -> Box<[u8]> {
    let fpath = Path::new(path);
    let mut buffer = Vec::new();

    let mut file = match File::open(&fpath) {
        Err(x) => panic!("Error loading the file: {}", x.description()),
        Ok(x)  => x,
    };

    file.read_to_end(&mut buffer);

    buffer.into_boxed_slice()
}

