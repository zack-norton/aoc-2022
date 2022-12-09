use crate::days::Problem;

use std::collections::HashSet;
use std::num;

pub struct Day09 {}

impl Problem for Day09 {
    fn part_one(&self, input: &str) -> String {
        let instructions_list = input.lines();

        let mut head = Node::new();
        let mut tail = Node::new();

        for row in instructions_list {
            let instruct: Vec<&str> = row.split(" ").collect();

            match instruct[0] {
                "U" => move_head(
                    Direction::Up,
                    instruct[1].parse::<i32>().unwrap(),
                    &mut head,
                    &mut tail,
                ),
                "R" => move_head(
                    Direction::Right,
                    instruct[1].parse::<i32>().unwrap(),
                    &mut head,
                    &mut tail,
                ),
                "D" => move_head(
                    Direction::Down,
                    instruct[1].parse::<i32>().unwrap(),
                    &mut head,
                    &mut tail,
                ),
                "L" => move_head(
                    Direction::Left,
                    instruct[1].parse::<i32>().unwrap(),
                    &mut head,
                    &mut tail,
                ),
                _ => panic!("Unknown instruction {}", instruct[0]),
            };
        }

        tail.visited.len().to_string()
    }

    fn part_two(&self, input: &str) -> String {
        let instructions_list: Vec<&str> = input.lines().collect();

        let mut node_list = vec![];

        for _ in 0..10 {
            node_list.push(Node::new());
        }

        for row in instructions_list.iter() {
            let direction = match row.split(" ").collect::<Vec<&str>>()[0] {
                "U" => Direction::Up,
                "R" => Direction::Right,
                "D" => Direction::Down,
                "L" => Direction::Left,
                _ => panic!("Unknown direction"),
            };
            let steps = row.split(" ").collect::<Vec<&str>>()[1]
                .parse::<i32>()
                .unwrap();

            for _ in 0..steps {
                match direction {
                    Direction::Up => {
                        node_list[0].move_up();
                    }
                    Direction::Right => {
                        node_list[0].move_right();
                    }
                    Direction::Left => {
                        node_list[0].move_left();
                    }
                    Direction::Down => {
                        node_list[0].move_down();
                    }
                };

                for i in 1..node_list.len() {
                    let h = node_list[i - 1].clone();
                    node_list[i].check_move_needed(&h);
                }
            }
        }

        node_list.last().unwrap().visited.len().to_string()
    }
}

fn move_head(direction: Direction, steps: i32, head: &mut Node, tail: &mut Node) {
    for _ in 0..steps {
        match direction {
            Direction::Up => head.move_up(),
            Direction::Right => head.move_right(),
            Direction::Down => head.move_down(),
            Direction::Left => head.move_left(),
        };
        tail.check_move_needed(head);
    }
}

enum Direction {
    Up,
    Right,
    Down,
    Left,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
struct Position {
    x: i32,
    y: i32,
}

impl Position {
    fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    fn distance(&self, other: &Position) -> f64 {
        let x_squared = (other.x - self.x).pow(2);
        let y_squared = (other.y - self.y).pow(2);
        ((x_squared + y_squared) as f64).sqrt()
    }
}

#[derive(Clone)]
struct Node {
    position: Position,
    visited: HashSet<Position>,
}

impl Node {
    fn new() -> Self {
        let position = Position::new(0, 0);
        let mut visited = HashSet::new();
        visited.insert(Position::new(0, 0));
        Self { position, visited }
    }

    fn move_position(&mut self, x: i32, y: i32) {
        self.position.x = x;
        self.position.y = y;
        self.visited.insert(Position::new(x, y));
    }

    fn move_up(&mut self) {
        self.move_position(self.position.x, self.position.y + 1);
    }

    fn move_right(&mut self) {
        self.move_position(self.position.x + 1, self.position.y);
    }

    fn move_down(&mut self) {
        self.move_position(self.position.x, self.position.y - 1);
    }

    fn move_left(&mut self) {
        self.move_position(self.position.x - 1, self.position.y);
    }

    fn move_diagonal_right_up(&mut self) {
        self.move_position(self.position.x + 1, self.position.y + 1);
    }

    fn move_diagonal_right_down(&mut self) {
        self.move_position(self.position.x + 1, self.position.y - 1);
    }

    fn move_diagonal_left_up(&mut self) {
        self.move_position(self.position.x - 1, self.position.y + 1);
    }

    fn move_diagonal_left_down(&mut self) {
        self.move_position(self.position.x - 1, self.position.y - 1);
    }

    fn check_move_needed(&mut self, head: &Node) {
        if self.position.distance(&head.position) >= 2.0 {
            //move needed
            //check same row
            if self.position.y == head.position.y {
                if self.position.x < head.position.x {
                    self.move_right();
                } else {
                    self.move_left();
                }
            } else if self.position.x == head.position.x {
                //same col
                if self.position.y < head.position.y {
                    self.move_up();
                } else {
                    self.move_down();
                }
            } else {
                //diagonal move
                if self.position.x < head.position.x {
                    //need right
                    //up or down
                    if self.position.y < head.position.y {
                        //right up
                        self.move_diagonal_right_up();
                    } else {
                        //right down
                        self.move_diagonal_right_down();
                    }
                } else {
                    //need left
                    //up or down
                    if self.position.y < head.position.y {
                        //left up
                        self.move_diagonal_left_up();
                    } else {
                        //left down
                        self.move_diagonal_left_down();
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::util;

    #[test]
    fn test_part_one() {
        assert_eq!(
            "13",
            Day09 {}.part_one(&util::input(9, util::InputType::Example))
        )
    }

    #[test]
    fn test_part_two() {
        assert_eq!(
            "1",
            Day09 {}.part_two(&util::input(9, util::InputType::Example))
        )
    }
}
