use std::env;

fn main() {
    let mut file: String = String::new();
    for args in env::args() {
        file = args;
    }
    let part_var = env::var("part");
    match part_var {
        Ok(var) => {
            let contents = std::fs::read_to_string(file).expect("Could not load file");
            let all_inputs = contents
                .split("\n")
                .map(|row| row.parse::<i32>().unwrap())
                .collect::<Vec<i32>>();
            let resp = match var.as_str() {
                "part1" => part_one(&all_inputs),
                "part2" => part_two(&all_inputs),
                _ => panic!("Does not recognice part"),
            };
            println!("Answer is {}", resp)
        }
        Err(err) => panic!("Err: {}", err),
    }
}

fn part_one(inputs: &Vec<i32>) -> i32 {
    let mut increased = 0;
    inputs.iter().reduce(|a, b| {
        if b > a {
            increased += 1;
        }
        b
    });
    increased
}

fn part_two(inputs: &Vec<i32>) -> i32 {
    let mut increased = 0;
    for n in 0..inputs.len()-3 {
        let first_sum: i32 = inputs[n..=n+2].iter().sum();
        let second_sum: i32 = inputs[n+1..=n+3].iter().sum();
        if second_sum > first_sum {
            increased += 1;
        }
    }
    increased
}