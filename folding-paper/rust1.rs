use std::io;
use std::collections::HashMap;

fn main() {
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let order = input_line.trim_right().to_string();
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let side = input_line.trim_right().to_string();
    
    let mut result = HashMap::new();
    result.insert("R", 1);
    result.insert("L", 1);
    result.insert("U", 1);
    result.insert("D", 1);
    
    for c in order.chars() {
        let R = result["R"];
        let L = result["L"];
        let U = result["U"];
        let D = result["D"];
        match c {
            'R' => {
                result.insert("L", R + L);
                result.insert("R", 1);
                result.insert("U", U * 2);
                result.insert("D", D * 2);
            },
            'L' => {
                result.insert("R", R + L);
                result.insert("L", 1);
                result.insert("U", U * 2);
                result.insert("D", D * 2);
            },
            'U' => {
                result.insert("D", U + D);
                result.insert("U", 1);
                result.insert("R", R * 2);
                result.insert("L", L * 2);
            },
            'D' => {
                result.insert("U", U + D);
                result.insert("D", 1);
                result.insert("R", R * 2);
                result.insert("L", L * 2);
            },
            _ => (),
        }
    }
    let ans = result.get(&side[..]).unwrap();
    println!("{}", ans);
}
