use std::cmp;
use std::collections::BinaryHeap;

use crate::days::Problem;

pub struct Day01 {}

impl Problem for Day01 {
    fn part_one(&self, input: &str) -> String {
        let content: Vec<&str> = input.split("\n").collect();

        let mut max_calories: u32 = 0;
        let mut current_max: u32 = 0;
        for line in content.iter() {
            if line.is_empty() {
                max_calories = cmp::max(max_calories, current_max);
                current_max = 0;
            } else {
                current_max += line.parse::<u32>().unwrap();
            }
        }

        max_calories.to_string()
    }

    fn part_two(&self, input: &str) -> String {
        let content: Vec<&str> = input.split("\n").collect();

        let mut priority_queue = BinaryHeap::new();
        let mut current_calories: u32 = 0;

        for line in content.iter() {
            if line.is_empty() {
                priority_queue.push(current_calories);
                current_calories = 0;
            } else {
                current_calories += line.parse::<u32>().unwrap();
            }
        }

        (priority_queue.pop().unwrap()
            + priority_queue.pop().unwrap()
            + priority_queue.pop().unwrap())
        .to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::util;

    #[test]
    fn test_part_one() {
        assert_eq!(
            "24000",
            Day01 {}.part_one(&util::input(1, util::InputType::Example))
        )
    }

    #[test]
    fn test_part_two() {
        assert_eq!(
            "45000",
            Day01 {}.part_two(&util::input(1, util::InputType::Example))
        )
    }
}
