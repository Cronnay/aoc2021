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
    let mut sum = 0;
    for (index, row) in inputs.iter().enumerate() {
        if is_number_prime(*row as i16) {
            sum += index as i32 * row;
        }
    }
    sum
}

fn part_two(inputs: &Vec<i32>) -> i32 {
    let mut sum = 0;
    for (index, row) in inputs.iter().enumerate() {
        if !is_number_prime(*row as i16) {
            if index % 2 == 0 {
                sum += inputs.get(index).expect("Input does not exist");
            } else {
                sum -= inputs.get(index).expect("Input does not exist");
            }
        }
    }
    sum
}

fn is_number_prime(number: i16) -> bool {
    recursive_number_prime(number, 2)
}

fn recursive_number_prime(n: i16, i: i16) -> bool {
    if n <= 2 {
        return n == 2;
    } else if n % i == 0 {
        return false;
    } else if i * i > n {
        return true;
    } else {
        recursive_number_prime(n, i + 1)
    }
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
    assert_eq!(is_number_prime(2), true);
    assert_eq!(is_number_prime(0), false);
}

#[test]
fn test_part_one_should_return_2421() {
    let inputs = vec![0, 3, 4, 42, 106, 107, 267, 269];
    assert_eq!(part_one(&inputs), 2421)
}

#[test]
fn test_part_two_should_return_335() {
    let inputs = vec![0, 3, 4, 42, 106, 107, 267, 269];
    assert_eq!(part_two(&inputs), 335)
}
