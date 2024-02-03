pub(crate) fn run(input: &str) -> i64 {
    let lines: Vec<_> = input.lines().collect();
    let t = lines[0].split(':').last().unwrap().replace(' ', "").parse::<i64>().unwrap();
    let d = lines[1].split(':').last().unwrap().replace(' ', "").parse::<i64>().unwrap();

    // eprintln!(time);
    // dbg!(t, d);

    // tj - j^2 >= d

    //(-b +- sqrt(b^2 - 4ac)) / 2a

    let a = -1f64;
    let b = t as f64;
    let c = -d as f64;

    let ha = (b * b - 4f64 * a * c);
    let mut x1 = (-b - ha.sqrt()) / (2f64 * a);
    let mut x2 = (-b + ha.sqrt()) / (2f64 * a);
    if x1 > x2 {
        (x1, x2) = (x2, x1)
    }
    // dbg!(x1, x2);

    return (x2.floor() - x1.ceil() + 1.0) as i64;
}