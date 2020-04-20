use std::io;
use std::io::prelude::*;

fn compute_required_fuel(mass: f64) -> f64 {
    (mass / 3f64).floor() - 2f64
}

fn main() {
    let mut required_fuel: f64 = 0.0;
    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        let line = line.expect("Can't read stdin");
        let mass = line
            .parse::<f64>()
            .expect(&(format!("Can't parse line: \"{}\"", line)[..]));

        let mut fuel = compute_required_fuel(mass);
        while fuel > 0f64 {
            required_fuel += fuel;
            fuel = compute_required_fuel(fuel);
        }
    }
    println!("Required_fuel: {}", required_fuel);
}
