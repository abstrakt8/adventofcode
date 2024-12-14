use nom::bytes::complete::{tag, take_till};
use nom::character::complete::char;
use nom::sequence::separated_pair;
use nom::IResult;
use std::ops::{Add, Mul};

#[derive(Hash, Eq, PartialEq, Copy, Clone, Debug)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
}

#[allow(clippy::duplicate_code)]
impl Add for Point {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}
impl<T> Mul<T> for Point
where
    T: Copy,
    i32: Mul<T, Output=i32>,
{
    type Output = Self;

    fn mul(self, rhs: T) -> Self::Output {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

impl Point {
    fn parse_point(i: &str) -> IResult<&str, Self> {
        let (i, _) = take_till(|c| c == '=')(i)?;
        let (i, _) = char('=')(i)?;
        let (i, (x, y)) = separated_pair(
            nom::character::complete::i32,
            tag(","),
            nom::character::complete::i32,
        )(i)?;

        Ok((i, Self { x, y }))
    }
}

fn parse_points(i: &str) -> IResult<&str, (Point, Point)> {
    separated_pair(Point::parse_point, tag(" "), Point::parse_point)(i)
}

fn modulo(i: i32, n: i32) -> i32 {
    ((i % n) + n) % n
}

fn parse_input(content: &str) -> Vec<(Point, Point)> {
    content
        .lines()
        .map(|line| {
            let (_, (a, v)) = crate::y24::d14::parse_points(line).expect("Expected!");
            (a, v)
        })
        .collect()
}

fn simulate(v: Vec<(Point, Point)>, seconds: i32, width: usize, height: usize) -> Vec<(Point, Point)> {
    v.into_iter()
        .map(|(a, v)| {
            let mut c = a + (v * seconds);
            c.x = modulo(c.x, width as i32);
            c.y = modulo(c.y, height as i32);
            (c, v)
        })
        .collect()
}

pub fn solve_part1(content: &str, width: usize, height: usize, seconds: i32) -> usize {
    // If we just look at x and y independently, then it makes sense to just wrap around in their respective dimensions
    let mut quadrant_count = [0; 4];
    let w_half = width / 2;
    let h_half = height / 2;

    let quadrant = |c: &Point| -> Option<usize> {
        if c.x == w_half as i32 || c.y == h_half as i32 {
            None
        } else {
            let qx = if c.x < w_half as i32 { 0 } else { 1 };
            let qy = if c.y < h_half as i32 { 0 } else { 1 };
            Some(qx * 2 + qy)
        }
    };

    let input: Vec<(Point, Point)> = parse_input(content);
    let simulated = simulate(input, seconds, width, height);
    for (point, _) in simulated {
        if let Some(q) = quadrant(&point) {
            quadrant_count[q] += 1;
        }
    }
    quadrant_count.into_iter().product()
}

pub fn solve_part2(content: &str, width: usize, height: usize) -> usize {
    let mut input: Vec<(Point, Point)> = parse_input(content);

    let draw = |v: &Vec<(Point, Point)>| -> Vec<Vec<char>> {
        let mut grid = vec![vec!['.'; width]; height];

        for (point, _) in v {
            grid[point.y as usize][point.x as usize] = '#';
        }

        grid
    };

    let might_be_tree = |v: &Vec<Vec<char>>| -> bool {
        let mut count_triangle = 0;
        for i in 0..v.len() - 1 {
            for j in 1..v[i].len() - 1 {
                count_triangle += [(i, j), (i + 1, j), (i + 1, j - 1), (i + 1, j + 1)]
                    .into_iter()
                    .all(|(i, j)| v[i][j] == '#') as usize;
            }
        }
        // maybe more conditions
        count_triangle > width
    };

    for seconds in 0..10000 {
        input = simulate(input.clone(), 1, width, height);
        let grid = draw(&input);

        if might_be_tree(&grid) {
            if true {
                println!("t={}", seconds + 1);
                for row in grid {
                    println!("{}", row.iter().collect::<String>());
                }
            }
            return seconds;
        }
    }
    panic!("Not found!");
}

pub fn run(content: &str) -> usize {
    solve_part1(content, 101, 103, 100)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    pub fn parsing() {
        let (_, (a, b)) = parse_points("p=0,4 v=3,-3").expect("A");
        assert_eq!(a, Point::new(0, 4));
        assert_eq!(b, Point::new(3, -3));
    }
    #[test]
    pub fn example() {
        let content = r"p=0,4 v=3,-3
p=6,3 v=-1,-3
p=10,3 v=-1,2
p=2,0 v=2,-1
p=0,0 v=1,3
p=3,0 v=-2,-2
p=7,6 v=-1,-3
p=3,0 v=-1,-2
p=9,3 v=2,3
p=7,3 v=-1,2
p=2,4 v=2,-3
p=9,5 v=-3,-3
";
        assert_eq!(12, solve_part1(content, 11, 7, 100));
    }
}
