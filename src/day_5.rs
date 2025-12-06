use std::fs;

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
        let mut out: Vec<u64> = fresh_id_range
            .iter()
            .flat_map(|x| {
                let range: Vec<u64> = x.split("-").map(|z| z.parse::<u64>().unwrap()).collect();
                range_vec
            })
            .collect();

        out.sort();
        out.dedup();
        out.len() as u64
    }
}
