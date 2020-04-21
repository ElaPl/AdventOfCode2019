use std::io::prelude::*;
use std::result::Result;
use std::{error, io, str};

fn read_number_from(buf: &Vec<u8>) -> Result<usize, Box<dyn error::Error>> {
    let as_str = str::from_utf8(buf)?;
    return Ok(str::parse::<usize>(&(as_str[..as_str.len() - 1]))?);
}

fn run_intcode_program(buf: &mut Vec<usize>) -> Option<&Vec<usize>> {
    let mut iterator = 0;
    while iterator + 3 < buf.len() {
        let (l, r, out_position) = (buf[iterator + 1], buf[iterator + 2], buf[iterator + 3]);

        if out_position >= buf.len() {
            println!(
                "Output position : {} at idx {} is to large",
                out_position,
                iterator + 3
            );
            return Option::None;
        }

        match buf[iterator] {
            1 => {
                buf[out_position] = buf[l] + buf[r];
            }
            2 => buf[out_position] = buf[l] * buf[r],
            99 => break,
            _ => {
                println!(
                    "Unknown operation number: {} at position: {}",
                    buf[iterator], iterator
                );
                return Option::None;
            }
        }

        iterator += 4;
    }

    Option::Some(buf)
}

fn main() {
    let stdin = io::stdin();
    let mut input: Vec<usize> = vec![];
    let mut buf: Vec<u8> = vec![];
    while stdin.lock().read_until(',' as u8, &mut buf).unwrap() > 0 {
        input.push(read_number_from(&buf).unwrap());
        buf.clear();
    }

    input[1] = 12;
    input[2] = 2;
    if let Some(result) = run_intcode_program(&mut input) {
        println!("{:?}", result);
    }
}
