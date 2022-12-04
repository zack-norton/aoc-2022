use crate::days::Problem;

use std::collections::HashSet;

pub struct Day03 {}

impl Problem for Day03 {
    fn part_one(&self, input: &str) -> String {
        let mut sum = 0;
        for line in input.lines() {
            let half = line.len() / 2;

            let front: HashSet<char> = HashSet::from_iter(line.split_at(half).0.chars());
            let back: HashSet<char> = HashSet::from_iter(line.split_at(half).1.chars());

            for c in front.iter() {
                if back.contains(c) {
                    sum += get_priority(c);
                    break;
                }
            }
        }

        sum.to_string()
    }

    fn part_two(&self, input: &str) -> String {
        let bags: Vec<&str> = input.lines().collect();
        let mut sum = 0;

        let mut i = 0;
        while i < bags.len() {
            let bag1: HashSet<char> = HashSet::from_iter(bags[i].chars());
            let bag2: HashSet<char> = HashSet::from_iter(bags[i + 1].chars());
            let bag3: HashSet<char> = HashSet::from_iter(bags[i + 2].chars());

            for c in bag1.iter() {
                if bag2.contains(c) && bag3.contains(c) {
                    sum += get_priority(c);
                    break;
                }
            }

            i += 3;
        }

        sum.to_string()
    }
}

fn get_priority(c: &char) -> u32 {
    if c.is_uppercase() {
        *c as u32 - 38
    } else {
        *c as u32 - 96
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::util;

    #[test]
    fn test_part_one() {
        assert_eq!(
            "157",
            Day03 {}.part_one(&util::input(3, util::InputType::Example))
        )
    }

    #[test]
    fn test_part_two() {
        assert_eq!(
            "70",
            Day03 {}.part_two(&util::input(3, util::InputType::Example))
        )
    }
}
