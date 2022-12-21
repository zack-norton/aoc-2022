use crate::days::Problem;

use std::collections::VecDeque;

pub struct Day12 {}

impl Problem for Day12 {
    fn part_one(&self, input: &str) -> String {
        let mut map: Vec<Vec<char>> = vec![];

        for line in input.lines() {
            map.push(line.chars().collect::<Vec<char>>());
        }

        let mut start = Position { row: 0, col: 0 };
        let mut end = Position { row: 0, col: 0 };

        let mut height_map: Vec<Vec<Node>> = vec![];

        for i in 0..map.len() {
            let mut row: Vec<Node> = vec![];
            for j in 0..map[i].len() {
                if map[i][j] == 'S' {
                    start.row = i as u32;
                    start.col = j as u32;
                    row.push(Node::new(
                        Position {
                            row: i as u32,
                            col: j as u32,
                        },
                        'a',
                        true,
                        false,
                    ));
                } else if map[i][j] == 'E' {
                    end.row = i as u32;
                    end.col = j as u32;
                    row.push(Node::new(
                        Position {
                            row: i as u32,
                            col: j as u32,
                        },
                        'z',
                        false,
                        true,
                    ));
                } else {
                    row.push(Node::new(
                        Position {
                            row: i as u32,
                            col: j as u32,
                        },
                        map[i][j],
                        false,
                        false,
                    ));
                }
            }
            height_map.push(row);
        }

        let mut queue: VecDeque<Position> = VecDeque::new();
        queue.push_back(height_map[start.row as usize][start.col as usize].position);
        let mut min_distance = 0;

        while !queue.is_empty() {
            let current_position = queue.pop_front().unwrap();
            if height_map[current_position.row as usize][current_position.col as usize].is_end {
                min_distance = height_map[current_position.row as usize]
                    [current_position.col as usize]
                    .distance;
                break;
            }
            if height_map[current_position.row as usize][current_position.col as usize].visited {
                continue;
            }

            //up
            if current_position.row > 0
                && height_map[(current_position.row - 1) as usize][current_position.col as usize]
                    .height as u8
                    <= height_map[current_position.row as usize][current_position.col as usize]
                        .height as u8
                        + 1
                && !height_map[(current_position.row - 1) as usize][current_position.col as usize]
                    .visited
            {
                let next = Position {
                    row: current_position.row - 1,
                    col: current_position.col,
                };
                height_map[next.row as usize][next.col as usize].distance = height_map
                    [current_position.row as usize][current_position.col as usize]
                    .distance
                    + 1;
                queue.push_back(next);
            }
            //right
            if (current_position.col as usize) < height_map[0].len() - 1
                && height_map[current_position.row as usize][(current_position.col + 1) as usize]
                    .height as u8
                    <= height_map[current_position.row as usize][current_position.col as usize]
                        .height as u8
                        + 1
                && !height_map[current_position.row as usize][(current_position.col + 1) as usize]
                    .visited
            {
                let next = Position {
                    row: current_position.row,
                    col: current_position.col + 1,
                };
                height_map[next.row as usize][next.col as usize].distance = height_map
                    [current_position.row as usize][current_position.col as usize]
                    .distance
                    + 1;
                queue.push_back(next);
            }

            //down
            if (current_position.row as usize) < height_map.len() - 1
                && height_map[(current_position.row + 1) as usize][current_position.col as usize]
                    .height as u8
                    <= height_map[current_position.row as usize][current_position.col as usize]
                        .height as u8
                        + 1
                && !height_map[(current_position.row + 1) as usize][current_position.col as usize]
                    .visited
            {
                let next = Position {
                    row: current_position.row + 1,
                    col: current_position.col,
                };
                height_map[next.row as usize][next.col as usize].distance = height_map
                    [current_position.row as usize][current_position.col as usize]
                    .distance
                    + 1;
                queue.push_back(next);
            }

            //left
            if current_position.col as usize > 0
                && height_map[current_position.row as usize][(current_position.col - 1) as usize]
                    .height as u8
                    <= height_map[current_position.row as usize][current_position.col as usize]
                        .height as u8
                        + 1
                && !height_map[current_position.row as usize][(current_position.col - 1) as usize]
                    .visited
            {
                let next = Position {
                    row: current_position.row,
                    col: current_position.col - 1,
                };
                height_map[next.row as usize][next.col as usize].distance = height_map
                    [current_position.row as usize][current_position.col as usize]
                    .distance
                    + 1;
                queue.push_back(next);
            }

            height_map[current_position.row as usize][current_position.col as usize].visited = true;
            let visited_count = height_map
                .iter()
                .flatten()
                .map(|x| if x.visited { 1 } else { 0 })
                .sum::<u32>();
        }

        min_distance.to_string()
    }

