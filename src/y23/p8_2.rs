use num::integer::{gcd, lcm};
use std::collections::{HashMap, HashSet};
use std::panic::panic_any;

use log::debug;
use regex::Regex;

type NodeID = u16;

// lazy_static! {
//     static ref ID_COUNTER: Mutex<usize> = Mutex::new(0);
// }

//
// static mut MAP: HashMap<&str, u16> = HashMap::new();
//
// unsafe fn GetId(key: &str) {
//     let e = MAP.entry(key).or_insert()
// }
//
// impl FromStr for Node {
//     type Err = Report;
//
//     fn from_str(s: &str) -> Result<Self, Self::Err> {
//         // e.g. GXT = (MQM, CHN)
//         let mut id_counter = ID_COUNTER.lock().unwrap();
//     }
// }

pub fn run(input: &str) -> i128 {
    let lines: Vec<&str> = input.lines().collect();

    let steps: Vec<char> = lines[0].chars().collect();

    let mut ids: HashMap<String, NodeID> = HashMap::new();
    let mut is_goal: HashSet<NodeID> = HashSet::new();

    let reg = Regex::new(r"([A-Z]{3}) = \(([A-Z]{3}), ([A-Z]{3})\)").unwrap();

    let mut info: Vec<[NodeID; 3]> = vec![];

    let mut startin_nodes = vec![];

    let mut get_id = |s: String| {
        if let Some(&node_id) = ids.get(&s) {
            node_id
        } else {
            let new_node_id = ids.len() as NodeID;
            if s.ends_with("A") {
                startin_nodes.push(new_node_id);
            }
            if s.ends_with("Z") {
                is_goal.insert(new_node_id);
            }
            ids.insert(s, new_node_id);
            new_node_id
        }
    };

    lines[2..].iter().for_each(|l| {
        let caps = reg.captures(*l).unwrap();
        let ints: [NodeID; 3] = [1, 2, 3].map(|i| {
            get_id(caps[i].to_string())
        });
        info.push(ints);
    });
    info.sort();
    for i in 0..info.len() {
        assert_eq!(info[i][0], i as NodeID);
    }


    println!("Steps.cnt = {:}", steps.len());
    //
    let mut cycles_lcm: i128 = 1;
    let mut cycles_gcd = 0; // checking for coprime for CRT

    let mut cycles: Vec<u32> = vec![];

    startin_nodes.iter().for_each(|s| {
        let mut vis: HashMap<(NodeID, usize), usize> = HashMap::new();
        let mut cur = *s;
        let mut cnt = 0;
        let mut z = 0;

        println!("Cycle for {:}", *s);
        loop {
            let i = cnt % steps.len();
            if let Some(prev) = vis.get(&(cur, i)) {
                let cycle_len = cnt - prev;
                let Z = z - prev;
                dbg!(cycle_len, Z, prev);

                cycles.push(cycle_len as u32);
                break;
            } else {
                vis.insert((cur, i), cnt);
            }

            // if !vis.insert((cur, i)) {
            //     cycles_lcm = lcm(cycles_lcm, cnt as i128);
            //     cycles_gcd = gcd(cycles_gcd, cnt);
            //     println!("Size = {:}, LCM = {:}, GCD = {:}", cnt, cycles_lcm, cycles_gcd);
            //     cycles.push((cnt as u32, z));
            //     break;
            // }
            //
            if is_goal.contains(&cur) {
                assert_eq!(z, 0); // Unassigned
                z = cnt;
            }
            // dbg!(cur, cnt);

            let d = if steps[i] == 'L' { 1 } else { 2 };
            cur = info[cur as usize][d];
            cnt += 1;
        }
    });


    let precomputed_cycles = [
        (
            13207,
            13205,
        ),
        (
            19951,
            19949,
        ),
        (
            14893,
            14891,
        ),
        (
            20513,
            20511,
        ),
        (
            22199,
            22197,
        ),
    ];
    // // other_lcm= 13244297877792339435
    // //  LCM     = 1046299532345594815365,
    let outlier_cycle: (i128, i128) =
        (
            12083,
            12079,
        );
    //
    let mut other_lcm: i128 = 1;
    for (c, _) in precomputed_cycles {
        other_lcm = lcm(other_lcm, c);
    }
    println!("o % co = {:}", other_lcm.rem_euclid(outlier_cycle.0));

    // let mut i = 0;
    //
    for i in 0..=outlier_cycle.0 {
    // for i in 0..outlier_cycle.0 {
        let k: i128 = (outlier_cycle.0  - 2 -  i * other_lcm).rem_euclid(outlier_cycle.0);

        if k == outlier_cycle.1  {
            break;
        }
        println!("i={:}, k={:}", i, k);
    }
    //
    // println!("i={:}", i);
    // //
    //
    // let ans: i128 = (cycles_lcm - 2 - i * other_lcm) as i128;

    // let mut ans = 1;
    // for x in cycles {
    //     ans = lcm(ans, x as i128);
    // }
    let mut ans = 1;
    return ans ;
}
