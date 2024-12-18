use nom::branch::alt;
use nom::character::complete as ch;
use nom::character::complete::anychar;
use nom::combinator::map;
use nom::sequence::{preceded, separated_pair, terminated, tuple};
use nom::{IResult, Parser};

struct Tree {}

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
}

#[derive(Clone, Debug, PartialEq)]
pub enum NodeVariant<'a> {
    Operation(Operator, &'a str, &'a str),
    Number(u32),
}

impl<'a> NodeVariant<'a> {
    fn parse(i: &'a str) -> IResult<&'a str, Self> {
        alt((
            map(ch::u32, NodeVariant::Number),
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