    fn part_two(&self, input: &str) -> String {
        let mut map: Vec<Vec<char>> = vec![];

        for line in input.lines() {
            map.push(line.chars().collect::<Vec<char>>());
        }

        let mut start = vec![];
        let mut end = Position { row: 0, col: 0 };

        let mut height_map: Vec<Vec<Node>> = vec![];

        for i in 0..map.len() {
            let mut row: Vec<Node> = vec![];
            for j in 0..map[i].len() {
                if map[i][j] == 'S' || map[i][j] == 'a' {
                    let rowi = i as u32;
                    let colj = j as u32;
                    row.push(Node::new(
                        Position {
                            row: i as u32,
                            col: j as u32,
                        },
                        'a',
                        true,
                        false,
                    ));
                    start.push(Position {
                        row: rowi,
                        col: colj,
                    });
                } else if map[i][j] == 'E' {
                    end.row = i as u32;
                    end.col = j as u32;
                    row.push(Node::new(
                        Position {
                            row: i as u32,
                            col: j as u32,
                        },
                        'z',
                        false,
                        true,
                    ));
                } else {
                    row.push(Node::new(
                        Position {
                            row: i as u32,
                            col: j as u32,
                        },
                        map[i][j],
                        false,
                        false,
                    ));
                }
            }
            height_map.push(row);
        }

        let mut mins = vec![];

        for i in 0..start.len() {
            let start_pos = start[i];
            let mut queue: VecDeque<Position> = VecDeque::new();
            queue.push_back(height_map[start_pos.row as usize][start_pos.col as usize].position);
            let mut min_distance = 0;
            let mut finished = false;

            while !queue.is_empty() {
                let current_position = queue.pop_front().unwrap();
                if height_map[current_position.row as usize][current_position.col as usize].is_end {
                    min_distance = height_map[current_position.row as usize]
                        [current_position.col as usize]
                        .distance;
                    finished = true;
                    break;
                }
                if height_map[current_position.row as usize][current_position.col as usize].visited
                {
                    continue;
                }

                //up
                if current_position.row > 0
                    && height_map[(current_position.row - 1) as usize]
                        [current_position.col as usize]
                        .height as u8
                        <= height_map[current_position.row as usize][current_position.col as usize]
                            .height as u8
                            + 1
                    && !height_map[(current_position.row - 1) as usize]
                        [current_position.col as usize]
                        .visited
                {
                    let next = Position {
                        row: current_position.row - 1,
                        col: current_position.col,
                    };
                    height_map[next.row as usize][next.col as usize].distance = height_map
                        [current_position.row as usize][current_position.col as usize]
                        .distance
                        + 1;
                    queue.push_back(next);
                }
                //right
                if (current_position.col as usize) < height_map[0].len() - 1
                    && height_map[current_position.row as usize]
                        [(current_position.col + 1) as usize]
                        .height as u8
                        <= height_map[current_position.row as usize][current_position.col as usize]
                            .height as u8
                            + 1
                    && !height_map[current_position.row as usize]
                        [(current_position.col + 1) as usize]
                        .visited
                {
                    let next = Position {
                        row: current_position.row,
                        col: current_position.col + 1,
                    };
                    height_map[next.row as usize][next.col as usize].distance = height_map
                        [current_position.row as usize][current_position.col as usize]
                        .distance
                        + 1;
                    queue.push_back(next);
                }

                //down
                if (current_position.row as usize) < height_map.len() - 1
                    && height_map[(current_position.row + 1) as usize]
                        [current_position.col as usize]
                        .height as u8
                        <= height_map[current_position.row as usize][current_position.col as usize]
                            .height as u8
                            + 1
                    && !height_map[(current_position.row + 1) as usize]
                        [current_position.col as usize]
                        .visited
                {
                    let next = Position {
                        row: current_position.row + 1,
                        col: current_position.col,
                    };
                    height_map[next.row as usize][next.col as usize].distance = height_map
                        [current_position.row as usize][current_position.col as usize]
                        .distance
                        + 1;
                    queue.push_back(next);
                }

                //left
                if current_position.col as usize > 0
                    && height_map[current_position.row as usize]
                        [(current_position.col - 1) as usize]
                        .height as u8
                        <= height_map[current_position.row as usize][current_position.col as usize]
                            .height as u8
                            + 1
                    && !height_map[current_position.row as usize]
                        [(current_position.col - 1) as usize]
                        .visited
                {
                    let next = Position {
                        row: current_position.row,
                        col: current_position.col - 1,
                    };
                    height_map[next.row as usize][next.col as usize].distance = height_map
                        [current_position.row as usize][current_position.col as usize]
                        .distance
                        + 1;
                    queue.push_back(next);
                }

                height_map[current_position.row as usize][current_position.col as usize].visited =
                    true;
                let visited_count = height_map
                    .iter()
                    .flatten()
                    .map(|x| if x.visited { 1 } else { 0 })
                    .sum::<u32>();
            }

            if finished {
                mins.push(min_distance);
            }

            for i in height_map.iter_mut().flatten() {
                i.visited = false;
                i.distance = 0;
            }
        }

        mins.sort();
        mins[0].to_string()
    }
}

#[derive(Debug, Copy, Clone)]
struct Position {
    row: u32,
    col: u32,
}

struct Node {
    position: Position,
    visited: bool,
    height: char,
    is_start: bool,
    is_end: bool,
    distance: u32,
}

impl Node {
    fn new(position: Position, height: char, is_start: bool, is_end: bool) -> Self {
        Self {
            position,
            visited: false,
            height,
            is_start,
            is_end,
            distance: 0,
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
            "31",
            Day12 {}.part_one(&util::input(12, util::InputType::Example))
        )
    }

    #[test]
    fn test_part_two() {
        assert_eq!(
            "29",
            Day12 {}.part_two(&util::input(12, util::InputType::Example))
        )
    }
}
