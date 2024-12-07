use bitvec::prelude::*;
use memchr::memchr;
use rayon::prelude::*;
use std::sync::Mutex;

const DIRECTIONS: [(i32, i32); 4] = [(-1, 0), (0, 1), (1, 0), (0, -1)];
enum State {
    Wall,
    Outside,
    Nothing,
}

pub enum Part {
    Part1,
    Part2,
}

pub fn run_part(content: &str, part: Part) -> u32 {
    let bytes = content.as_bytes();
    let n = memchr(b'\n', bytes).unwrap() as i32;
    let starting_pos = memchr(b'^', bytes).unwrap() as i32;

    let pos_to_xy = |pos: i32| -> (i32, i32) {
        (pos / (n + 1), pos % (n + 1))
    };

    let xy = |x: i32, y: i32| -> usize {
        ((x * (n + 1)) + y) as usize
    };
    let xyd = |x: i32, y: i32, d: i32| -> usize {
        xy(x, y) * 4 + d as usize
    };

    let at = |x: i32, y: i32| -> State {
        if x >= 0 && x < n && y >= 0 && y < n {
            if bytes[xy(x, y)] == b'#' {
                State::Wall
            } else {
                State::Nothing
            }
        } else {
            State::Outside
        }
    };

    let next = |x: i32, y: i32, d: i32| -> (i32, i32) {
        (x + DIRECTIONS[d as usize].0, y + DIRECTIONS[d as usize].1)
    };

    let go = |x: i32, y: i32, d: i32| -> (i32, i32, i32) {
        let (nx, ny) = next(x, y, d);
        match at(nx, ny) {
            State::Outside => (0, 0, -1),
            State::Nothing => (nx, ny, d),
            State::Wall => (x, y, (d + 1) % 4),
        }
    };

    let mut ans1 = 0;
    let mut bset = bitarr![0; 150*150];
    let mut d = 0;
    let (mut x, mut y) = pos_to_xy(starting_pos);
    loop {
        if !bset.replace(xy(x, y), true) {
            ans1 += 1;
        }
        (x, y, d) = go(x, y, d);
        if d == -1 {
            break;
        }
    }

    if matches!(part, Part::Part1) {
        return ans1;
    }

    type BitArrayType = BitArray<[u16; 5625]>;
    // thread_local! {
    //     static BSET: RefCell<BitArrayType> = RefCell::new(bitarr![u16, LocalBits; 0; 150*150*4]);
    // }
    let num_threads = rayon::current_num_threads();
    let bsets: Vec<_> = (0..num_threads)
        .map(|_| Mutex::new(bitarr![0; 150*150*4]))
        .collect();

    let simulate = |(ox, oy)| {
        let thread_id = rayon::current_thread_index().unwrap();
        let mut bset = bsets[thread_id].lock().unwrap();
        bset.fill(false);

        let mut d = 0;
        let (mut x, mut y) = pos_to_xy(starting_pos);
        loop {
            if bset.replace(xyd(x, y, d), true) {
                return 1;
            }
            let (nx, ny) = next(x, y, d);
            // Wall override
            if (nx, ny) == (ox, oy) {
                d = (d + 1) & 3;
            } else {
                (x, y, d) = go(x, y, d);
                if d == -1 {
                    return 0;
                }
            }
        }
    };

    bset.iter_ones().map(|i| {
        pos_to_xy(i as i32)
    })
        .par_bridge()
        .map(simulate)
        .sum()
}


// 1480
pub fn run(content: &str) -> u32 {
    // run_part(content, Part::Part1);
    run_part(content, Part::Part2)
}
