use crossbeam::thread;
use std::{env, panic, time::Instant};

fn main() {
    let mut file: String = String::new();
    let now = Instant::now();
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
    let duration = now.elapsed();
    println!("Elapsed: {:?}", duration);
}

fn part_one(contents: String) -> u64 {
    let list = contents.split("\n").collect::<Vec<&str>>();
    let length_of_binary = list.get(0).unwrap().chars().count();
    let mut gamma = String::new();
    let mut epsilon = String::new();
    for n in 0..length_of_binary {
        let (zeroes, ones) = get_ratings_of_bits(&list, n);
        if zeroes > ones {
            gamma.push('0');
            epsilon.push('1');
        } else {
            gamma.push('1');
            epsilon.push('0');
        }
    }
    let gamma_decimal = isize::from_str_radix(&gamma, 2).unwrap();
    let epsilon_decimal = isize::from_str_radix(&epsilon, 2).unwrap();
    (gamma_decimal * epsilon_decimal).try_into().unwrap()
}

fn part_two(contents: String) -> u64 {
    let splitted_contents = contents.split("\n").collect::<Vec<&str>>();
    thread::scope(|s| {
        let oxygen_thread = s.spawn(|_| {
            let mut oxygen = splitted_contents.clone();
            let mut n = 0;
            while oxygen.len() > 1 {
                if oxygen.len() > 1 {
                    let (z, o) = get_ratings_of_bits(&oxygen, n);
                    if z > o {
                        oxygen.retain(create_predicate('0', n));
                    } else if o > z {
                        oxygen.retain(create_predicate('1', n)); 
                    } else if o == z {
                        oxygen.retain(create_predicate('1', n));
                    }
                }
                n += 1;
            }
            oxygen.get(0).unwrap().clone()
        });
        let co2_thread = s.spawn(|_| {
            let mut co2 = splitted_contents.clone();
            let mut n = 0;
            while co2.len() > 1 {
                if co2.len() > 1 {
                    let (z, o) = get_ratings_of_bits(&co2, n);
                    if z > o {
                        co2.retain(create_predicate('1', n));
                    } else if o > z {
                        co2.retain(create_predicate('0', n));
                    } else if o == z {
                        co2.retain(create_predicate('0', n));
                    }
                }
                n += 1;
            }
            co2.get(0).unwrap().clone()
        });
        let oxygen_decimal = isize::from_str_radix(oxygen_thread.join().unwrap(), 2).unwrap();
        let co2_decimal = isize::from_str_radix(co2_thread.join().unwrap(), 2).unwrap();
        (oxygen_decimal * co2_decimal).try_into().unwrap()
    }).unwrap()
}

/// Returns a tuple which shows how many of each bit is represtened. (zeros, ones)
fn get_ratings_of_bits(binaries: &Vec<&str>, index: usize) -> (i32, i32) {
    let mut zeros = 0;
    let mut ones = 0;
    binaries.iter().for_each(|&binary| {
        match binary.chars().collect::<Vec<char>>().get(index).unwrap() {
            '1' => ones += 1,
            '0' => zeros += 1,
            _ => panic!("Not recognizeable character"),
        }
    });
    (zeros, ones)
}

fn create_predicate(character: char, n: usize) -> impl FnMut(&&str) -> bool {
    move |x: &&str| {
        return x.chars().collect::<Vec<char>>().get(n).unwrap() == &character;
    }
}
#[test]
fn test_get_ratings_from_bits() {
    let insert: Vec<&str> = vec![
        "000100011010",
        "110011110110",
        "011000101111",
        "001101100101",
        "011100001000",
        "101101011011",
        "101111010101",
        "011010000101",
        "010101000010",
        "100001111000",
        "111011111100",
    ];

    let (zeroes, ones) = get_ratings_of_bits(&insert, 0);
    assert_eq!(zeroes, 6);
    assert_eq!(ones, 5);
}

#[test]
fn test_part2() {
    let inputs = vec![
        "00100", "11110", "10110", "10111", "10101", "01111", "00111", "11100", "10000", "11001",
        "00010", "01010",
    ];
    let part_two = part_two(inputs.join("\n"));
    assert_eq!(part_two, 230);
}
