use crate::days::Problem;

pub struct Day04 {}

impl Problem for Day04 {
    fn part_one(&self, input: &str) -> String {
        let mut fully_contained = 0;
        for line in input.lines() {
            let parts = line.split(",").collect::<Vec<&str>>();
            let part_one_min = parts[0].split("-").collect::<Vec<&str>>()[0]
                .parse::<u32>()
                .unwrap();
            let part_one_max = parts[0].split("-").collect::<Vec<&str>>()[1]
                .parse::<u32>()
                .unwrap();

            let part_two_min = parts[1].split("-").collect::<Vec<&str>>()[0]
                .parse::<u32>()
                .unwrap();
            let part_two_max = parts[1].split("-").collect::<Vec<&str>>()[1]
                .parse::<u32>()
                .unwrap();

            if part_one_min <= part_two_min && part_one_max >= part_two_max {
                //part one contains part 2
                fully_contained += 1;
            } else if part_two_min <= part_one_min && part_two_max >= part_one_max {
                fully_contained += 1;
            }
        }

        fully_contained.to_string()
    }

    fn part_two(&self, input: &str) -> String {
        let mut overlap = 0;
        for line in input.lines() {
            let parts = line.split(",").collect::<Vec<&str>>();
            let part_one_min = parts[0].split("-").collect::<Vec<&str>>()[0]
                .parse::<u32>()
                .unwrap();
            let part_one_max = parts[0].split("-").collect::<Vec<&str>>()[1]
                .parse::<u32>()
                .unwrap();

            let part_two_min = parts[1].split("-").collect::<Vec<&str>>()[0]
                .parse::<u32>()
                .unwrap();
            let part_two_max = parts[1].split("-").collect::<Vec<&str>>()[1]
                .parse::<u32>()
                .unwrap();

            if part_one_min <= part_two_min {
                if part_one_max >= part_two_min {
                    overlap += 1;
                }
            } else if part_two_min <= part_one_min {
                if part_two_max >= part_one_min {
                    overlap += 1;
                }
            }
        }

        overlap.to_string()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::util;

    #[test]
    fn test_part_one() {
        assert_eq!(
            "2",
            Day04 {}.part_one(&util::input(4, util::InputType::Example))
        )
    }

    #[test]
    fn test_part_two() {
        assert_eq!(
            "4",
            Day04 {}.part_two(&util::input(4, util::InputType::Example))
        )
    }
}
