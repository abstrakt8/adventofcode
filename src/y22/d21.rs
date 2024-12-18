use nom::branch::alt;
use nom::character::complete as ch;
use nom::character::complete::anychar;
use nom::combinator::{map, map_res};
use nom::multi::separated_list1;
use nom::sequence::{preceded, terminated, tuple};
use nom::IResult;
use std::collections::HashMap;

#[derive(Clone, Debug, PartialEq)]
pub enum Operator {
    Addition,
    Subtraction,
    Multiplication,
    Division,
}

impl Operator {
    fn parse(i: &str) -> IResult<&str, Self> {
        let (i, c) = anychar(i)?;
        let op = match c {
            '+' => Operator::Addition,
            '-' => Operator::Subtraction,
            '*' => Operator::Multiplication,
            '/' => Operator::Division,
            _ => {
                return Err(nom::Err::Error(nom::error::Error::new(
                    i,
                    nom::error::ErrorKind::Char,
                )))
            }
        };
        Ok((i, op))
    }

    fn apply(&self, a: i64, b: i64) -> i64 {
        match self {
            Operator::Addition => a + b,
            Operator::Subtraction => a - b,
            Operator::Multiplication => a * b,
            Operator::Division => a / b,
        }
    }

    pub(crate) fn solve_equation(
        &self,
        x: Option<i64>,
        y: Option<i64>,
        expected: i64,
    ) -> (Option<i64>, Option<i64>) {
        match (self, x, y) {
            // x + y = expected
            (Operator::Addition, None, Some(y)) => (Some(expected - y), None),
            (Operator::Addition, Some(x), None) => (None, Some(expected - x)),
            // x - y = expected
            (Operator::Subtraction, None, Some(y)) => (Some(expected + y), None),
            (Operator::Subtraction, Some(x), None) => (None, Some(x - expected)),
            // x * y = expected
            (Operator::Multiplication, None, Some(y)) => (Some(expected / y), None),
            (Operator::Multiplication, Some(x), None) => (None, Some(expected / x)),
            // x / y = expected
            (Operator::Division, None, Some(y)) => (Some(expected * y), None),
            (Operator::Division, Some(x), None) => (None, Some(x / expected)),
            _ => unreachable!("Should not happen"),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum NodeVariant<'a> {
    Operation(Operator, &'a str, &'a str),
    Number(i64),
}

impl NodeVariant<'_> {
    pub fn change_operator(self, op: Operator) -> Self {
        match self {
            NodeVariant::Operation(_, a, b) => NodeVariant::Operation(op, a, b),
            NodeVariant::Number(_) => unreachable!("Not possible on this"),
        }
    }
}

impl<'a> NodeVariant<'a> {
    fn parse(i: &'a str) -> IResult<&'a str, Self> {
        alt((
            map(map_res(ch::digit1, str::parse), NodeVariant::Number),
            map(
                tuple((
                    ch::alpha1,
                    ch::space1,
                    Operator::parse,
                    ch::space1,
                    ch::alpha1,
                )),
                |(a, _, op, _, b)| NodeVariant::Operation(op, a, b),
            ),
        ))(i)
    }
}

#[derive(Debug, Clone, PartialEq)]
struct Node<'a> {
    id: &'a str,
    variant: NodeVariant<'a>,
}

impl<'a> Node<'a> {
    fn parse_node(i: &'a str) -> IResult<&'a str, Self> {
        map(
            tuple((
                terminated(ch::alpha1, ch::char(':')),
                preceded(ch::space1, NodeVariant::parse),
            )),
            |(id, variant)| Node { id, variant },
        )(i)
    }
}

struct Solver<'a> {
    map: HashMap<&'a str, Node<'a>>,
    task2: bool,
    cache: HashMap<&'a str, Option<i64>>,
}

impl<'a> Solver<'a> {
    pub fn from(nodes: Vec<Node<'a>>, task2: bool) -> Self {
        let map = nodes
            .into_iter()
            .map(|node| (node.id, node))
            .collect::<HashMap<&str, Node>>();

        Self {
            map,
            task2,
            cache: HashMap::new(),
        }
    }

