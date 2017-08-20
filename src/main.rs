extern crate serde_json;

use std::fs::File;
use std::io::Read;
use std::fmt::Write;
use std::error::Error;
use std::path::Path;
use std::str;

use serde_json::{Value};


fn main() {
    let json_buffer = load_file("./arch/z80.json").into_vec();
    let json_str = String::from_utf8(json_buffer).unwrap_or(String::from("{}"));

    let mut s = String::new();

    for &byte in load_file("./bin/bootstrap.bin").iter() {
        write!(&mut s, "{} ", format!("{:01$X}", byte, 2)).expect("Unable to write");
    }

    let cpu_json: Value = serde_json::from_str(&json_str).unwrap();

    println!("{}", s);
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
