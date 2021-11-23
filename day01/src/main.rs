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
            let mut all_inputs: Vec<i32> = Vec::new();
            for line in contents.lines() {
                if let Ok(parsed) = line.parse::<i32>() {
                    all_inputs.push(parsed);
                }
            }
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
    let mut sum = 0;
    for n in 0..inputs.len() {
        if is_number_prime(n as i16) {
            sum += n as i32 * inputs.get(n).expect("Input does not exist");
        }
    }
    sum
}

fn part_two(inputs: &Vec<i32>) -> i32 {
    let mut sum = 0;
    for n in 0..inputs.len() {
        if !is_number_prime(n as i16) {
            if n % 2 == 0 {
                sum += inputs.get(n).expect("Input does not exist");
            } else {
                sum -= inputs.get(n).expect("Input does not exist");
            }
        }
    }
    sum
}

fn is_number_prime(number: i16) -> bool {
    if number == 1 {
        return true;
    }

    if number % 2 == 0 {
        return false;
    }

    for n in 3..number {
        if number % n == 0 {
            return false;
        }
    }

    true
}

#[test]
fn test_is_number_prime() {
    assert_eq!(is_number_prime(3), true);
    assert_eq!(is_number_prime(5), true);
    assert_eq!(is_number_prime(7), true);
    assert_eq!(is_number_prime(8), false);
    assert_eq!(is_number_prime(9), false);
    assert_eq!(is_number_prime(11), true);
    assert_eq!(is_number_prime(12), false);
    assert_eq!(is_number_prime(83), true);
    assert_eq!(is_number_prime(84), false);
    assert_eq!(is_number_prime(89), true);
    assert_eq!(is_number_prime(2), false);
    assert_eq!(is_number_prime(0), false);
}

#[test]
fn test_part_one_should_return_105() {
    let inputs = vec![1337, 25, 66, 10, 79, 10];
    assert_eq!(part_one(&inputs), 105)
}


#[test]
fn test_part_one_should_return_0() {
    let inputs = vec![0];
    assert_eq!(part_one(&inputs), 0)
}

#[test]
fn test_part_one_should_return_15() {
    let inputs = vec![0,15,33];
    assert_eq!(part_one(&inputs), 15)
}

#[test]
fn test_part_two_should_return_1324() {
    let inputs = vec![1337, 25, 66, 10, 79, 10];
    assert_eq!(part_two(&inputs), 1482)
}

#[test]
fn test_part_two_should_return_1668() {
    let inputs = vec![1337, 25, 66, 10, 79, 10, 66, 9, 123, 3];
    assert_eq!(part_two(&inputs), 1668)
} 