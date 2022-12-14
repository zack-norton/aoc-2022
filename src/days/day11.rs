use crate::days::Problem;

use std::collections::VecDeque;

pub struct Day11 {}

impl Problem for Day11 {
    fn part_one(&self, input: &str) -> String {
        let mut lines = input.lines();

        let mut monkeys = vec![];

        loop {
            let line = lines.next().expect("unexpected eof");
            if !line.starts_with("Monkey") {
                panic!("expected monkey");
            }

            let mut line = lines.next().expect("unexpected eof");
            let items = line.split(": ").collect::<Vec<&str>>()[1]
                .split(", ")
                .map(|x| x.parse::<u128>().unwrap())
                .collect::<VecDeque<u128>>();

            line = lines.next().expect("unexpected eof");
            let op_str = line.split("= ").collect::<Vec<&str>>()[1];
            let op = match op_str {
                "old * old" => OperationAction::Square,
                _ => {
                    if op_str.starts_with("old +") {
                        OperationAction::Add(
                            op_str.split(" + ").collect::<Vec<&str>>()[1]
                                .parse::<u128>()
                                .unwrap(),
                        )
                    } else {
                        OperationAction::Multiply(
                            op_str.split(" * ").collect::<Vec<&str>>()[1]
                                .parse::<u128>()
                                .unwrap(),
                        )
                    }
                }
            };

            line = lines.next().expect("unexpected eof");
            let divisor = line.split("by ").collect::<Vec<&str>>()[1]
                .parse::<u128>()
                .unwrap();
            line = lines.next().expect("unexpected eof");
            let test_true = line.split("monkey ").collect::<Vec<&str>>()[1]
                .parse::<u128>()
                .unwrap();
            line = lines.next().expect("unexpected eof");
            let test_false = line.split("monkey ").collect::<Vec<&str>>()[1]
                .parse::<u128>()
                .unwrap();

            monkeys.push(Monkey {
                items,
                operation: Operation { action: op },
                test: divisor,
                test_true,
                test_false,
                inspection_count: 0,
            });

            let line = lines.next();

            match line {
                None => break,
                Some(l) => {
                    if l != "" {
                        panic!("expected blank line")
                    }
                }
            };
        }

        for _ in 0..20 {
            play_round(&mut monkeys.as_mut_slice());
        }

        let mut inspections: Vec<u128> = monkeys.iter().map(|x| x.inspection_count).collect();
        inspections.sort_by(|a, b| b.cmp(a));

        (inspections[0] * inspections[1]).to_string()
    }

    fn part_two(&self, input: &str) -> String {
        let mut lines = input.lines();

        let mut monkeys = vec![];

        loop {
            let line = lines.next().expect("unexpected eof");
            if !line.starts_with("Monkey") {
                panic!("expected monkey");
            }

            let mut line = lines.next().expect("unexpected eof");
            let items = line.split(": ").collect::<Vec<&str>>()[1]
                .split(", ")
                .map(|x| x.parse::<u128>().unwrap())
                .collect::<VecDeque<u128>>();

            line = lines.next().expect("unexpected eof");
            let op_str = line.split("= ").collect::<Vec<&str>>()[1];
            let op = match op_str {
                "old * old" => OperationAction::Square,
                _ => {
                    if op_str.starts_with("old +") {
                        OperationAction::Add(
                            op_str.split(" + ").collect::<Vec<&str>>()[1]
                                .parse::<u128>()
                                .unwrap(),
                        )
                    } else {
                        OperationAction::Multiply(
                            op_str.split(" * ").collect::<Vec<&str>>()[1]
                                .parse::<u128>()
                                .unwrap(),
                        )
                    }
                }
            };

            line = lines.next().expect("unexpected eof");
            let divisor = line.split("by ").collect::<Vec<&str>>()[1]
                .parse::<u128>()
                .unwrap();
            line = lines.next().expect("unexpected eof");
            let test_true = line.split("monkey ").collect::<Vec<&str>>()[1]
                .parse::<u128>()
                .unwrap();
            line = lines.next().expect("unexpected eof");
            let test_false = line.split("monkey ").collect::<Vec<&str>>()[1]
                .parse::<u128>()
                .unwrap();

            monkeys.push(Monkey {
                items,
                operation: Operation { action: op },
                test: divisor,
                test_true,
                test_false,
                inspection_count: 0,
            });

            let line = lines.next();

            match line {
                None => break,
                Some(l) => {
                    if l != "" {
                        panic!("expected blank line")
                    }
                }
            };
        }

        for _ in 0..10000 {
            play_alt_round(&mut monkeys.as_mut_slice());
        }

        let mut inspections: Vec<u128> = monkeys.iter().map(|x| x.inspection_count).collect();
        inspections.sort_by(|a, b| b.cmp(a));

        (inspections[0] * inspections[1]).to_string()
    }
}

fn play_round(monkeys: &mut [Monkey]) {
    for i in 0..monkeys.len() {
        unsafe {
            let monkey: *mut Monkey = &mut monkeys[i];

            let item_count = (*monkey).items.len();
            for _ in 0..item_count {
                (*monkey).inspection_count += 1;
                let mut item = (*monkey).items.pop_front().unwrap();
                item = (*monkey).operation.operate(item);

                item = item / 3;

                if item % ((*monkey).test) == 0 {
                    let other_monkey: *mut Monkey = &mut monkeys[(*monkey).test_true as usize];
                    (*other_monkey).items.push_back(item.clone());
                } else {
                    let other_monkey: *mut Monkey = &mut monkeys[(*monkey).test_false as usize];
                    (*other_monkey).items.push_back(item.clone());
                }
            }
        }
    }
}

fn play_alt_round(monkeys: &mut [Monkey]) {
    let magic = monkeys.iter().map(|m| m.test).product::<u128>();
    for i in 0..monkeys.len() {
        unsafe {
            let monkey: *mut Monkey = &mut monkeys[i];

            let item_count = (*monkey).items.len();
            for _ in 0..item_count {
                (*monkey).inspection_count += 1;
                let mut item = (*monkey).items.pop_front().unwrap();
                item = (*monkey).operation.operate(item);

                item %= magic;

                //item = item / 3;

                if item % ((*monkey).test) == 0 {
                    let other_monkey: *mut Monkey = &mut monkeys[(*monkey).test_true as usize];
                    (*other_monkey).items.push_back(item.clone());
                } else {
                    let other_monkey: *mut Monkey = &mut monkeys[(*monkey).test_false as usize];
                    (*other_monkey).items.push_back(item.clone());
                }
            }
        }
    }
}

#[derive(Debug)]
enum OperationAction {
    Multiply(u128),
    Add(u128),
    Square,
}

#[derive(Debug)]
struct Operation {
    action: OperationAction,
}

impl Operation {
    fn operate(&self, old: u128) -> u128 {
        match self.action {
            OperationAction::Multiply(val) => old * val,
            OperationAction::Add(val) => old + val,
            OperationAction::Square => old * old,
        }
    }
}

#[derive(Debug)]
struct Monkey {
    items: VecDeque<u128>,
    operation: Operation,
    test: u128,
    test_true: u128,
    test_false: u128,
    inspection_count: u128,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::util;

    #[test]
    fn part_one() {
        assert_eq!(
            "10605",
            Day11 {}.part_one(&util::input(11, util::InputType::Example))
        )
    }

    #[test]
    fn part_two() {
        assert_eq!(
            "2713310158",
            Day11 {}.part_two(&util::input(11, util::InputType::Example))
        )
    }
}
