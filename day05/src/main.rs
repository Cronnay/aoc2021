use std::{env, panic, time::Instant};

struct Coords {
    x: u32,
    y: u32,
}

impl Coords {
    pub fn new(coords: &str) -> Coords {
        let splitted = coords.split(",").collect::<Vec<&str>>();
        Coords {
            x: splitted.get(0).unwrap().parse::<u32>().unwrap(),
            y: splitted.get(1).unwrap().parse::<u32>().unwrap(),
        }
    }
}

struct Line {
    from: Coords,
    to: Coords,
}

impl Line {
    pub fn new(row: &str) -> Line {
        let split = row.split("->").collect::<Vec<&str>>();
        let from_string = split.get(0).unwrap().trim();
        let to_string = split.get(1).unwrap().trim();

        Line {
            from: Coords::new(from_string),
            to: Coords::new(to_string),
        }
    }

    pub fn is_horizontal_or_vertical(&self) -> bool {
        self.from.x == self.to.x || self.from.y == self.to.y
    }

    pub fn get_points(&self) -> Vec<Coords> {
        let mut x_range = {
            if self.from.x < self.to.x {
                (self.from.x..=self.to.x).collect::<Vec<u32>>()
            } else {
                (self.to.x..=self.from.x).rev().collect::<Vec<u32>>()
            }
        };
        let mut y_range = {
            if self.from.y < self.to.y {
                (self.from.y..=self.to.y).collect::<Vec<u32>>()
            } else {
                (self.to.y..=self.from.y).rev().collect::<Vec<u32>>()
            }
        };

        let min_length = usize::max(x_range.len(), y_range.len());
        while x_range.len() != min_length {
            x_range.push(*x_range.first().unwrap());
        }

        while y_range.len() != min_length {
            y_range.push(*y_range.first().unwrap());
        }

        let mut return_vec: Vec<Coords> = vec![];
        for n in 0..min_length {
            return_vec.push(Coords {
                x: *x_range.get(n).unwrap(),
                y: *y_range.get(n).unwrap(),
            });
        };
        return_vec
    }
}

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

fn part_one(content: String) -> u64 {
    let mut field = vec![vec![0u32; 1000]; 1000];

    let splitted_contents = content.split("\n").collect::<Vec<&str>>();
    let lines = splitted_contents
        .iter()
        .map(|input| Line::new(input))
        .filter(|l| l.is_horizontal_or_vertical())
        .collect::<Vec<Line>>();
    lines.iter().for_each(|line| {
        line.get_points().iter().for_each(|point| {
            field[point.y as usize][point.x as usize] += 1;
        });
    });
    field.iter().flat_map(|i| i.iter()).filter(|&&i| i >= 2).collect::<Vec<&u32>>().len() as u64
}

fn part_two(content: String) -> u64 {
    let mut field = vec![vec![0u32; 1000]; 1000];

    let splitted_contents = content.split("\n").collect::<Vec<&str>>();
    let lines = splitted_contents
        .iter()
        .map(|input| Line::new(input))
        .collect::<Vec<Line>>();
    lines.iter().for_each(|line| {
        line.get_points().iter().for_each(|point| {
            field[point.y as usize][point.x as usize] += 1;
        });
    });
    field.iter().flat_map(|i| i.iter()).filter(|&&i| i >= 2).collect::<Vec<&u32>>().len() as u64
}

#[test]
fn test_parse_line() {
    let input = "559,436 -> 133,862";
    let line = Line::new(input);
    assert_eq!(line.from.x, 559);
    assert_eq!(line.from.y, 436);
    assert_eq!(line.to.x, 133);
    assert_eq!(line.to.y, 862);
}

#[test]
fn test_part_one() {
    let input = "0,9 -> 5,9
8,0 -> 0,8
9,4 -> 3,4
2,2 -> 2,1
7,0 -> 7,4
6,4 -> 2,0
0,9 -> 2,9
3,4 -> 1,4
0,0 -> 8,8
5,5 -> 8,2".to_string();
    let resp = part_one(input);
    assert_eq!(resp, 5);
}

#[test]
fn test_part_two() {
    let input = "0,9 -> 5,9
8,0 -> 0,8
9,4 -> 3,4
2,2 -> 2,1
7,0 -> 7,4
6,4 -> 2,0
0,9 -> 2,9
3,4 -> 1,4
0,0 -> 8,8
5,5 -> 8,2".to_string();
    let resp = part_two(input);
    assert_eq!(resp, 12);
}