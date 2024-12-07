use crate::utils::grid2d::{Direction, GridCell};
use color_eyre::owo_colors::OwoColorize;
use color_eyre::{Report, Result};
use std::str::FromStr;

struct Operation {
    dir: Direction,
    amount: i32,
}

impl Operation {}


impl FromStr for Operation {
    type Err = Report;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut it = s.split(' ');

        if false {
            let dir = it.next().ok_or_else(|| Report::msg("No direction given"))?;
            let dir = dir.chars().nth(0).ok_or_else(|| Report::msg("Empty first token"))?;
            let dir = Direction::from_letter(dir)?;

            let amount = it.next().ok_or_else(|| Report::msg("No amount given"))?;
            let amount = amount.parse::<i32>().map_err(Report::new)?;

            let color = it.next().ok_or_else(|| Report::msg("No color given"))?;
            let color = color[1..color.len() - 1].to_owned();
            Ok(Operation { dir, amount })
        } else {
            let mut it = it.skip(2);

            let color = it.next().ok_or_else(|| Report::msg("No color given"))?;
            let color = &color[2..color.len() - 1];
            let amount = i32::from_str_radix(&color[..5], 16)?;
            let mut dir = i32::from_str_radix(&color[5..], 10)?;

            // 0 means R, 1 means D, 2 means L, and 3 means U.
            let dir = match dir {
                0 => Direction::RIGHT,
                1 => Direction::DOWN,
                2 => Direction::LEFT,
                3 => Direction::UP,
                _ => Err(Report::msg("Wrong direction"))?
            };

            Ok(Operation { dir, amount })
        }
    }
}

pub fn run(content: &str) -> Result<i64> {
    let operations: Result<Vec<Operation>> = content
        .lines()
        .map(str::parse::<Operation>)
        .collect();
    // I don't know why I can't just specify .collect()?; and define the type as Vec<Operation>
    let operations = operations?;
    let mut current_cell = GridCell::new(0, 0);
    let mut points = vec![current_cell];

    // let mut grid: HashMap<GridCell, i32> = HashMap::new();
    let mut generation = 0i64;
    let (mut min_cell, mut max_cell) = (current_cell, current_cell);

    // Outer layer
    let mut perimeter_area = 1i64;
    for Operation { amount, dir, .. } in &operations {
        perimeter_area += *amount as i64;
        current_cell = current_cell.step(*dir, *amount);
        points.push(current_cell);
    }
    perimeter_area = perimeter_area / 2 + 1;

    let mut inner_area = 0i64;
    for i in 0..points.len() {
        let p = if i > 0 { points[i - 1] } else { points[points.len() - 1] };
        let q = points[i];
        inner_area += (p.x - q.x) as i64 * (p.y + q.y) as i64;
    }

    if inner_area < 0 {
        inner_area = -inner_area;
    }
    inner_area /= 2;

    println!("{inner_area} {perimeter_area}");

    Ok(inner_area + perimeter_area)
}

#[cfg(test)]
mod test {
    use super::{run, Operation};
    use crate::utils::grid2d::Direction;
    use color_eyre::*;

    pub fn setup() {
        color_eyre::install().unwrap();
    }

    #[test]
    pub fn test_parse_err() {
        setup();
        let err = "R X (#70c710)".parse::<Operation>();
        if let Err(ref e) = err {
            println!("{}", e);
        }
        assert!(err.is_err());
    }

    #[test]
    pub fn test_parse() -> Result<()> {
        let Operation { amount, dir } = "R 6 (#70c710)".parse::<Operation>()?;
        assert_eq!(dir, Direction::RIGHT);
        assert_eq!(amount, 461937);
        Ok(())
    }

    #[test]
    pub fn test_example() -> Result<()> {
        let content = r##"R 6 (#70c710)
D 5 (#0dc571)
L 2 (#5713f0)
D 2 (#d2c081)
R 2 (#59c680)
D 2 (#411b91)
L 5 (#8ceee2)
U 2 (#caa173)
L 1 (#1b58a2)
U 2 (#caa171)
R 2 (#7807d2)
U 3 (#a77fa3)
L 2 (#015232)
U 2 (#7a21e3)"##;
        assert_eq!(run(content)?, 952408144115);
        Ok(())
    }
}
