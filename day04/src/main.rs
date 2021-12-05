use std::{collections::HashMap, env, panic, time::Instant};

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

#[derive(Debug)]
struct Bingo {
    board: Vec<Vec<u8>>,
    inputs: HashMap<u8, bool>,
}

impl Bingo {
    pub fn new(board_inputs: Vec<String>) -> Bingo {
        let mut number_inputs = HashMap::new();
        let rows: Vec<Vec<u8>> = board_inputs
            .clone()
            .iter()
            .map(|row| {
                row.split(" ")
                    .filter(|c| !c.trim().is_empty())
                    .map(|field| {
                        let parsed = field.trim().parse::<u8>().unwrap();
                        number_inputs.insert(parsed, false);
                        parsed
                    })
                    .collect()
            })
            .collect();
        Bingo {
            board: rows,
            inputs: number_inputs,
        }
    }

    pub fn add_input(&mut self, input: u8) {
        self.inputs.insert(input, true);
    }

    fn are_we_winning(&self) -> bool {
        if self.inputs.len() < 5 {
            return false;
        }

        for n in 0..self.board.len() {
            let row = self
                .board
                .get(n)
                .unwrap()
                .iter()
                .filter(|number| *self.inputs.get(number).unwrap())
                .collect::<Vec<&u8>>();

            if row.len() == 5 {
                return true;
            }

            let column = self
                .board
                .iter()
                .map(|row| row.get(n).unwrap())
                .filter(|number| *self.inputs.get(number).unwrap())
                .collect::<Vec<&u8>>();

            if column.len() == 5 {
                return true;
            }
        }
        false
    }

    pub fn get_all_unused(&self) -> Vec<u8> {
        let mut return_vector: Vec<u8> = vec![];
        for (key, value) in self.inputs.iter() {
            if !value {
                return_vector.push(*key);
            }
        }

        return_vector
    }
}

fn part_one(contents: String) -> u64 {
    let mut list = contents.split("\n").collect::<Vec<&str>>();
    let inputs_numbers = get_number_inputs(&list);
    list.retain(|x| {
        let q = x.trim();
        !q.is_empty()
    });
    let board_numbers = list[1..].to_owned();
    let mut boards = board_numbers
        .chunks(5)
        .map(|chunk| {
            let vec_of_strings = chunk
                .to_vec()
                .iter()
                .map(|&s| s.into())
                .collect::<Vec<String>>();
            Bingo::new(vec_of_strings)
        })
        .collect::<Vec<Bingo>>();

    for s in inputs_numbers.iter() {
        let n = s.parse::<u8>().unwrap();
        for b in boards.iter_mut() {
            b.add_input(n);
            if b.are_we_winning() {
                let sum: i32 = b.get_all_unused().iter().map(|n| *n as i32).sum();
                return (sum * n as i32).try_into().unwrap();
            }
        }
    }
    0
}

fn part_two(contents: String) -> u64 {
    let mut list = contents.split("\n").collect::<Vec<&str>>();
    let inputs_numbers = get_number_inputs(&list);
    list.retain(|x| {
        let q = x.trim();
        !q.is_empty()
    });
    let board_numbers = list[1..].to_owned();
    let mut boards = board_numbers
        .chunks(5)
        .map(|chunk| {
            let vec_of_strings = chunk
                .to_vec()
                .iter()
                .map(|&s| s.into())
                .collect::<Vec<String>>();
            Bingo::new(vec_of_strings)
        })
        .collect::<Vec<Bingo>>();
    for s in inputs_numbers.iter() {
        let n = s.parse::<u8>().unwrap();
        let length_of_boards = boards.len();
        let mut i: usize = 0;
        while i < length_of_boards {
            let mut shame = 1;
            if let Some(b) = boards.get_mut(i) {
                b.add_input(n);
                if b.are_we_winning() {
                    if length_of_boards == 1 {
                        let sum: i32 = b.get_all_unused().iter().map(|number| *number as i32).sum();
                        return (sum * n as i32).try_into().unwrap();
                    } else {
                        boards.remove(i);
                        shame -= 1;
                    }
                }
            }
            i += shame;
        }
    }
    0
}

fn get_number_inputs(splitted_contents: &Vec<&str>) -> Vec<String> {
    let inputs = splitted_contents.get(0).unwrap().clone();
    inputs.split(",").map(|i| String::from(i)).collect()
}

#[test]
fn parse_input() {
    let input = "7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

22 13 17 11  0
 8  2 23  4 24
21  9 14 16  7
 6 10  3 18  5
 1 12 20 15 19
    
 3 15  0  2 22
 9 18 13 17  5
19  8  7 25 23
20 11 10 24  4
14 21 16 12  6
    
14 21 17 24  4
10 16 15  9 19
18  8 23 26 20
22 11 13  6  5
 2  0 12  3  7";

    let splitted_contents = input.split("\n").collect::<Vec<&str>>();
    let from_inputs = get_number_inputs(&splitted_contents);
    assert_eq!(
        from_inputs,
        vec![
            "7", "4", "9", "5", "11", "17", "23", "2", "0", "14", "21", "24", "10", "16", "13",
            "6", "15", "25", "12", "22", "18", "20", "8", "19", "3", "26", "1"
        ]
    );
    assert_eq!(part_one(input.into()), 0);
}

