use std::{env, panic};

fn main() {
    let mut file: String = String::new();
    for args in env::args() {
        file = args;
    }
    let part_var = env::var("part");
    match part_var {
        Ok(var) => {
            let contents = std::fs::read_to_string(file).expect("Could not load file");
            let resp = match var.as_str() {
                "part1" => part_one(contents),
                "part2" => part_two(contents),
                _ => panic!("Does not recognice part"),
            };
            println!("Answer is {}", resp)
        }
        Err(err) => panic!("Err: {}", err),
    }
}

fn part_one(contents: String) -> i32 {
    let mut x_axis = 0;
    let mut y_axis = 0;

    contents.split("\n").for_each(|row| {
        let step = row.split(" ").collect::<Vec<&str>>();
        let axis = step.get(0).unwrap();
        let amount_of_steps = step.get(1).unwrap().parse::<i32>().unwrap();
        match *axis {
            "forward" => x_axis += amount_of_steps,
            "down" => y_axis += amount_of_steps,
            "up" => y_axis -= amount_of_steps,
            _ => panic!("Axis not recognized: {}", *axis),
        };
    });
    x_axis * y_axis
}

fn part_two(contents: String) -> i32 {
    let mut horizontal = 0;
    let mut depth = 0;
    let mut aim = 0;

    contents.split("\n").for_each(|row| {
        let step = row.split(" ").collect::<Vec<&str>>();
        let axis = step.get(0).unwrap();
        let amount_of_steps = step.get(1).unwrap().parse::<i32>().unwrap();
        match *axis {
            "forward" => {
                horizontal += amount_of_steps;
                depth += aim * amount_of_steps;
            }
            "down" => aim += amount_of_steps,
            "up" => aim -= amount_of_steps,
            _ => panic!("Axis not recognized: {}", *axis),
        };
    });
    horizontal * depth
}
