use std::fs;

// find the largest value between 2 indices (inclusive)
// return value and index where the value is found (relative to entire array)
fn find_max_in(v: &[u64], start_index: usize, end_index: usize) -> (usize, &u64) {
    v.iter()
        .enumerate()
        .reduce(|acc, x| {
            if start_index == x.0 {
                x
            } else if start_index < x.0 && x.0 <= end_index {
                if x.1 > acc.1 { x } else { acc }
            } else {
                acc
            }
        })
        .unwrap()
}

pub fn day_3(part2: bool) -> u64 {
    let out: u64 = fs::read_to_string("./puzzle_input/day_3")
        .expect("Can't read day 3 file")
        .split("\n")
        .filter(|x| !x.is_empty())
        .map(|x| {
            let char_vec: Vec<u64> = x
                .chars()
                .map(|c| c.to_string().parse::<u64>().unwrap())
                .collect();
            if !part2 {
                // finding largest value and index (not including last element)
                let max = find_max_in(&char_vec, 0, char_vec.len() - 2);

                // finding largest value for index > index(max)
                let second_max = find_max_in(&char_vec, max.0 + 1, char_vec.len() - 1);

                let v = format!("{}{}", max.1, second_max.1).parse::<u64>().unwrap();
                v
            } else {
                let mut current_start = 0;
                let mut out = Vec::<String>::new();
                for i in 0..12 {
                    let temp = find_max_in(&char_vec, current_start, char_vec.len() - 12 + i);
                    out.push((*temp.1).to_string());
                    current_start = temp.0 + 1;
                }
                out.join("").parse::<u64>().unwrap()
            }
        })
        .sum();
    out
}
