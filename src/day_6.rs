use std::{fs, panic};

pub fn day_6(part2: bool) -> u64 {
    let mut puzzle_input: Vec<String> = fs::read_to_string("./puzzle_input/day_6")
        .expect("Can't read day 6 puzzle input")
        .split("\n")
        .map(|x| x.to_string())
        .filter(|x| !x.is_empty())
        .collect();

    let operation_string = puzzle_input.pop().unwrap().clone();
    let operation = operation_string.split_whitespace().collect::<Vec<&str>>();

    if !part2 {
        puzzle_input
            .iter()
            .fold(Vec::<u64>::new(), |acc, x| {
                x.split_whitespace()
                    .map(|y| y.parse::<u64>().unwrap())
                    .enumerate()
                    .map(|(i, y)| {
                        if acc.len() == 0 {
                            y
                        } else {
                            match operation[i] {
                                "+" => y + acc[i],
                                "*" => y * acc[i],
                                _ => {
                                    panic!("uknown operation")
                                }
                            }
                        }
                    })
                    .collect::<Vec<u64>>()
            })
            .iter()
            .sum()
    } else {
        todo!()
    }
}
