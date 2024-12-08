use num::integer::gcd_lcm;

#[derive(Debug, Clone)]
enum Operation {
    Square,
    Plus(u64),
    Mul(u64),
}

#[derive(Debug, Clone)]
struct Monkey {
    items: Vec<u64>,
    operation: Operation,
    divisible: u64,
    true_throw: usize,
    false_throw: usize,
    operation_count: usize,
}

impl Monkey {
    fn apply_operation(&self, x: u64) -> u64 {
        match self.operation {
            Operation::Square => x * x,
            Operation::Plus(y) => x + y,
            Operation::Mul(y) => x * y,
        }
    }
    fn increase_count(&mut self, x: usize) {
        self.operation_count += x;
    }

    fn next_monkey(&self, value: u64) -> usize {
        if value % self.divisible == 0 {
            self.true_throw
        } else {
            self.false_throw
        }
    }
}

impl From<&str> for Monkey {
    fn from(value: &str) -> Self {
        let mut it = value.lines().skip(1);
        let items = it
            .next()
            .unwrap()
            .split_once(":")
            .unwrap()
            .1
            .split(",")
            .map(|x| x.trim().parse().unwrap())
            .collect(); // interesting..

        let operation = {
            let s = it.next().unwrap().split_once("=").unwrap().1.trim();
            if s.starts_with("old * old") {
                Operation::Square
            } else if s.starts_with("old * ") {
                let u: u64 = s.rsplit_once("*").unwrap().1.trim().parse().unwrap();
                Operation::Mul(u)
            } else {
                let u: u64 = s.rsplit_once("+").unwrap().1.trim().parse().unwrap();
                Operation::Plus(u)
            }
        };

        let divisible: u64 = it
            .next()
            .unwrap()
            .rsplit_once(' ')
            .unwrap()
            .1
            .parse()
            .unwrap();

        let true_throw: usize = it
            .next()
            .unwrap()
            .rsplit_once(' ')
            .unwrap()
            .1
            .parse()
            .unwrap();

        let false_throw: usize = it
            .next()
            .unwrap()
            .rsplit_once(' ')
            .unwrap()
            .1
            .parse()
            .unwrap();

        Self {
            items,
            operation,
            divisible,
            true_throw,
            false_throw,
            operation_count: 0,
        }
    }
}

fn simulate2(mut monkeys: Vec<Monkey>) -> Vec<Monkey> {
    let lcm = monkeys
        .iter()
        .fold(1u64, |lcm, m| gcd_lcm(lcm, m.divisible).1);

    for _ in 0..10000 {
        // for monkey in monkeys.iter_mut() {
        for i in 0..monkeys.len() {
            let (lhs, rhs) = monkeys.split_at_mut(i);
            let (current, rhs) = rhs.split_first_mut().unwrap();
            current.increase_count(current.items.len());
            for item in &current.items {
                let value = current.apply_operation(*item) % lcm;
                let j = current.next_monkey(value);
                if j > i {
                    rhs[j - i - 1].items.push(value);
                } else {
                    lhs[j].items.push(value);
                }
            }
            current.items.drain(..);
        }
    }
    monkeys
}

fn simulate1(mut monkeys: Vec<Monkey>) -> Vec<Monkey> {
    for _ in 0..20 {
        for i in 0..monkeys.len() {
            let (lhs, rhs) = monkeys.split_at_mut(i);
            let (current, rhs) = rhs.split_first_mut().unwrap();
            current.increase_count(current.items.len());
            for item in &current.items {
                let value = current.apply_operation(*item) / 3;
                let j = current.next_monkey(value);
                if j > i {
                    rhs[j - i - 1].items.push(value);
                } else {
                    lhs[j].items.push(value);
                }
            }
            current.items.drain(..);
        }
    }
    monkeys
}

fn monkey_business(mut monkeys: Vec<Monkey>) -> usize {
    monkeys.sort_by(|a, b| b.operation_count.cmp(&a.operation_count));
    monkeys[0..=1].iter().map(|x| x.operation_count).product()
}

pub fn run(content: &str) -> (usize, usize) {
    let monkeys: Vec<Monkey> = content.split("\n\n").map(Monkey::from).collect();
    // println!("{:?}", monkeys);

    let ans1 = monkey_business(simulate1(monkeys.clone()));
    let ans2 = monkey_business(simulate2(monkeys.clone()));

    (ans1, ans2)
}
