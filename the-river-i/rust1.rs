use std::io;

macro_rules! parse_input {
    ($x:expr, $t:ident) => {
        $x.trim().parse::<$t>().unwrap()
    };
}

fn main() {
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let mut r1 = parse_input!(input_line, i64);
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let mut r2 = parse_input!(input_line, i64);

    while r1 != r2 {
        let sorted = sort_two((r1, r2));
        r1 = sorted.0;
        r2 = sorted.1;
        let mut step = 0;
        for c in r1.to_string().chars() {
            step += c.to_digit(10).unwrap();
        }
        r1 += step as i64;
    }

    println!("{}", r1);
}

fn sort_two(ab: (i64, i64)) -> (i64, i64) {
    let (a, b) = ab;
    if a > b {
        (b, a)
    } else {
        (a, b)
    }
}
