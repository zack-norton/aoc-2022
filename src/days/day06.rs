use crate::days::Problem;

use std::collections::HashSet;

pub struct Day06 {}

impl Problem for Day06 {
    fn part_one(&self, input: &str) -> String {
        let chars = input.chars().collect::<Vec<char>>();
        let mut start: usize = 0;
        let mut end: usize = 4;
        let mut done = false;

        while !done {
            let slice = &chars[start..end];

            let set: HashSet<&char> = HashSet::from_iter(slice);

            if set.len() == 4 {
                done = true;
            } else {
                start += 1;
                end += 1;
            }
        }

        end.to_string()
    }

    fn part_two(&self, input: &str) -> String {
        let chars = input.chars().collect::<Vec<char>>();
        let mut start: usize = 0;
        let mut end: usize = 14;
        let mut done = false;

        while !done {
            let slice = &chars[start..end];

            let set: HashSet<&char> = HashSet::from_iter(slice);

            if set.len() == 14 {
                done = true;
            } else {
                start += 1;
                end += 1;
            }
        }

        end.to_string()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::util;

    #[test]
    fn test_part_one() {
        assert_eq!(
            "7",
            Day06 {}.part_one(&util::input(6, util::InputType::Example))
        )
    }

    #[test]
    fn test_part_two() {
        assert_eq!(
            "19",
            Day06 {}.part_two(&util::input(6, util::InputType::Example))
        )
    }
}
