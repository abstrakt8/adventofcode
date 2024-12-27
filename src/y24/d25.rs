pub fn run(content: &str) -> u32 {
    let mut schemas: Vec<Vec<i32>> = vec![];
    for lines in content.split("\n\n") {
        let vec = lines.lines().map(&str::as_bytes).collect::<Vec<&[u8]>>();

        let mut schema : Vec<i32> = vec![];
        for j in 0..vec[0].len() {
            let hash = (0..vec.len()).position(|i| vec[i][j] == b'#').unwrap() as i32;
            let dot = (0..vec.len()).position(|i| vec[i][j] == b'.').unwrap() as i32;
            if dot < hash {
                schema.push(hash - (vec.len() as i32));
            } else {
                schema.push(dot);
            }
        }
        schemas.push(schema);
    }

    let mut ans = 0;
    for i in 0..schemas.len() {
        for j in i + 1..schemas.len() {
            if schemas[i][0] * schemas[j][0] > 0 {
                continue;
            }

            let mut good = 1;
            for k in 0..schemas[i].len() {
                if schemas[i][k].abs() + schemas[j][k].abs() > 7 {
                    good = 0;
                }
            }
            ans += good;
        }
    }
    ans
}
