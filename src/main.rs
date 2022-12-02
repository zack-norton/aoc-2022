mod days;
mod util;

use days::day01::Day01;
use days::day02::Day02;

fn main() {
    let part_1_result = Day01::part_one(&util::input(1, util::InputType::Real));
    let part_2_result = Day01::part_two(&util::input(1, util::InputType::Real));

    let day02_part1 = Day02::part_one(&util::input(2, util::InputType::Real));
    let day02_part2 = Day02::part_two(&util::input(2, util::InputType::Real));

    println!("Day 01 Part 1: {}", part_1_result);
    println!("Day 01 Part 2: {}", part_2_result);
    println!("Day 02 Part 1: {}", day02_part1);
    println!("Day 02 Part 2: {}", day02_part2);
}
