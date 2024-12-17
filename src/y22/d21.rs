use nom::{IResult, Parser};
use nom::sequence::{preceded, terminated};
use nom::character::complete::alpha1;
use nom::character::complete::char;

struct Tree {
    
}


pub enum Operation {
    
}

pub enum NodeVariant {
    Operation(Operation),
    Number(i32)
    
}

struct Node<'a> {
    id: &'a str,
}

fn parse_node<'a>(i: &'a str) -> IResult<&str, Node<'a>> {
    let (i, id) = terminated(alpha1, char(':'))(i)?;
    
    Ok((i, Node { id }))
}

#[cfg(test)]
mod tests {
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

