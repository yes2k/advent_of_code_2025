use std::{char, fs, panic};

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
        let math_nums: Vec<Vec<&str>> = puzzle_input
            .iter()
            .map(|x| x.split_whitespace().collect::<Vec<&str>>())
            .collect();

        let mut max_lengths = Vec::<usize>::new();
        for i in 0..math_nums[0].len() {
            max_lengths.push(math_nums.iter().map(|x| x[i].len()).max().unwrap());
        }

        let out: Vec<Vec<String>> = puzzle_input
            .iter()
            .map(|x| x.to_string())
            .map(|x| {
                let mut start_index = 0;
                let mut end_index = 0;
                let mut split_strs = Vec::<String>::new();

                for i in 0..max_lengths.len() {
                    start_index = end_index;
                    end_index = end_index + max_lengths[i];
                    split_strs.push(x[start_index..end_index].to_string());
                    end_index += 1;
                }

                split_strs
            })
            .collect();

        let mut total_sum: u64 = 0;
        for i in 0..math_nums[0].len() {
            let to_operate: Vec<String> = out.iter().map(|x| x[i].to_string()).collect();

            let summands: Vec<u64> = (0..max_lengths[i])
                .map(|m| {
                    to_operate
                        .iter()
                        .map(|s| s.chars().nth(m).unwrap())
                        .collect::<String>()
                        .trim()
                        .to_string()
                        .parse::<u64>()
                        .unwrap()
                })
                .collect();

            total_sum += match operation[i] {
                "*" => summands.iter().product::<u64>(),
                "+" => summands.iter().sum::<u64>(),
                _ => 0,
            };
        }

        total_sum
    }
}
