use regex::Regex;
use std::cmp::max;
use std::collections::{HashMap, VecDeque};

#[derive(Default, Debug)]
struct Node {
    flow_rate: u32,
    others: Vec<usize>,
    candidate_id: Option<usize>,
}



#[derive(Default, Debug)]
struct Graph<'a> {
    entries: HashMap<&'a str, usize>,
    pub nodes: Vec<Node>,
    candidates: Vec<usize>,
}

#[inline(always)]
fn is_bit_set(mask: u16, i: usize) -> bool {
    ((mask >> i) & 1) > 0
}


impl<'a> Graph<'a> {
    pub fn get_id(&self, key: &'a str) -> usize {
        *self.entries.get(key).unwrap()
    }

    pub fn get_or_set_id(&mut self, key: &'a str) -> usize {
        let current_len = self.entries.len();
        *self.entries.entry(key).or_insert_with(|| {
            self.nodes.push(Default::default());
            current_len
        })
    }

    pub fn assign_flowrate(&mut self, id: usize, flow_rate: u32) {
        self.nodes[id].flow_rate = flow_rate;
        if flow_rate > 0 {
            self.nodes[id].candidate_id = Some(self.candidates.len());
            self.candidates.push(id);
        }
    }

    pub fn add_edge(&mut self, u: usize, v: usize) {
        self.nodes[u].others.push(v);
    }

    pub fn flow_rate(&self, mask: u16) -> u32 {
        self.candidates
            .iter()
            .enumerate()
            .filter(|&(i, _)| is_bit_set(mask, i))
            .map(|(_, c)| self.nodes[*c].flow_rate)
            .sum()
    }

}
mod part_1 {
    use super::*;

    const MAX_TIME: usize = 30;
    #[derive(Hash, Eq, PartialEq, Clone, Debug, Default)]
    pub struct State {
        u: usize,
        t: usize,
        mask: u16,
    }


    #[derive(Default)]
    pub struct SolverData {
        dp: HashMap<State, u32>,
        queue: VecDeque<State>,
    }

    pub struct Solver<'a> {
        graph: Graph<'a>,
        data: SolverData,
    }

    impl SolverData {
        fn check(&mut self,  state: State, max_flow: u32) -> u32 {
            let mut inserted = true;
            let new_best_flow = *self.dp
                .entry(state.clone())
                .and_modify(|state| {
                    *state = max(*state, max_flow);
                    inserted = false;
                })
                .or_insert(max_flow);

            if inserted {
                self.queue.push_back(state);
            }
            new_best_flow
        }
    }

    impl<'a> Solver<'a> {
        pub fn new(graph: Graph<'a>) -> Self {
            Self {
                graph,
                data: SolverData::default()
            }
        }

        pub fn solve(&mut self) -> u32 {
            let graph = &self.graph;
            let data = &mut self.data;
            let initial_id = graph.get_id("AA");

            data.check(
                State {
                    u: initial_id,
                    t: 0,
                    mask: 0,
                },
                0,
            );

            let mut ans = 0;

            // The reason the queue approach works is that every action takes t=1 time.
            // We need to process the DP state by the order of time i.e.
            // DP[t][u][mask] = max(DP[t-1][..][..] + f(...))
            while let Some(state) = data.queue.pop_front() {
                let State { u, t, mask } = state;
                let pressure = data.dp[&state];
                // println!("State={:?} Pressure={:?}", state, pressure);

                ans = max(ans, pressure);

                if t > MAX_TIME {
                    break;
                }

                let new_pressure = pressure + graph.flow_rate(mask);
                // Open the valve if it's a candidate to open up
                if let Some(candidate_id) = graph.nodes[u].candidate_id {
                    if !is_bit_set(mask, candidate_id) {

                        let new_mask = mask | (1 << candidate_id);
                        data.check(
                            State {
                                u,
                                t: t + 1,
                                mask: new_mask,
                            },
                            new_pressure,
                        );
                    }
                }

                data.check(State { u, t: t + 1, mask }, new_pressure);
                for other in &graph.nodes[u].others {
                    data.check(State { u: *other, t: t + 1, mask }, new_pressure);
                }
            }

            ans
        }
    }
}

pub fn run(content: &str) -> u32 {
    let re = Regex::new(r"Valve (\w+) has flow rate=(\d+); tunnels? leads? to valves? (.+)").unwrap();

    let mut graph: Graph = Default::default();
    for line in content.lines() {
        let captures = re.captures(line).unwrap();
        let (_, [valve, flow_rate, adjacent]) = captures.extract();
        let valve = graph.get_or_set_id(valve);
        let flow_rate = flow_rate.parse().unwrap();

        graph.assign_flowrate(valve, flow_rate);

        for other in adjacent.split(", ") {
            let other = graph.get_or_set_id(other);
            graph.add_edge(valve, other);
        }

    }

    println!("{:?}", graph);

    part_1::Solver::new(graph).solve()

}
