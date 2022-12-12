mod days;
mod util;

use days::day01::Day01;
use days::day02::Day02;
use days::day03::Day03;
use days::day04::Day04;
use days::day05::Day05;
use days::day06::Day06;
use days::day07::Day07;
use days::day08::Day08;
use days::day09::Day09;
use days::day10::Day10;
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
        "5" => run(Box::new(Day05 {}), 5),
        "6" => run(Box::new(Day06 {}), 6),
        "7" => run(Box::new(Day07 {}), 7),
        "8" => run(Box::new(Day08 {}), 8),
        "9" => run(Box::new(Day09 {}), 9),
        "10" => run(Box::new(Day10 {}), 10),
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
