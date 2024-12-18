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
            // ?
            _ => unreachable!("Invalid equation state")
        }
    }
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
            NodeVariant::Number(_) => unreachable!("Attempted to change operator of a number node"),
        }
    }
}

impl<'a> NodeVariant<'a> {
    fn parse(i: &'a str) -> IResult<&'a str, Self> {
        alt((
            // This is more resilient and does not require knowledge of the specific integer type
            map(map_res(ch::digit1, str::parse), NodeVariant::Number),
            map(
                tuple((
                    ch::alpha1,
                    ch::space1,
                    Operator::parse,
                    ch::space1,
                    ch::alpha1,
                )),
                |(lhs, _, op, _, rhs)| NodeVariant::Operation(op, lhs, rhs),
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

type CacheType<'a> = HashMap<&'a str, Option<i64>>;

struct Solver<'a> {
    map: HashMap<&'a str, Node<'a>>,
    task2: bool,
}

impl<'a> Solver<'a> {
    pub fn from(nodes: Vec<Node<'a>>, task2: bool) -> Self {
        let map = nodes
            .into_iter()
            .map(|node| (node.id, node))
            .collect::<HashMap<&str, Node>>();

        Self { map, task2 }
    }

    fn evaluate_id(&'a self, cache: &mut CacheType<'a>, id: &'a str) -> Option<i64> {
        let node = self.map.get(id).unwrap();
        self.evaluate(cache, node)
    }

    pub fn evaluate(&'a self, cache: &mut CacheType<'a>, node: &'a Node) -> Option<i64> {
        if let Some(&cached) = cache.get(node.id) {
            return cached;
        }

        if self.task2 && node.id == "humn" {
            return None;
        }

        let ans = match &node.variant {
            NodeVariant::Operation(op, a, b) => {
                let a = self.evaluate_id(cache, a);
                let b = self.evaluate_id(cache, b);
                if let (Some(a), Some(b)) = (a, b) {
                    Some(op.apply(a, b))
                } else {
                    None
                }
            }
            NodeVariant::Number(num) => Some(*num),
        };

        cache.insert(node.id, ans);

        ans
    }

    pub fn solve1(&mut self) -> i64 {
        let mut cache = Default::default();
        self.evaluate_id(&mut cache, "root").unwrap()
    }

    pub fn solve_equation(
        &'a self,
        cache: &mut CacheType<'a>,
        node: &Node<'a>,
        expected: i64,
    ) -> i64 {
        let variant = if node.id == "root" {
            &node.variant.clone().change_operator(Operator::Subtraction)
        } else {
            &node.variant
        };

        match variant {
            NodeVariant::Operation(op, id_a, id_b) => {
                let a = self.evaluate_id(cache, id_a);
                let b = self.evaluate_id(cache, id_b);
                let (x, y) = op.solve_equation(a, b, expected);

                if let Some(x) = x {
                    self.solve_equation(cache, self.map.get(id_a).unwrap(), x)
                } else if let Some(y) = y {
                    self.solve_equation(cache, self.map.get(id_b).unwrap(), y)
                } else {
                    unreachable!("Path not here")
                }
            }
            NodeVariant::Number(_) => {
                if node.id == "humn" {
                    expected
                } else {
                    unreachable!()
                }
            }
        }
    }

    pub fn solve2(&self) -> i64 {
        self.solve_equation(&mut Default::default(), self.map.get("root").unwrap(), 0)
    }
}

pub fn run(content: &str) -> (i64, i64) {
    let (_, nodes) = separated_list1(ch::newline, Node::parse_node)(content).unwrap();

    let ans1 = Solver::from(nodes.clone(), false).solve1();
    let ans2 = Solver::from(nodes, true).solve2();

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
    }
}
