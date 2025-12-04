use crate::day_1::day_1;
use crate::day_2::day_2;
use crate::day_3::day_3;
use crate::day_4::day_4;

mod day_1;
mod day_2;
mod day_3;
mod day_4;

fn main() {
    println!("day 1, part 1: {:?}", day_1(true));
    println!("day 1, part 2: {:?}", day_1(false));
    println!("day 2, part 1: {:?}", day_2(false));
    println!("day 2, part 2: {:?}", day_2(true));
    println!("day 3, part 1: {:?}", day_3(false));
    println!("day 3, part 2: {:?}", day_3(true));
    println!("day 4, part 1: {:?}", day_4(false));
    println!("day 4, part 2: {:?}", day_4(true));
}
