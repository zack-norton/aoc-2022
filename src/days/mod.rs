pub mod day01;
pub mod day02;
pub mod day03;
pub mod day04;

pub trait Problem {
    fn part_one(&self, input: &str) -> String;
    fn part_two(&self, input: &str) -> String;
}
