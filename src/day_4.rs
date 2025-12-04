use std::fs;

pub fn check_adj(grid: &Vec<Vec<bool>>, row: usize, col: usize) -> bool {
    let mut adjacent_rolls = 0;
    for i in -1..=1 {
        for j in -1..=1 {
            if row as i64 + i < 0 {
                continue;
            }
            if col as i64 + j < 0 {
                continue;
            }
            if i == 0 && j == 0 {
                continue;
            }

            let row_index = (row as i64 + i) as usize;
            let col_index = (col as i64 + j) as usize;

            match grid.get(row_index) {
                Some(x) => match x.get(col_index) {
                    Some(y) => {
                        if *y {
                            adjacent_rolls += 1
                        }
                    }
                    None => {}
                },
                None => {}
            }
        }
    }
    adjacent_rolls < 4
}

pub fn day_4(part2: bool) -> u64 {
    // grid[row][col]
    // True if contains forklift, False if empty
    let mut grid: Vec<Vec<bool>> = fs::read_to_string("./puzzle_input/day_4")
        .expect("Can't read day 4 puzzle input")
        .split("\n")
        .filter(|x| !x.is_empty())
        .map(|x| {
            x.chars()
                .map(|y| if y == '@' { true } else { false })
                .collect()
        })
        .collect();

    if !part2 {
        let mut accesible_by_forklift = 0;
        for row in 0..grid.len() {
            for col in 0..grid[0].len() {
                if grid[row][col] {
                    if check_adj(&grid, row, col) {
                        accesible_by_forklift += 1;
                    }
                }
            }
        }

        accesible_by_forklift
    } else {
        let mut accesible_by_forklift = 0;
        loop {
            let mut curr_accesible_by_forklift = 0;
            let mut paper_rolls_to_remove = Vec::<(usize, usize)>::new();
            for row in 0..grid.len() {
                for col in 0..grid[0].len() {
                    if grid[row][col] {
                        if check_adj(&grid, row, col) {
                            curr_accesible_by_forklift += 1;
                            paper_rolls_to_remove.push((row, col));
                        }
                    }
                }
            }

            accesible_by_forklift += curr_accesible_by_forklift;
            if curr_accesible_by_forklift == 0 {
                break;
            }

            paper_rolls_to_remove.iter().for_each(|x| {
                grid[x.0][x.1] = false;
            });
        }
        accesible_by_forklift
    }
}
