use std::char;
use std::fs;

pub fn day_1(part1: bool) -> usize {
    let mut pass_by_zero: usize = 0;
    let dial_rots = fs::read_to_string("./puzzle_input/day_1")
        .expect("Can't read day 1 puzzle file")
        .split("\n")
        .filter(|x| !x.is_empty())
        .map(|x| {
            let char_vec: Vec<char> = x.chars().collect();
            match char_vec[0] {
                'L' => {
                    let num_rot: String = char_vec[1..].iter().collect();
                    -1 * (num_rot.parse::<i64>().unwrap())
                }
                'R' => {
                    let num_rot: String = char_vec[1..].iter().collect();
                    num_rot.parse::<i64>().unwrap()
                }
                _ => {
                    panic!("Not L or R");
                }
            }
        })
        .fold(vec![50], |mut acc, x| {
            let rot_distance = acc.last().unwrap() + x;
            let last_elem = *acc.last().unwrap();

            // part 2
            let mut curr_num = last_elem;
            for _ in 1..=x.abs() {
                curr_num += x.signum();
                if curr_num == -1 {
                    curr_num = 99;
                }
                if curr_num == 100 {
                    curr_num = 0;
                }

                if curr_num == 0 {
                    pass_by_zero += 1;
                }
            }

            acc.push(rot_distance.rem_euclid(100));
            acc
        });

    if part1 {
        dial_rots.iter().filter(|&x| *x == 0).count()
    } else {
        pass_by_zero
    }
}
