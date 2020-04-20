use std::io::prelude::*;
use std::result::Result;
use std::{error, io, str};

fn read_number_from(buf: &Vec<u8>) -> Result<u64, Box<dyn error::Error>> {
    let as_str = str::from_utf8(buf)?;
    return Ok(str::parse::<u64>(&(as_str[..as_str.len() - 1]))?);
}

fn main() {
    let stdin = io::stdin();
    let mut input: Vec<u64> = vec![];
    let mut buf: Vec<u8> = vec![];
    while stdin.lock().read_until(',' as u8, &mut buf).unwrap() > 0 {
        input.push(read_number_from(&buf).unwrap());
        buf.clear();
    }

    println!("{:?}", input);
    // let mut cursor = io::Cursor::new(input);
}
