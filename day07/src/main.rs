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
                .split(",")
                .map(|character| character.parse::<i32>().unwrap())
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
    let max_value = inputs.iter().max().unwrap();
    let mut lowest_fuel = i32::MAX;

    for i in 0..=*max_value {
        let mut cost_of_position = 0;
        for j in 0..inputs.len() {
            let diff = (inputs[j] - i).abs();
            cost_of_position += diff;
        }
        if lowest_fuel > cost_of_position {
            lowest_fuel = cost_of_position;
        }
    }
    lowest_fuel
}

fn part_two(inputs: &Vec<i32>) -> i32 {
    0
}

#[test]
fn test_part_one() {
    let input = vec![16,1,2,0,4,2,7,1,2,14];
    assert_eq!(part_one(&input), 37);
}