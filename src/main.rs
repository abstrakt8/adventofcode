use std::cell::Cell;
use std::fs;

mod p15;
mod p15_2;

fn main() {
    let content = fs::read_to_string("15.in")
        .expect("Should have been able to read the file");
    // println!("{}", p15::run(&content));
    println!("{}", p15_2::run(&content));
}