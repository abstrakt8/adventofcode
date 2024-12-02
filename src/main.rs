
use std::fs::read_to_string;
use color_eyre::Result;
use adventofcode::y24;

pub fn main() -> Result<()> {
    color_eyre::install()?;
    let str = read_to_string("./inputs/y24/2.in")?;
    println!("ANS {}", y24::p2::run(&str));
    let str = read_to_string("./inputs/y24/2_example.in")?;
    println!("ANS {}", y24::p2::run(&str));

    Ok(())
}

