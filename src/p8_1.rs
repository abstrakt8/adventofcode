use std::collections::HashMap;

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

pub fn run(input: &str) -> u32 {
    let lines: Vec<&str> = input.lines().collect();

    let steps: Vec<char> = lines[0].chars().collect();

    let mut ids: HashMap<String, NodeID> = HashMap::new();

    let reg = Regex::new(r"([A-Z]{3}) = \(([A-Z]{3}), ([A-Z]{3})\)").unwrap();

    let mut info: Vec<[NodeID; 3]> = vec![];

    let mut get_id = |s: String| {
        if let Some(&node_id) = ids.get(&s) {
            node_id
        } else {
            let new_node_id = ids.len() as NodeID;
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


    let aaa_id = *ids.get("AAA").unwrap();
    let zzz_id = *ids.get("ZZZ").unwrap();

    // cycle (cur, i)

    let mut cnt = 0;
    let mut cur = aaa_id as NodeID;
    while cur != zzz_id {
        let c = steps[cnt % steps.len()];
        let d = if c == 'L' { 1 } else { 2 };
        cur = info[cur as usize][d];
        cnt += 1;
    }
    return cnt as u32;
}
