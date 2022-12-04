mod days;
mod util;

use days::day01::Day01;
use days::day02::Day02;
use days::day03::Day03;
use days::day04::Day04;
use days::Problem;

use std::time::Instant;

fn main() {
    let day = std::env::args().nth(1).expect("no days given");

    run_day(&day);
}

fn run_day(day: &str) {
    match day {
        "1" => run(Box::new(Day01 {}), 1),
        "2" => run(Box::new(Day02 {}), 2),
        "3" => run(Box::new(Day03 {}), 3),
        "4" => run(Box::new(Day04 {}), 4),
        _ => panic!("day {} not implemented", day),
    }
}

fn run(problem: Box<dyn Problem>, day: u8) {
    let now = Instant::now();
    let part_one = problem.part_one(&util::input(day, util::InputType::Real));
    let part_two = problem.part_two(&util::input(day, util::InputType::Real));
    let elapsed = now.elapsed().as_millis();

    println!("Day {} Part 1: {}", day, part_one);
    println!("Day {} Part 2: {}", day, part_two);
    println!("Elapsed time: {} milliseconds", elapsed);
}