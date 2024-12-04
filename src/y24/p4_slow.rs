// .XMASAMX
// 01234567
#[inline(always)]
pub fn automata(state: u8, next: u8) -> (u8, u32) {
    match (state, next) {
        (1, b'M') => (2, 0),
        (2, b'A') => (3, 0),
        (3, b'S') => (4, 1),
        (4, b'A') => (5, 0),
        (5, b'M') => (6, 0),
        (6, b'X') => (1, 1),
        _ =>
            match next {
                b'X' => (1, 0),
                b'S' => (4, 0),
                _ => (0, 0)
            }
    }
}

#[inline(always)]
fn go(state: &mut u8, next: u8, cnt: &mut u32) {
    let (next, new) = automata(*state, next);

    *state = next;
    *cnt += new;
}

pub fn run1(content: &str) -> u32 {
    const N: usize = 150;
    const D: usize = 2 * N + 1;
    static mut cols: [u8; N] = [0; N];
    static mut diag_se: [u8; D] = [0; D];
    static mut diag_sw: [u8; D] = [0; D];
    let mut row: u8 = 0;

    let mut ans = 0;
    let mut bi = 0;
    let mut i = 0;
    let mut j = 0;
    let bytes = content.as_bytes();

    // SAFETY: Because I say so
    unsafe {
        while bi < bytes.len() {
            let b = *bytes.get_unchecked(bi);
            bi += 1;

            if b.is_ascii_whitespace() {
                row = 0;
                j = 0;
                i += 1;
                continue;
            }

            go(&mut row, b, &mut ans);
            go(&mut cols[j], b, &mut ans);
            go(&mut diag_se[i + j], b, &mut ans);
            go(&mut diag_sw[N + i - j], b, &mut ans);

            j += 1;
        }
    }
    ans
}

pub fn run2(content: &str) -> u32 {
    let g: Vec<Vec<char>> = content.lines().map(|line| line.chars().collect()).collect();

    let matches2 = |i: usize, j: usize, c1: char, c2: char| -> u32 {
        if i >= 1 && i <= g.len() && j >= 1 && j <= g[i - 1].len() {
            let c = g[i - 1][j - 1];
            if c == c1 {
                1
            } else if c == c2 {
                2
            } else {
                0
            }
        } else {
            0
        }
    };
    let matches = |i: usize, j: usize, c: char| -> u32 {
        matches2(i, j, c, c)
    };

    let mut ans = 0;
    for i in 1..=g.len() {
        for j in 1..=g.len() {
            if matches(i, j, 'A') == 0 {
                continue;
            }
            if matches2(i - 1, j - 1, 'M', 'S') + matches2(i + 1, j + 1, 'M', 'S') == 3 &&
                matches2(i - 1, j + 1, 'M', 'S') + matches2(i + 1, j - 1, 'M', 'S') == 3 {
                ans += 1;
            }
        }
    }

    ans
}


pub fn run(content: &str) -> u32 {
    run2(content)
}
