use std::fs;

fn get_overlap(r1: (u64, u64), r2: (u64, u64)) -> Option<(u64, u64)> {
    if r1.0.max(r2.0) <= r1.1.min(r2.1) {
        Some((r1.0.min(r2.0), r1.1.max(r2.1)))
    } else {
        None
    }
}

pub fn day_5(part2: bool) -> u64 {
    let data: Vec<String> = fs::read_to_string("./puzzle_input/day_5")
        .expect("Can't read day_5 puzzle")
        .split("\n")
        .map(|x| x.to_string())
        .collect();

    let split_index = data.iter().position(|x| x.is_empty()).unwrap();

    let (fresh_id_range, avalible_id) = data.split_at(split_index);
    if !part2 {
        let out: u64 = avalible_id
            .iter()
            .filter(|x| !x.is_empty())
            .map(|x| {
                let id = x.parse::<u64>().unwrap();

                let id_check: u64 = fresh_id_range
                    .iter()
                    .map(|y| {
                        let range: Vec<u64> =
                            y.split("-").map(|z| z.parse::<u64>().unwrap()).collect();
                        if range[0] <= id && id <= range[1] {
                            1
                        } else {
                            0
                        }
                    })
                    .sum();

                if id_check > 0 { 1 } else { 0 }
            })
            .sum();
        out
    } else {
        let mut out: Vec<(u64, u64)> = fresh_id_range
            .iter()
            .map(|x| {
                let range: Vec<u64> = x.split("-").map(|z| z.parse::<u64>().unwrap()).collect();
                (range[0], range[1])
            })
            .collect();

        loop {
            let mut any_overlapping = false;
            for i in 0..out.len() {
                for j in 0..out.len() {
                    if i != j {
                        if i < out.len() && j < out.len() {
                            match get_overlap(out[i], out[j]) {
                                Some(r) => {
                                    out.remove(i.max(j));
                                    out.remove(i.min(j));
                                    out.push(r);
                                    any_overlapping = true;
                                }
                                None => {}
                            }
                        }
                    }
                }
            }

            if !any_overlapping {
                break;
            }
        }

        out.iter().map(|x| x.1 - x.0 + 1).sum()
    }
}
