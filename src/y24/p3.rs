use num::traits::{WrappingAdd, WrappingMul};

pub fn run(content: &str) -> u32 {
    let bytes = content.as_bytes();
    let mut result = 0;
    let mut state = 0;
    let mut first_num: u32 = 0;
    let mut second_num: u32 = 0;
    let mut i = 0;

    unsafe {
        while i < bytes.len() {
            let b = *bytes.get_unchecked(i);
            match state {
                0 => {
                    state = (b == b'm') as u8; // 1 if true, 0 if false
                }
                1 => {
                    state = (b == b'u') as u8 * 2;
                }
                2 => {
                    state = (b == b'l') as u8 * 3;
                }
                3 => {
                    if b == b'(' {
                        first_num = 0;
                        state = 4;
                    } else {
                        state = 0;
                    }
                }
                4 => {
                    if b == b',' {
                        second_num = 0;
                        state = 5;
                    } else if b >= b'0' && b <= b'9' {
                        first_num = first_num.wrapping_mul(10).wrapping_add((b - b'0') as u32);
                    } else {
                        state = 0;
                    }
                }
                5 => {
                    if b == b')' {
                        result = result.wrapping_add(&first_num.wrapping_mul(second_num));
                        state = 0;
                    } else if b >= b'0' && b <= b'9' {
                        second_num = second_num.wrapping_mul(10).wrapping_add((b - b'0') as u32);
                    } else {
                        state = 0;
                    }
                }
                _ => {
                    state = 0;
                }
            }
            i += 1;
        }
    }

    result
}

