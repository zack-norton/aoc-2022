use crate::days::Problem;

pub struct Day05 {}

impl Problem for Day05 {
    fn part_one(&self, input: &str) -> String {
        let mut read_stacks = true;
        let mut crate_rows: Vec<Vec<char>> = vec![];
        let mut instructions: Vec<Vec<&str>> = vec![];
        for line in input.lines() {
            if read_stacks {
                let row: Vec<char> = line.chars().collect();
                if row.contains(&'1') {
                    read_stacks = false;
                    continue;
                }
                let mut chars: Vec<char> = vec![];
                let mut index = 1;
                while index < row.len() {
                    chars.push(row[index]);
                    index += 4;
                }
                crate_rows.push(chars);
            } else {
                instructions.push(line.split(" ").collect())
            }
        }

        let mut stacks = build_stacks(crate_rows);

        for row in instructions.iter() {
            if row.len() <= 1 {
                continue;
            }
            let mut count = row[1].parse::<u8>().unwrap();
            let source = row[3].parse::<usize>().unwrap() - 1;
            let dest = row[5].parse::<usize>().unwrap() - 1;

            while count > 0 {
                match stacks[source].pop() {
                    Some(crate_to_move) => stacks[dest].push(crate_to_move),
                    None => panic!("no crates in stack"),
                }
                count -= 1;
            }
        }

        let mut result = String::from("");

        for (_, stack) in stacks.iter().enumerate() {
            if let Some(top) = stack.last() {
                result.push(*top);
            }
        }

        result
    }

    fn part_two(&self, input: &str) -> String {
        let mut read_stacks = true;
        let mut crate_rows: Vec<Vec<char>> = vec![];
        let mut instructions: Vec<Vec<&str>> = vec![];
        for line in input.lines() {
            if read_stacks {
                let row: Vec<char> = line.chars().collect();
                if row.contains(&'1') {
                    read_stacks = false;
                    continue;
                }
                let mut chars: Vec<char> = vec![];
                let mut index = 1;
                while index < row.len() {
                    chars.push(row[index]);
                    index += 4;
                }
                crate_rows.push(chars);
            } else {
                instructions.push(line.split(" ").collect())
            }
        }

        let mut stacks = build_stacks(crate_rows);

        for row in instructions.iter() {
            if row.len() <= 1 {
                continue;
            }
            let count = row[1].parse::<u8>().unwrap();
            let source = row[3].parse::<usize>().unwrap() - 1;
            let dest = row[5].parse::<usize>().unwrap() - 1;

            let mut crane = vec![];
            let mut crane_count = count;
            while crane_count > 0 {
                match stacks[source].pop() {
                    Some(top) => crane.push(top),
                    None => panic!("no crates in stack"),
                }
                crane_count -= 1;
            }

            while crane_count < count {
                match crane.pop() {
                    Some(top) => stacks[dest].push(top),
                    None => panic!("no crates in crane"),
                }
                crane_count += 1;
            }
        }

        let mut result = String::from("");

        for (_, stack) in stacks.iter().enumerate() {
            if let Some(top) = stack.last() {
                result.push(*top);
            }
        }

        result
    }
}

fn build_stacks(crate_rows: Vec<Vec<char>>) -> Vec<Vec<char>> {
    let mut stacks: Vec<Vec<char>> = vec![Vec::new(); crate_rows[0].len()];
    for row in crate_rows.iter().rev() {
        for (i, c) in row.iter().enumerate() {
            if *c != ' ' {
                stacks[i].push(*c);
            }
        }
    }

    stacks
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::util;

    #[test]
    fn test_part_one() {
        assert_eq!(
            "CMZ",
            Day05 {}.part_one(&util::input(5, util::InputType::Example))
        )
    }

    #[test]
    fn test_part_two() {
        assert_eq!(
            "MCD",
            Day05 {}.part_two(&util::input(5, util::InputType::Example))
        )
    }
}
