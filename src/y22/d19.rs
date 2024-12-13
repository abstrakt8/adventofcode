use nom::bytes::complete::take_till;
use nom::character::{complete as ch, is_digit};
use nom::sequence::preceded;
use nom::{IResult, Parser};
use std::cmp::{max, Ordering};
use std::collections::{BinaryHeap, HashSet};

#[derive(Default, Eq, PartialEq, Debug)]
struct Blueprint {
    id: u32,
    ore_bot_ore_cost: u32,
    clay_bot_ore_cost: u32,
    obsidian_bot_ore_cost: u32,
    obsidian_bot_clay_cost: u32,
    geode_bot_ore_cost: u32,
    geode_bot_obsidian_cost: u32,
}

impl Blueprint {
    fn parse(i: &[u8]) -> IResult<&[u8], Self> {
        let parse_number = || preceded(take_till(is_digit), ch::u32);

        let (i, id) = parse_number().parse(i)?;
        let (i, ore_bot_ore_cost) = parse_number().parse(i)?;
        let (i, clay_bot_ore_cost) = parse_number().parse(i)?;
        let (i, obsidian_bot_ore_cost) = parse_number().parse(i)?;
        let (i, obsidian_bot_clay_cost) = parse_number().parse(i)?;
        let (i, geode_bot_ore_cost) = parse_number().parse(i)?;
        let (i, geode_bot_obsidian_cost) = parse_number().parse(i)?;

        Ok((
            i,
            Self {
                id,
                ore_bot_ore_cost,
                clay_bot_ore_cost,
                obsidian_bot_ore_cost,
                obsidian_bot_clay_cost,
                geode_bot_ore_cost,
                geode_bot_obsidian_cost,
            },
        ))
    }
}

// type = [ore, clay, obsidian, geode]
// C[i][0] = ore cost to build bot to produce type[i]
// C[i][1] = type[i-1] cost to build bot to produce type[i]
// C[0][1] will be undefined or not considered

// State will be `(time, A[0..=3], B[0..=3])` where A[i] = how many are available of type[i] and B[i] = number of bots of type[i]
// Let r = time_remaining
// Heuristic will be ((P[3][r]*r)+ (P[3][r-1]*(r-1) + P[3][r-2]*(r-2) ... P[3][1]*1) , which will not underestimate
// P[i][t] = number of potential bots fo type i that we can have at time t
// Essentially at P[i][r] it will just be B[i]
// For the heuristic we just assume that for each type i, we ignore other resources (assume that they are here) and fully invest
// what is produced into type i
// P[0][t] = P[0][t] + CanProduceOreBot(P[0][t-1])
// Same for type 1 clay that also only uses ore
// P[2][t] = P[2][t] + CanProduceObsBot(P[0][t-1], P[1][t-1])

// CanProduceObsBot(c0, c1) = max(c0 / C[Obs][0], c1 / C[Obs][1]) // Assuming nothing is left to make it easier
// Similar for CanProduceGeoBot(c0, c2) ...
// To not underestimate with the given available resources A, we should also just add them to P[i][t+1]
// Calculation should just be 24*2 DP entries, so quite feasible

const N_RESOURCES: usize = 4;
const MAX_TIME: usize = 24;
type CostMatrix = [[u32; 2]; N_RESOURCES];
type DpMatrix = [[u32; MAX_TIME]; N_RESOURCES];

#[derive(Default, Debug, Clone, Hash, Eq, PartialEq)]
struct State {
    t: usize,
    i: usize,                 // Bot type we currently consider
    a: [u32; N_RESOURCES],    // Available resources
    b: [u32; N_RESOURCES],    // Bots available now
    next: [u32; N_RESOURCES], // Bots available in the next time frame (but can't be used in this time frame)
}

#[derive(Default, Debug)]
struct HeapData {
    state: State,
    potential: u32,
}

impl HeapData {
    pub fn potential_geodes(&self) -> u32 {
        self.state.a[3] + self.potential
    }
}

impl Eq for HeapData {}

impl PartialEq<Self> for HeapData {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(&other).is_eq()
    }
}

impl PartialOrd<Self> for HeapData {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for HeapData {
    fn cmp(&self, other: &Self) -> Ordering {
        self.potential_geodes().cmp(&other.potential_geodes())
    }
}

struct Solver {
    blueprint: Blueprint,
}

impl Blueprint {
    fn cost_matrix(&self) -> CostMatrix {
        [
            [self.ore_bot_ore_cost, 0],
            [self.clay_bot_ore_cost, 0],
            [self.obsidian_bot_ore_cost, self.obsidian_bot_clay_cost],
            [self.geode_bot_ore_cost, self.geode_bot_obsidian_cost],
        ]
    }
}

fn can_build_and_modify(i: usize, a: &mut [u32; N_RESOURCES], cost: &CostMatrix) -> bool {
    let possible = {
        if i < 2 && a[0] >= cost[i][0] {
            a[0] -= cost[i][0];
            true
        } else if i >= 2 && a[0] >= cost[i][0] && a[i - 1] >= cost[i][1] {
            a[0] -= cost[i][0];
            a[i - 1] -= cost[i][1];
            true
        } else {
            false
        }
    };
    possible
}

impl Solver {
    pub fn new(blueprint: Blueprint) -> Self {
        Self { blueprint }
    }

