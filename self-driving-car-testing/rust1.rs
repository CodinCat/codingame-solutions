use std::io;

macro_rules! parse_input {
    ($x:expr, $t:ident) => {
        $x.trim().parse::<$t>().unwrap()
    };
}

fn main() {
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let n = parse_input!(input_line, i32);

    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let xthen_commands = input_line.trim_right().to_string();

    let xthen_commands: Vec<&str> = xthen_commands.split(";").collect();
    let mut car_position = parse_input!(xthen_commands[0], usize) - 1;
    let commands = build_commands(xthen_commands);

    let mut ans = String::from("");
    let mut command_i = 0;
    for _ in 0..n as usize {
        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        let rthen_roadpattern = input_line.trim_right().to_string();
        let mut rthen_roadpattern = rthen_roadpattern.split(";");

        let road_repeat = parse_input!(rthen_roadpattern.next().unwrap(), i32);
        let road = rthen_roadpattern.next().unwrap();

        for _ in 0..road_repeat {
            match commands[command_i] {
                "R" => car_position += 1,
                "L" => car_position -= 1,
                _ => (),
            };
            command_i += 1;
            let row = get_car_on_road(road, car_position);
            ans.push_str(&row);
        }
    }

    println!("{}", ans);
}

fn build_commands(raw_commands: Vec<&str>) -> Vec<&str> {
    let mut commands: Vec<&str> = Vec::new();
    for i in 1..raw_commands.len() {
        let split = raw_commands[i].split_at(raw_commands[i].len() - 1);
        let repeat = parse_input!(split.0, i32);
        let command = split.1;
        for _ in 0..repeat {
            commands.push(command);
        }
    }
    commands
}

fn get_car_on_road(road: &str, car_position: usize) -> String {
    let mut ans = String::from("");
    let road_split = road.split_at(car_position);
    ans.push_str(road_split.0);
    ans.push_str("#");
    ans.push_str(road_split.1.split_at(1).1);
    ans.push_str("\n");
    ans
}
