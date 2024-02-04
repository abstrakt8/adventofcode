use std::fs;

mod p16;
mod p16_2;

fn main() {
    let content = fs::read_to_string("16.in")
        .expect("Should have been able to read the file");
    println!("{}", p16_2::run(&content));
}