
use std::fs::read_to_string;
use color_eyre::Result;
use adventofcode::y24;

pub fn main() -> Result<()> {
    color_eyre::install()?;
    let str = read_to_string("./inputs/y24/1.in")?;
    println!("ANS {}", y24::p1::run(&str));

    Ok(())
}

