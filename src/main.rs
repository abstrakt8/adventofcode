use std::fs;

mod p12;
mod p12_2;
mod p13;

fn main() {
    let contents = fs::read_to_string("13.in")
        .expect("Should have been able to read the file");
    // println!("{}", p12::run(&contents));
    println!("{}", p13::run(&contents));
}