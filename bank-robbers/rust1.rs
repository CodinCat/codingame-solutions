use std::io;

macro_rules! parse_input {
    ($x:expr, $t:ident) => {
        $x.trim().parse::<$t>().unwrap()
    };
}

fn main() {
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let r = parse_input!(input_line, usize);
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let v = parse_input!(input_line, usize);

    let mut robber_times = vec![0; r as usize];
    for _ in 0..v as usize {
        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        let inputs = input_line.split(" ").collect::<Vec<_>>();
        let c = parse_input!(inputs[0], usize);
        let n = parse_input!(inputs[1], usize);
        let time = usize::pow(10, n as u32) * usize::pow(5, (c - n) as u32);
        let next_rubber = find_index_of_min(&robber_times);
        robber_times[next_rubber] += time;
    }
    println!("{}", find_max(&robber_times));
}

fn find_index_of_min(v: &Vec<usize>) -> usize {
    let mut min = v[0];
    let mut index = 0;
    for i in 0..v.len() {
        if v[i] <= min {
            min = v[i];
            index = i;
        }
    }
    index
}

fn find_max(v: &Vec<usize>) -> usize {
    let mut max = v[0];
    for val in v.iter() {
        max = if *val > max { *val } else { max }
    }
    max
}
