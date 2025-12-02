use std::fs;

pub fn day_2(part2: bool) -> u64 {
    let out: u64 = fs::read_to_string("./puzzle_input/day_2")
        .expect("Can't read day 2 puzzle file")
        .split(",")
        .map(|x| x.replace("\n", ""))
        .map(|x| {
            let id_range: Vec<&str> = x.split("-").collect();
            let id_range_start: u64 = id_range[0].parse().unwrap();
            let id_range_end: u64 = id_range[1].parse().unwrap();

            let invalid_ids: u64 = (id_range_start..=id_range_end)
                .map(|y| {
                    let y_s = y.to_string();
                    if !part2 {
                        if y_s.len() % 2 == 0 {
                            let (s1, s2) = y_s.split_at(y_s.len() / 2);
                            if s1 == s2 { y } else { 0 }
                        } else {
                            0
                        }
                    } else {
                        let mut out = Vec::<u64>::new();
                        for i in 1..=y_s.len() / 2 as usize {
                            if y_s.len() % i == 0 {
                                let (sub_id, _) = y_s.split_at(i);
                                if sub_id.repeat((y_s.len() / i) as usize) == y_s {
                                    out.push(y);
                                    break;
                                }
                            }
                        }
                        out.iter().sum()
                    }
                })
                .sum();

            invalid_ids
        })
        .sum();
    out
}