#[test]
fn test_parsing_board() {
    let inputs = "22 13 17 11  0
 8  2 23  4 24
21  9 14 16  7
 6 10  3 18  5
 1 12 20 15 19  
 3 15  0  2 22
 9 18 13 17  5
19  8  7 25 23
20 11 10 24  4
14 21 16 12  6
14 21 17 24  4
10 16 15  9 19
18  8 23 26 20
22 11 13  6  5
 2  0 12  3  7";

    let splitted_contents = inputs
        .split("\n")
        .map(|r| r.into())
        .collect::<Vec<String>>();
    let chunks = splitted_contents.chunks(5);
    let boards = chunks
        .map(|chunk| Bingo::new(chunk.to_vec()))
        .collect::<Vec<Bingo>>();
    let first_board = &boards.get(0).unwrap().board;
    assert_eq!(
        first_board,
        &vec![
            vec![22, 13, 17, 11, 0],
            vec![8, 2, 23, 4, 24],
            vec![21, 9, 14, 16, 7],
            vec![6, 10, 3, 18, 5],
            vec![1, 12, 20, 15, 19]
        ]
    );
}

#[test]
fn test_winning_in_row() {
    let mut bingo = Bingo::new(vec![
        String::from("22 13 17 11  0"),
        String::from(" 8  2 23  4 24"),
        String::from("21  9 14 16  7"),
        String::from(" 6 10  3 18  5"),
        String::from("1 12 20 15 19"),
    ]);
    bingo.add_input(8);
    bingo.add_input(2);
    bingo.add_input(23);
    bingo.add_input(4);
    bingo.add_input(24);

    let winner = bingo.are_we_winning();
    assert_eq!(winner, true);
}

#[test]
fn test_winning_on_last_row() {
    let mut bingo = Bingo::new(vec![
        String::from("22 13 17 11  0"),
        String::from(" 8  2 23  4 24"),
        String::from("21  9 14 16  7"),
        String::from(" 6 10  3 18  5"),
        String::from("1 12 20 15 19"),
    ]);
    bingo.add_input(1);
    bingo.add_input(12);
    bingo.add_input(8);
    bingo.add_input(23);
    bingo.add_input(20);
    bingo.add_input(15);
    bingo.add_input(19);

    let winner = bingo.are_we_winning();
    assert_eq!(winner, true);
}

#[test]
fn test_winning_in_first_column() {
    let mut bingo = Bingo::new(vec![
        String::from("22 13 17 11  0"),
        String::from(" 8  2 23  4 24"),
        String::from("21  9 14 16  7"),
        String::from(" 6 10  3 18  5"),
        String::from("1 12 20 15 19"),
    ]);
    bingo.add_input(8);
    bingo.add_input(22);
    bingo.add_input(21);
    bingo.add_input(6);
    bingo.add_input(1);

    let winner = bingo.are_we_winning();
    assert_eq!(winner, true);
}

#[test]
fn test_winning_in_third_column() {
    let mut bingo = Bingo::new(vec![
        String::from("22 13 17 11  0"),
        String::from(" 8  2 23  4 24"),
        String::from("21  9 14 16  7"),
        String::from(" 6 10  3 18  5"),
        String::from("1 12 20 15 19"),
    ]);
    bingo.add_input(3);
    bingo.add_input(14);
    bingo.add_input(17);
    bingo.add_input(23);
    bingo.add_input(20);

    let winner = bingo.are_we_winning();
    assert_eq!(winner, true);
}

#[test]
fn test_should_not_win() {
    let mut bingo = Bingo::new(vec![
        String::from("22 13 17 11  0"),
        String::from(" 8  2 23  4 24"),
        String::from("21  9 14 16  7"),
        String::from(" 6 10  3 18  5"),
        String::from("1 12 20 15 19"),
    ]);
    bingo.add_input(3);
    bingo.add_input(14);
    bingo.add_input(17);
    bingo.add_input(11);
    bingo.add_input(33);
    bingo.add_input(1);

    let winner = bingo.are_we_winning();
    assert_eq!(winner, false);
}

#[test]
fn test_should_win_and_calucate_correctly() {
    let mut bingo = Bingo::new(vec![
        String::from("22 13 17 11  0"),
        String::from(" 8  2 23  4 24"),
        String::from("21  9 14 16  7"),
        String::from(" 6 10  3 18  5"),
        String::from("1 12 20 15 19"),
    ]);
    bingo.add_input(22);
    bingo.add_input(13);
    bingo.add_input(17);
    bingo.add_input(11);
    bingo.add_input(0);

    let winner = bingo.are_we_winning();
    assert_eq!(winner, true);

    let expected_sum = 237;
    let unused: i32 = bingo.get_all_unused().iter().map(|n| *n as i32).sum();
    assert_eq!(unused, expected_sum);
}

#[test]
fn test_part_two() {
    let inputs = "7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

    22 13 17 11  0
     8  2 23  4 24
    21  9 14 16  7
     6 10  3 18  5
     1 12 20 15 19
    
     3 15  0  2 22
     9 18 13 17  5
    19  8  7 25 23
    20 11 10 24  4
    14 21 16 12  6
    
    14 21 17 24  4
    10 16 15  9 19
    18  8 23 26 20
    22 11 13  6  5
     2  0 12  3  7";

    let q = part_two(inputs.into());
    assert_eq!(q, 1924);
}
