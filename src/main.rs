use adventofcode::{y22, y24};
use color_eyre::Result;
use std::fs::read_to_string;

const YEAR: u32 = 24;
const DAY: u32 = 9;

pub fn main() -> Result<()> {
    color_eyre::install()?;
    let suffixes = [
        "_example.in", 
        ".in"
    ];
    suffixes.iter().for_each(|&suffix| {
        let file_name = format!("./inputs/y{YEAR}/{DAY}{suffix}");
        let input = read_to_string(&file_name);
        if let Ok(input) = input {
            let ans = y24::d9::run(&input);
            println!("{file_name}: {:?}", ans);
        } else {
            println!("Skipped {file_name}");
        }
    });

    Ok(())
}