    pub fn search(&mut self) -> u32 {
        let mut heap: BinaryHeap<HeapData> = BinaryHeap::new();
        let mut seen: HashSet<State> = HashSet::new();
        let cost: CostMatrix = self.blueprint.cost_matrix();

        let check_and_push =
            |seen: &mut HashSet<State>, heap: &mut BinaryHeap<HeapData>, state: State| {
                if seen.insert(state.clone()) {
                    let potential = heuristic(&state, &cost);
                    heap.push(HeapData { state, potential });
                }
            };

        check_and_push(
            &mut seen,
            &mut heap,
            State {
                t: 0,
                i: 0,
                a: [0, 0, 0, 0],
                b: [1, 0, 0, 0], // Only one ore bot available
                next: [0, 0, 0, 0],
            },
        );

        let mut max_geodes = 0;

        let mut seen_t = 0;
        let mut processed = 0;

        while let Some(data) = heap.pop() {
            processed += 1;
            if seen_t < data.state.t {
                println!(
                    "new seen_t = {seen_t} processed={processed} heap.len()={}",
                    heap.len()
                );
                seen_t = data.state.t;
                // dbg!(&data);
                // dbg!(&heap.len());
            }

            let state = data.state;

            // Two decisions:
            // 1. Build one more bot for i
            // 2. Decide for next bot i+1 (or move on to t+1)

            // 1. Build one more if possible
            {
                let mut state = state.clone();
                let i = state.i;
                let possible = can_build_and_modify(i, &mut state.a, &cost);
                if possible {
                    state.next[i] += 1;
                    check_and_push(&mut seen, &mut heap, state);
                }
            }

            // 2. Consider next one
            {
                let i = state.i;
                let mut state = state.clone();

                if i == 3 {
                    state.i = 0;
                    for i in 0..N_RESOURCES {
                        state.a[i] += state.b[i];
                        state.b[i] += state.next[i];
                        state.next[i] = 0;
                    }
                    state.t += 1;
                } else {
                    state.i += 1;
                }

                check_and_push(&mut seen, &mut heap, state);
            }
        }

        max_geodes * self.blueprint.id
    }
}

fn heuristic(state: &State, cost: &CostMatrix) -> u32 {
    let State { b, a, next, .. } = state;
    let cur_time = state.t;
    if cur_time == MAX_TIME {
        return 0;
    }

    let mut dp = [[0u32; MAX_TIME]; N_RESOURCES];

    // Bonus is taking the current available resources into account to not underestimate
    let potential_produce = |dp: &DpMatrix, i: usize, t: usize| -> u32 {
        let first_bonus = if t == cur_time { a[0] } else { 0 };
        let first = (dp[0][t - 1] + first_bonus).div_ceil(cost[i][0]);
        if i >= 2 {
            let second_bonus = if t == cur_time { a[i - 1] } else { 0 };
            let second = (dp[i - 1][t - 1] + second_bonus).div_ceil(cost[i][1]);
            max(first, second)
        } else {
            first
        }
    };

    for i in 0..N_RESOURCES {
        dp[i][cur_time] = b[i];
    }

    let mut potential_geode = 0;

    for t in cur_time + 1..MAX_TIME {
        for i in 0..N_RESOURCES {
            dp[i][t] = dp[i][t - 1] + potential_produce(&dp, i, t);

            // Bonus using the next set available of bots that we have already built
            if t == cur_time + 1 {
                dp[i][t] += next[i];
            }
        }
        potential_geode += dp[3][t - 1];
    }

    potential_geode
}

pub fn run(content: &str) -> u32 {
    let blueprints: Vec<Blueprint> = content
        .lines()
        .map(|line| {
            let (_, blueprint) = Blueprint::parse(line.as_bytes()).expect("Could not parse!");
            blueprint
        })
        .collect();

    blueprints
        .into_iter()
        .map(|blueprint| {
            dbg!(&blueprint.id);
            Solver::new(blueprint).search()
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn parse_blueprint() {
        let line: &str = "Blueprint 1: Each ore robot costs 4 ore. Each clay robot costs 2 ore. Each obsidian robot costs 3 ore and 14 clay. Each geode robot costs 2 ore and 7 obsidian.";
        let (_, blueprint) = Blueprint::parse(line.as_bytes()).expect("No parsing error");
        assert_eq!(
            blueprint,
            Blueprint {
                id: 1,
                ore_bot_ore_cost: 4,
                clay_bot_ore_cost: 2,
                obsidian_bot_ore_cost: 3,
                obsidian_bot_clay_cost: 14,
                geode_bot_ore_cost: 2,
                geode_bot_obsidian_cost: 7,
            }
        )
    }
}
