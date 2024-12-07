use crate::utils::dict::Dictionary;
use crate::utils::graph::FlexibleGraph;
use crate::utils::vec::ElasticVector;
use std::collections::{HashMap, VecDeque};

const N: u32 = 1000;

// Low = false, High = true
type Pulse = bool;

#[derive(Default)]
struct Memory {
    map: HashMap<usize, Pulse>,
    num_high: usize,
}

impl Memory {
    pub fn receive_pulse(&mut self, i: usize, pulse: bool) {
        if pulse {
            self.num_high += 1;
        }
        if let Some(true) = self.map.insert(i, pulse) {
            self.num_high -= 1;
        }
    }

    pub fn determine_pulse_to_send(&self) -> bool {
        self.num_high != self.map.len()
    }
}

#[derive(Default)]
enum ModuleType {
    CONJ(Memory),
    FLIP(bool),

    #[default]
    NONE,
}


pub fn run(content: &str) -> u32 {
    let dict = Dictionary::new();
    let mut graph = FlexibleGraph::new();
    let mut modules = ElasticVector::new();

    let mut broadcast_id = 0;

    content.lines().for_each(|x| {
        let mut it = x.split(" -> ");
        let token = it.next().unwrap();
        let id;
        if token == "broadcaster" {
            id = dict.id(token);
            broadcast_id = id;
        } else {
            id = dict.id(&token[1..]);
            modules.set(id, match &token[..1] {
                "&" => ModuleType::CONJ(Memory::default()),
                "%" => ModuleType::FLIP(false),
                _ => panic!("Should not happen even though we have NONE type")
            });
        }

        let token = it.next().unwrap();
        token.split(", ").for_each(|to| {
            graph.add_directed(id, dict.id(to));
        });
    });

    // Ensure for all
    modules.set(graph.number_of_vertices(), Default::default());
    for u in 0..graph.number_of_vertices() {
        for v in &graph.neighbors[u] {
            if let ModuleType::CONJ(ref mut mem) = modules.get_mut(*v) {
                mem.receive_pulse(u, false);
            }
        }
    }

    let rx = dict.id("rx");

    println!("N={}", graph.number_of_vertices());
    for u in 0..graph.number_of_vertices() {
        print!("u={u} ");
        match modules.get(u) {
            ModuleType::NONE => println!("NONE"),
            ModuleType::CONJ(mem) => println!("MEM {}", mem.map.len()),
            ModuleType::FLIP(_) => println!("FLIP"),
        }
    }
    let mut pulse_count = [0; 2];
    let mut it = 0;
    loop {
        it += 1;
        // println!("it={}", it);
        let mut q = VecDeque::new();
        q.push_back((broadcast_id, false));

        // The low pulse to broadcast
        pulse_count[0] += 1;

        let mut done = false;
        while let Some((from, pulse)) = q.pop_front() {
            for &to in &graph.neighbors[from] {
                pulse_count[if pulse { 1 } else { 0 }] += 1;
                match modules.get_mut(to) {
                    ModuleType::CONJ(ref mut mem) => {
                        mem.receive_pulse(from, pulse);
                        q.push_back((to, mem.determine_pulse_to_send()));
                    }
                    ModuleType::FLIP(ref mut state) => {
                        // Only low pulses will trigger a flip
                        if pulse == false {
                            *state = !*state;
                            q.push_back((to, *state));
                        }
                    }
                    ModuleType::NONE => {
                        done |= to == rx && pulse == false;
                    }
                }
            }
        }
        if done {
            break;
        }
    }
    it
}

#[cfg(test)]
mod test {
    use super::run;
    use std::fs::read_to_string;

    #[test]
    pub fn test_example1() {
        let content = r##"broadcaster -> a, b, c
%a -> b
%b -> c
%c -> inv
&inv -> a"##;

        assert_eq!(run(content), 32000000);
    }

    #[test]
    pub fn test_example2() {
        let content = r##"broadcaster -> a
%a -> inv, con
&inv -> b
%b -> con
&con -> output"##;
        assert_eq!(run(content), 11687500);
    }

    #[ignore] // Too slow for automated tests
    #[test]
    pub fn test_input() -> color_eyre::Result<()> {
        let str = read_to_string("../../inputs/y23/20.in")?;
        println!("{}", run(&str));
        Ok(())
    }
}