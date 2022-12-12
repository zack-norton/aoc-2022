use crate::days::Problem;

pub struct Day10 {}

impl Problem for Day10 {
    fn part_one(&self, input: &str) -> String {
        let mut nooper = Nooper::new();
        for line in input.lines() {
            let instruction = line.split(" ").collect::<Vec<&str>>();
            match instruction[0] {
                "noop" => {
                    nooper.noop();
                }
                "addx" => {
                    nooper.addx(instruction[1].parse::<i32>().unwrap());
                }
                _ => panic!("unknown instruction"),
            }
        }

        nooper.signal_strength.iter().sum::<i32>().to_string()
    }

    fn part_two(&self, input: &str) -> String {
        let mut nooper = Nooper2::new();
        for line in input.lines() {
            let instruction = line.split(" ").collect::<Vec<&str>>();
            match instruction[0] {
                "noop" => {
                    nooper.noop();
                }
                "addx" => {
                    nooper.addx(instruction[1].parse::<i32>().unwrap());
                }
                _ => panic!("unknown instruction"),
            }
        }

        nooper.print()
    }
}

struct Nooper {
    cycle: i32,
    signal_strength: Vec<i32>,
    register_x: i32,
}

impl Nooper {
    fn new() -> Self {
        Self {
            cycle: 0,
            signal_strength: vec![],
            register_x: 1,
        }
    }
    fn noop(&mut self) {
        self.cycle += 1;
        self.check_cycle_count();
    }

    fn check_cycle_count(&mut self) {
        if self.cycle == 20 || (self.cycle - 20) % 40 == 0 {
            self.signal_strength.push(self.cycle * self.register_x);
        }
    }

    fn addx(&mut self, x_val: i32) {
        self.cycle += 1;
        self.check_cycle_count();
        self.cycle += 1;
        self.check_cycle_count();
        self.register_x += x_val;
    }
}

struct Nooper2 {
    cycle: i32,
    register_x: i32,
    signal: Vec<String>,
}

impl Nooper2 {
    fn new() -> Self {
        Self {
            cycle: 0,
            register_x: 1,
            signal: vec![],
        }
    }

    fn noop(&mut self) {
        self.next_cycle();
    }

    fn addx(&mut self, x_val: i32) {
        self.next_cycle();
        self.next_cycle();
        self.register_x += x_val;
    }

    fn next_cycle(&mut self) {
        let position = self.cycle % 40;
        let row = self.cycle / 40;
        println!("position {}", position);
        println!("row {}", row);
        if position == 0 {
            self.signal.push(String::new());
        }
        if self.register_x == position
            || self.register_x - 1 == position
            || self.register_x + 1 == position
        {
            self.signal[row as usize].push('#');
        } else {
            self.signal[row as usize].push('.');
        }
        self.cycle += 1;
    }

    fn print(&self) -> String {
        let mut screen = String::new();
        for row in self.signal.iter() {
            screen = format!("{}\n{}", screen, row);
        }
        screen
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::util;

    #[test]
    fn test_part_one() {
        assert_eq!(
            "13140",
            Day10 {}.part_one(&util::input(10, util::InputType::Example))
        )
    }

    #[test]
    fn test_part_two() {
        assert_eq!(
            "\n##..##..##..##..##..##..##..##..##..##..\n###...###...###...###...###...###...###.\n####....####....####....####....####....\n#####.....#####.....#####.....#####.....\n######......######......######......####\n#######.......#######.......#######.....",
            Day10 {}.part_two(&util::input(10, util::InputType::Example))
        )
    }
}
