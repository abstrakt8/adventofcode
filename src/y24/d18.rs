use nom::bytes::complete::tag;
use nom::character::complete as ch;
use nom::character::complete::newline;
use nom::multi::separated_list1;
use nom::sequence::separated_pair;
use nom::IResult;
use std::collections::{HashMap, VecDeque};
use std::ops::Range;
use fxhash::{FxBuildHasher, FxHashSet};

#[derive(Hash, Eq, PartialEq, Clone, Debug)]
struct Point {
    x: usize,
    y: usize,
}

impl Point {
    pub fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }

    pub fn neighbours4(
        &self,
        x_range: Range<usize>,
        y_range: Range<usize>,
    ) -> impl Iterator<Item = Self> {
        let x = self.x;
        let y = self.y;

        [
            (x.checked_sub(1), Some(y)),
            (Some(x), y.checked_sub(1)),
            (x.checked_add(1), Some(y)),
            (Some(x), y.checked_add(1)),
        ]
        .into_iter()
        .filter_map(move |(x, y)| {
            let (Some(x), Some(y)) = (x, y) else {
                return None;
            };
            if x_range.contains(&x) && y_range.contains(&y) {
                Some(Point::new(x, y))
            } else {
                None
            }
        })
    }
}

// Some static parsing stuff
impl Point {
    pub fn parse(i: &str) -> IResult<&str, Self> {
        let (i, (x, y)) = separated_pair(ch::u32, tag(","), ch::u32)(i)?;
        Ok((
            i,
            Self {
                x: x as usize,
                y: y as usize,
            },
        ))
    }

    pub fn parse_points(i: &str) -> IResult<&str, Vec<Self>> {
        let (i, points) = separated_list1(newline, Self::parse)(i)?;
        Ok((i, points))
    }
}

#[derive(Default, Clone, Debug)]
struct BfsData {
    dist: HashMap<Point, u32, FxBuildHasher>,
    queue: VecDeque<Point>,
}

impl BfsData {
    pub fn check_and_push(&mut self, p: Point, d: u32) {
        if !self.dist.contains_key(&p) {
            self.dist.insert(p.clone(), d);
            self.queue.push_back(p);
        }
    }

    pub fn dist_unchecked(&self, p: &Point) -> u32 {
        *self.dist.get(p).unwrap()
    }
}

fn simulate(points: &Vec<Point>, width: usize, height: usize, take: usize) -> Option<u32> {
    let walls = points
        .iter()
        .take(take)
        .cloned()
        .collect::<FxHashSet<Point>>();

    let mut bfs = BfsData::default();
    bfs.check_and_push(Point::new(0, 0), 0);

    let end = Point::new(width - 1, height - 1);

    while let Some(front) = bfs.queue.pop_front() {
        let dist = bfs.dist_unchecked(&front);
        if front == end {
            return Some(dist);
        }

        for p in front
            .neighbours4(0..width, 0..height)
            .filter(|p| !walls.contains(p))
        {
            bfs.check_and_push(p, dist + 1);
        }
    }

    None
}

pub fn solve1(content: &str, width: usize, height: usize, take: usize) -> u32 {
    let (_, points) = Point::parse_points(content).expect("Parse points");
    simulate(&points, width, height, take).unwrap()
}

pub fn solve2(content: &str, width: usize, height: usize) -> String {
    let (_, points) = Point::parse_points(content).expect("Parse points");

    let mut range = 0..points.len() + 1;
    while !range.is_empty() {
        let mid = (range.start + range.end) / 2;
        if simulate(&points, width, height, mid).is_some() {
            range = mid + 1..range.end;
        } else {
            range = range.start..mid;
        }
    }
    // assert!(simulate(&points, width, height, range.start).is_none());
    // We take n elements, but the n-th element is n-1
    let p = &points[range.start - 1];
    format!("{},{}", p.x, p.y)
}

pub fn run(content: &str) -> String {
    // solve1(content, 71, 71, 1024).to_string()
    solve2(content, 71, 71)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn parse() {
        let (_i, actual) = Point::parse_points(
            r"5,4
4,2",
        )
        .expect("Expected");
        assert_eq!(vec![Point::new(5, 4), Point::new(4, 2)], actual);
    }

    #[test]
    fn example() {
        let content: &str = r"5,4
4,2
4,5
3,0
2,1
6,3
2,4
1,5
0,6
3,3
2,6
5,1
1,2
5,5
2,5
6,5
1,4
0,4
6,4
1,1
6,1
1,0
0,5
1,6
2,0";

        assert_eq!(22, solve1(content, 7, 7, 12));
        assert_eq!("6,1", solve2(content, 7, 7));
    }
}
