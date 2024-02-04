use std::fs;

mod p16;
mod p16_2;
mod p17;
mod p17_2;

fn main() {
    let content = fs::read_to_string("17.in")
        .expect("Should have been able to read the file");
    println!("{}", p17::run(&content));
    println!("{}", p17_2::run(&content));
}