    fn evaluate_id(&mut self, id: &'a str) -> Option<i64> {
        // Borrow node from map
        let node = self.map.get(id).unwrap();

        if let Some(&cached) = self.cache.get(node.id) {
            return cached;
        }

        // Handle special case
        if self.task2 && node.id == "humn" {
            self.cache.insert(node.id, None);
            return None;
        }

        match &node.variant {
            NodeVariant::Operation(op, a, b) => {
                let op = op.clone();
                let a_str = *a;
                let b_str = *b;
                let node_id = node.id;

                // Drop the reference to node before recursive calls
                let a_val = self.evaluate_id(a_str)?;
                let b_val = self.evaluate_id(b_str)?;
                let ans = Some(op.apply(a_val, b_val));
                self.cache.insert(node_id, ans);
                ans
            }
            NodeVariant::Number(num) => {
                // Drop node
                let ans = Some(*num);
                self.cache.insert(node.id, ans);
                ans
            }
        }
    }

    pub fn solve1(&mut self) -> i64 {
        self.evaluate_id("root").unwrap()
    }

    pub fn solve_equation(&mut self, node_id: &'a str, expected: i64) -> i64 {
        let node = self.map.get(node_id).unwrap();

        // Extract needed fields
        let is_root = node.id == "root";
        let variant = if is_root {
            node.variant.clone().change_operator(Operator::Subtraction)
        } else {
            node.variant.clone()
        };

        // Dropping the borrow here so we don't have an immutable reference
        let _ = node;

        match variant {
            NodeVariant::Operation(op, a, b) => {
                let a_val = self.evaluate_id(a);
                let b_val = self.evaluate_id(b);
                let (x, y) = op.solve_equation(a_val, b_val, expected);

                if let Some(x) = x {
                    self.solve_equation(a, x)
                } else if let Some(y) = y {
                    self.solve_equation(b, y)
                } else {
                    unreachable!("Path not here")
                }
            }
            NodeVariant::Number(_) => {
                if node_id == "humn" {
                    expected
                } else {
                    unreachable!()
                }
            }
        }
    }

    pub fn solve2(&mut self) -> i64 {
        self.solve_equation("root", 0)
    }
}

pub fn run(content: &str) -> (i64, i64) {
    let (_, nodes) = separated_list1(ch::newline, Node::parse_node)(content).unwrap();

    let mut solver1 = Solver::from(nodes.clone(), false);
    let ans1 = solver1.solve1();

    let mut solver2 = Solver::from(nodes, true);
    let ans2 = solver2.solve2();

    (ans1, ans2)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn parse_node() {
        let (_i, node) = Node::parse_node("root: pppw + sjmn").unwrap();
        assert_eq!(
            Node {
                id: "root",
                variant: NodeVariant::Operation(Operator::Addition, "pppw", "sjmn")
            },
            node
        );
    }

    #[test]
    pub fn parse_node_number() {
        let (_i, node) = Node::parse_node("zczc: 2").unwrap();
        assert_eq!(
            Node {
                id: "zczc",
                variant: NodeVariant::Number(2)
            },
            node
        );
    }

    #[test]
    pub fn example() {
        let content = r"root: pppw + sjmn
dbpl: 5
cczh: sllz + lgvd
zczc: 2
ptdq: humn - dvpt
dvpt: 3
lfqf: 4
humn: 5
ljgn: 2
sjmn: drzm * dbpl
sllz: 4
pppw: cczh / lfqf
lgvd: ljgn * ptdq
drzm: hmdt - zczc
hmdt: 32
";
        let (ans1, ans2) = run(content);
        assert_eq!(ans1, 152);
        assert_eq!(ans2, 301);
    }
}