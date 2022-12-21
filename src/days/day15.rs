use crate::days::Problem;

pub struct Day15 {}

impl Problem for Day15 {
    fn part_one(&self, input: &str) -> String {
        format!("not implemented")
    }

    fn part_two(&self, input: &str) -> String {
        format!("not implemented")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::util;

    fn test_part_one() {
        assert_eq!(
            "26",
            Day15 {}.part_one(&util::input(15, util::InputType::Example))
        )
    }
}
