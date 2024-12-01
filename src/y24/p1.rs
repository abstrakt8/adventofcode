// 50microseconds, fastest solution
pub fn run(content: &str) -> u32 {
    const MAX_NUM: usize = 100000;

    static mut COUNT1: [u32; MAX_NUM] = [0; MAX_NUM];
    static mut COUNT2: [u32; MAX_NUM] = [0; MAX_NUM];

    unsafe {
        let mut ptr = content.as_ptr();
        let mut ans = 0u32;

        for _ in 0..1000 {
            let num1 = parse_5digit(ptr) as usize;
            let num2 = parse_5digit(ptr.add(8)) as usize;

            ans += COUNT2[num1] * num1 as u32;
            COUNT1[num1] += 1;

            ans += COUNT1[num2] * num2 as u32;
            COUNT2[num2] += 1;

            ptr = ptr.add(14);
        }

        ans
    }
}

#[inline(always)]
unsafe fn parse_5digit(ptr: *const u8) -> u32 {
    (*ptr as u32 & 15) * 10000 +
        (*ptr.add(1) as u32 & 15) * 1000 +
        (*ptr.add(2) as u32 & 15) * 100 +
        (*ptr.add(3) as u32 & 15) * 10 +
        (*ptr.add(4) as u32 & 15)
}
