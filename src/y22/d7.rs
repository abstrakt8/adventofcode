use std::cell::RefCell;
use std::cmp::min;
use std::collections::HashMap;
use std::rc::{Rc, Weak};

#[derive(Default)]
struct Node {
    children: HashMap<String, Rc<RefCell<Node>>>,
    files: Vec<(u32, String)>,
    parent: Weak<RefCell<Node>>,
    size: u32,
}

impl Node {
    pub fn new(parent: Weak<RefCell<Node>>) -> Self {
        Self {
            parent,
            ..Default::default()
        }
    }

    pub fn calculate_size(&mut self) -> u32 {
        self.size = self.files.iter().map(|x| x.0).sum();
        self.size += self
            .children.values().map(|c| c.borrow_mut().calculate_size())
            .sum::<u32>();
        self.size
    }

    pub fn find_part1(&self) -> u32 {
        (self.size <= 100000) as u32 * self.size
            + self
                .children.values().map(|c| c.borrow().find_part1())
                .sum::<u32>()
    }
    pub fn find_part2(&self, required: u32) -> u32 {
        let this = if self.size >= required {
            self.size
        } else {
            u32::MAX
        };

        min(
            this,
            self.children.values().map(|c| c.borrow().find_part2(required))
                .min()
                .unwrap_or(u32::MAX), // Can also not have children
        )
    }
}

pub fn run(content: &str) -> (u32, u32) {
    let root = Rc::new(RefCell::new(Node::default()));
    let mut cur_ref = Rc::clone(&root);
    let lines: Vec<&str> = content.lines().skip(1).collect();

    let mut i = 0;
    while i < lines.len() {
        if lines[i].starts_with("$ cd") {
            let dir = lines[i].split_whitespace().last().unwrap().to_string();
            if dir == ".." {
                let next_ref = cur_ref.borrow().parent.upgrade().unwrap();
                cur_ref = next_ref;
            } else {
                let next_ref = cur_ref.borrow().children.get(&dir).unwrap().clone();
                cur_ref = next_ref;
            }
            // println!("cd {:}", dir);
            i += 1;
        } else if lines[i].starts_with("$ ls") {
            i += 1;
            while i < lines.len() {
                if lines[i].starts_with("$") {
                    break;
                }
                if lines[i].starts_with("dir") {
                    let (_, dir) = lines[i].split_once(" ").unwrap();
                    let parent = Rc::downgrade(&cur_ref);
                    let new_child = Node::new(parent);
                    cur_ref
                        .borrow_mut()
                        .children
                        .entry(dir.to_string())
                        .or_insert_with(|| Rc::new(RefCell::new(new_child)));
                } else {
                    let (size, name) = lines[i].split_once(" ").unwrap();
                    let size: u32 = size.parse().unwrap();
                    cur_ref.borrow_mut().files.push((size, name.to_string()));
                }
                i += 1;
            }
        }
    }
    let root_size = root.borrow_mut().calculate_size();
    let ans1 = root.borrow().find_part1();
    let required = 30000000 - (70000000 - root_size);
    println!("{required}");
    let ans2 = root.borrow().find_part2(required);

    (ans1, ans2)
}
