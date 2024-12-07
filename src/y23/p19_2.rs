use itertools;
use itertools::Itertools;
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
    goto_idx: usize,
}

impl Rule {
    pub fn does_apply(&self, p: &Part) -> bool {
        p[self.xmas].cmp(&self.threshold) == self.ord
    }
}

impl Workflow {
    pub fn goto(&self, p: &Part) -> usize {
        for r in &self.rules {
            if r.does_apply(p) {
                return r.goto_idx;
            }
        }
        self.fallback_idx
    }
}

#[derive(Debug, Eq, PartialEq)]
struct Workflow {
    label: String,
    fallback: String,
    fallback_idx: usize,
    rules: Vec<Rule>,
}

#[derive(Debug, Eq, PartialEq)]
struct OptimisedWorkflow {
    fallback: usize,
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
            goto_idx: 0,
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

        fallback_idx: 0,
    }
}

const ACCEPT_LABEL: &str = "A";
const REJECT_LABEL: &str = "R";


pub fn run(content: &str) -> i64 {
    let mut it = content.split("\n\n");
    let mut workflows: Vec<_> = it.next().unwrap().lines().map(parse_workflow).collect();

    let mut workflow_idx: HashMap<String, usize> = HashMap::from_iter(
        workflows.iter().
            enumerate()
            .map(|(i, w)| (w.label.to_owned(), i))
    );
    let (accept_idx, reject_idx) = (workflows.len(), workflows.len() + 1);

    workflow_idx.insert(ACCEPT_LABEL.to_string(), accept_idx);
    workflow_idx.insert(REJECT_LABEL.to_string(), reject_idx);

    for workflow in workflows.iter_mut() {
        workflow.fallback_idx = *workflow_idx.get(&workflow.fallback).unwrap();
        for rule in workflow.rules.iter_mut() {
            rule.goto_idx = *workflow_idx.get(&rule.goto).unwrap();
        }
    }

    let mut ans = 0i64;

    let starting_idx = *workflow_idx.get(&"in".to_string()).unwrap();
    let check = |xmas: &Part| {
        let mut steps = 0;
        let mut current = starting_idx;
        while current != accept_idx && current != reject_idx {
            current = workflows[current].goto(xmas);
        }
        current == accept_idx
    };

    let mut cand = vec![vec![1, 4001]; 4];
    for workflow in &workflows {
        for rule in &workflow.rules {
            // x < 5 => insert(5)
            // x > 5 => insert(6)
            let z = (rule.threshold) + (if rule.ord == Ordering::Greater { 1 } else { 0 });
            cand[rule.xmas].push(z);
        }
    }
    for c in cand.iter_mut() {
        c.sort();
        c.dedup();
        println!("{}", c.len());
    }
    for xw in cand[0].windows(2) {
        let x1 = xw[0];
        let x2 = xw[1];

        for mw in cand[1].windows(2) {
            let m1 = mw[0];
            let m2 = mw[1];

            println!("x1={x1} m1={m1}");
            for aw in cand[2].windows(2) {
                let a1 = aw[0];
                let a2 = aw[1];

                for sw in cand[3].windows(2) {
                    let s1 = sw[0];
                    let s2 = sw[1];

                    let part: Part = [x1, m1, a1, s1];
                    if check(&part) {
                        ans += ((x2 - x1) as i64) * ((m2 - m1) as i64) * ((a2 - a1) as i64) * ((s2 - s1) as i64);
                    }
                    // println!("{x1} {m1} {a1} {s1}");
                }
            }
        }
    }


    ans
}

#[cfg(test)]
mod test {
    use super::{parse_xmas, run};

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
        assert_eq!(run(content), 167409079868000);
    }

    #[test]
    pub fn test_parse_xmas() {
        assert_eq!([2127, 1623, 2188, 1013], parse_xmas("{x=2127,m=1623,a=2188,s=1013}"));
        assert_eq!([2127, 1623, 2188, 1013], parse_xmas("{x=2127,m=1623,s=1013,a=2188}"));
    }

    // #[test]
    // pub fn test_parse_rule() {
    //     assert_eq!(parse_rule("m>1548:A"), Some(Rule { ord: Greater, threshold: 1548, goto: "A".to_string(), xmas: 1 }));
    // }
    //
    // #[test]
    // pub fn test_parse_workflow() {
    //     let rule = Rule { ord: Greater, threshold: 1548, goto: "A".to_string(), xmas: 1 };
    //     let workflow = Workflow { fallback: "A".to_string(), rules: vec![rule], label: "lnx".to_string() };
    //
    //     assert_eq!(parse_workflow("lnx{m>1548:A,A}"), workflow);
    // }
}