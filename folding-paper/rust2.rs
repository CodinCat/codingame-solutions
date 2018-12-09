use std::io;

type Pattern = (char, char, char, char);

struct Paper {
    r: i32,
    l: i32,
    u: i32,
    d: i32,
}

impl Paper {
    pub fn fold(&mut self, side: char) {
        let pattern = match side {
            'R' => ('R', 'L', 'U', 'D'),
            'L' => ('L', 'R', 'U', 'D'),
            'U' => ('U', 'D', 'R', 'L'),
            'D' => ('D', 'U', 'R', 'L'),
            _ => ('F', 'U', 'C', 'K')
        };
        self.update(pattern);
    }
    fn update(&mut self, pattern: Pattern) {
        let (a, b, c, d) = pattern;
        let values;
        {
            values = self.get_abcd(pattern);
        }
        self.set(b, values.0 + values.1);
        self.set(a, 1);
        self.set(c, values.2 * 2);
        self.set(d, values.3 * 2);
    }
    fn get_abcd(&self, pattern: Pattern) -> (i32, i32, i32, i32) {
        let (a, b, c, d) = pattern;
        (self.get(a), self.get(b), self.get(c), self.get(d))
    } 
    fn get(&self, c: char) -> i32 {
        match c {
            'R' => self.r,
            'L' => self.l,
            'U' => self.u,
            'D' => self.d,
            _ => 0
        }
    }
    fn set(&mut self, c: char, value: i32) {
        match c {
            'R' => { self.r = value; },
            'L' => { self.l = value; },
            'U' => { self.u = value; },
            'D' => { self.d = value; },
            _ => {}
        };
    }
}

fn main() {
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let order = input_line.trim_right().to_string();
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let side = input_line.trim_right().to_string();
    
    let mut paper = Paper {
        r: 1,
        l: 1,
        u: 1,
        d: 1,
    };

    for c in order.chars() {
        paper.fold(c);
    }
    println!("{}", paper.get(side.chars().next().unwrap()));
}
