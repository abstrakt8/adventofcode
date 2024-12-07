use std::cmp::Ordering;
use std::collections::HashMap;
use std::str::FromStr;

type Part = [i32; 4];


fn idx(s: &str) -> usize {
    match s {
        "x" => 0,
        "m" => 1,
        "a" => 2,
        "s" => 3,
        _ => panic!()
    }
}

fn parse_xmas(s: &str) -> Part {
    let mut val = [0; 4];
    s[1..s.len() - 1]
        .split(',')
        .for_each(|x| {
            let mut it = x.split('=');
            let i = idx(it.next().unwrap());
            let v = it.next().unwrap().parse::<i32>().unwrap();
            val[i] = v;
        });
    val
}

#[derive(Eq, PartialEq, Debug)]
struct Rule {
    ord: Ordering,
    xmas: usize,
    threshold: i32,
    goto: String,
}

impl Rule {
    pub fn does_apply(&self, p: &Part) -> bool {
        p[self.xmas].cmp(&self.threshold) == self.ord
    }
}

impl Workflow {
    pub fn goto(&self, p: &Part) -> &String {
        for r in &self.rules {
            if r.does_apply(p) {
                return &r.goto;
            }
        }
        &self.fallback
    }
}

#[derive(Debug, Eq, PartialEq)]
struct Workflow {
    label: String,
    fallback: String,
    rules: Vec<Rule>,
}

fn parse_ordering(s: &str) -> Ordering {
    match s {
        "<" => Ordering::Less,
        ">" => Ordering::Greater,
        _ => panic!()
    }
}

pub fn parse_rule(s: &str) -> Option<Rule> {
    let mut it = s.split(':');
    let fi = it.next().unwrap();
    if let Some(goto) = it.next() {
        let xmas = idx(&fi[..1]);
        let ordering = parse_ordering(&fi[1..2]);
        let threshold = i32::from_str(&fi[2..]).unwrap();

        Some(Rule {
            ord: ordering,
            xmas,
            threshold,
            goto: goto.to_owned(),
        })
    } else {
        None
    }
}

pub fn parse_workflow(s: &str) -> Workflow {
    let mut it = s.split("{");
    let label = it.next().unwrap();
    let mut it = it.next()
        .unwrap()
        .split(',');

    let fallback;

    let mut rules = vec![];
    loop {
        let str = it.next().unwrap();
        if let Some(r) = parse_rule(str) {
            rules.push(r);
        } else {
            fallback = str;
            break;
        }
    }
    Workflow {
        label: label.to_owned(),
        fallback: fallback[..fallback.len() - 1].to_owned(),
        rules,
    }
}

const ACCEPT_LABEL: &str = "A";
const REJECT_LABEL: &str = "R";

pub fn run(content: &str) -> i32 {
    let mut it = content.split("\n\n");
    let workflows: Vec<_> = it.next().unwrap().lines().map(parse_workflow).collect();
    let map: HashMap<String, &Workflow> = HashMap::from_iter(workflows.iter().map(|w| (w.label.to_owned(), w)));

    let mut ans = 0;
    it.next().unwrap().lines().for_each(|line| {
        let xmas = parse_xmas(line);

        let mut current = &"in".to_string();

        while current != ACCEPT_LABEL && current != REJECT_LABEL {
            let wf: &Workflow = map.get(current).unwrap();
            current = wf.goto(&xmas);
        }

        if current == ACCEPT_LABEL {
            ans += xmas.iter().sum::<i32>();
        }
    });
    ans
}

#[cfg(test)]
mod test {
    use super::{parse_rule, parse_workflow, parse_xmas, run, Rule, Workflow};
    use std::cmp::Ordering::Greater;

    #[test]
    pub fn test_example() {
        let content = r##"px{a<2006:qkq,m>2090:A,rfg}
pv{a>1716:R,A}
lnx{m>1548:A,A}
rfg{s<537:gd,x>2440:R,A}
qs{s>3448:A,lnx}
qkq{x<1416:A,crn}
crn{x>2662:A,R}
in{s<1351:px,qqz}
qqz{s>2770:qs,m<1801:hdj,R}
gd{a>3333:R,R}
hdj{m>838:A,pv}

{x=787,m=2655,a=1222,s=2876}
{x=1679,m=44,a=2067,s=496}
{x=2036,m=264,a=79,s=2244}
{x=2461,m=1339,a=466,s=291}
{x=2127,m=1623,a=2188,s=1013}"##;
        assert_eq!(run(content), 19114);
    }

    #[test]
    pub fn test_parse_xmas() {
        assert_eq!([2127, 1623, 2188, 1013], parse_xmas("{x=2127,m=1623,a=2188,s=1013}"));
        assert_eq!([2127, 1623, 2188, 1013], parse_xmas("{x=2127,m=1623,s=1013,a=2188}"));
    }

    #[test]
    pub fn test_parse_rule() {
        assert_eq!(parse_rule("m>1548:A"), Some(Rule { ord: Greater, threshold: 1548, goto: "A".to_string(), xmas: 1 }));
    }

    #[test]
    pub fn test_parse_workflow() {
        let rule = Rule { ord: Greater, threshold: 1548, goto: "A".to_string(), xmas: 1 };
        let workflow = Workflow { fallback: "A".to_string(), rules: vec![rule], label: "lnx".to_string() };

        assert_eq!(parse_workflow("lnx{m>1548:A,A}"), workflow);
    }
}