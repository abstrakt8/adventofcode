pub fn run(content: &str) -> u32 {
    const MAX_NUM: usize = 100000; // Max possible number + 1
    const LINES: usize = 1000;
    const LINE_LEN: usize = 14; // 5 digits + 3 spaces + 5 digits + newline

    let mut count1 = [0u32; MAX_NUM];
    let mut count2 = [0u32; MAX_NUM];
    let bytes = content.as_bytes();
    let mut ans = 0u32;

    unsafe {
        // Count occurrences and collect first numbers
        for i in 0..LINES {
            let line_start = i * LINE_LEN;
            let num1 = parse_5digit(bytes.as_ptr().add(line_start));
            let num2 = parse_5digit(bytes.as_ptr().add(line_start + 8));

            count1[num1 as usize] += 1;
            ans += count2[num1 as usize] * num1;

            count2[num2 as usize] += 1;
            ans += count1[num2 as usize] * num2;
        }

        ans as u32
    }
}

#[inline(always)]
unsafe fn parse_5digit(ptr: *const u8) -> u32 {
    ((*ptr - b'0') as u32) * 10000 +
        ((*ptr.add(1) - b'0') as u32) * 1000 +
        ((*ptr.add(2) - b'0') as u32) * 100 +
        ((*ptr.add(3) - b'0') as u32) * 10 +
        ((*ptr.add(4) - b'0') as u32)
}