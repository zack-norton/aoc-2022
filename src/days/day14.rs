use crate::days::Problem;

pub struct Day14 {}

impl Problem for Day14 {
    fn part_one(&self, input: &str) -> String {
        let mut cave = vec![vec!['.'; 1000]; 1000];

        let rocks: Vec<Vec<Point>> = input
            .lines()
            .map(|line| {
                line.split(" -> ")
                    .collect::<Vec<&str>>()
                    .iter()
                    .map(|x| Point::from_str(x))
                    .collect::<Vec<Point>>()
            })
            .collect();

        for wall in rocks.iter() {
            for i in 0..wall.len() - 1 {
                let mut start = wall[i];
                let finish = wall[i + 1];
                while start.row != finish.row || start.col != finish.col {
                    cave[start.row as usize][start.col as usize] = '#';
                    if start.row == finish.row {
                        //left or right
                        if start.col < finish.col {
                            //go right
                            start.col += 1;
                        } else {
                            //go left
                            start.col -= 1;
                        }
                    } else {
                        //up or down
                        if start.row < finish.row {
                            //go down
                            start.row += 1;
                        } else {
                            //go up
                            start.row -= 1;
                        }
                    }
                }
                cave[start.row as usize][start.col as usize] = '#';
            }
        }

        let mut done = false;
        let mut sand_count = 0;
        while !done {
            let mut sand = Point { row: 0, col: 500 };
            let mut falling = true;
            while falling {
                if ((sand.row + 1) as usize) >= cave.len() {
                    //at bottom
                    done = true;
                    break;
                }
                if cave[(sand.row + 1) as usize][sand.col as usize] == '.' {
                    cave[sand.row as usize][sand.col as usize] = '.';
                    sand.row += 1;
                    cave[sand.row as usize][sand.col as usize] = 'o';
                    continue;
                }
                if cave[(sand.row + 1) as usize][sand.col as usize] == 'o'
                    || cave[(sand.row + 1) as usize][sand.col as usize] == '#'
                {
                    //try left
                    if sand.col > 0 {
                        if cave[(sand.row + 1) as usize][(sand.col - 1) as usize] == '.' {
                            cave[sand.row as usize][sand.col as usize] = '.';
                            sand.row += 1;
                            sand.col -= 1;
                            cave[sand.row as usize][sand.col as usize] = 'o';
                            continue;
                        }
                    }
                    //try right
                    if (sand.col as usize) < cave[sand.row as usize].len() - 1 {
                        if cave[(sand.row as usize) + 1][(sand.col as usize) + 1] == '.' {
                            cave[sand.row as usize][sand.col as usize] = '.';
                            sand.row += 1;
                            sand.col += 1;
                            cave[sand.row as usize][sand.col as usize] = 'o';
                            continue;
                        }
                    }
                }
                //if here, no longer falling
                falling = false;
                sand_count += 1;
            }
        }

        sand_count.to_string()
    }

    fn part_two(&self, input: &str) -> String {
        let mut cave = vec![vec!['.'; 1000]; 1000];

        let rocks: Vec<Vec<Point>> = input
            .lines()
            .map(|line| {
                line.split(" -> ")
                    .collect::<Vec<&str>>()
                    .iter()
                    .map(|x| Point::from_str(x))
                    .collect::<Vec<Point>>()
            })
            .collect();

        let mut max_row = 0;
        for wall in rocks.iter() {
            for i in 0..wall.len() - 1 {
                let mut start = wall[i];
                let finish = wall[i + 1];
                if start.row > max_row {
                    max_row = start.row;
                }
                if finish.row > max_row {
                    max_row = finish.row;
                }
                while start.row != finish.row || start.col != finish.col {
                    cave[start.row as usize][start.col as usize] = '#';
                    if start.row == finish.row {
                        //left or right
                        if start.col < finish.col {
                            //go right
                            start.col += 1;
                        } else {
                            //go left
                            start.col -= 1;
                        }
                    } else {
                        //up or down
                        if start.row < finish.row {
                            //go down
                            start.row += 1;
                        } else {
                            //go up
                            start.row -= 1;
                        }
                    }
                }
                cave[start.row as usize][start.col as usize] = '#';
            }
        }

        for i in 0..1000 {
            cave[(max_row + 2) as usize][i] = '#';
        }

        let mut done = false;
        let mut sand_count = 0;
        while !done {
            let mut sand = Point { row: 0, col: 500 };
            let mut falling = true;
            if cave[sand.row as usize][sand.col as usize] == 'o' {
                done = true;
                break;
            }
            cave[sand.row as usize][sand.col as usize] = 'o';
            while falling {
                if ((sand.row + 1) as usize) >= cave.len() {
                    //at bottom
                    done = true;
                    break;
                }
                if cave[(sand.row + 1) as usize][sand.col as usize] == '.' {
                    cave[sand.row as usize][sand.col as usize] = '.';
                    sand.row += 1;
                    cave[sand.row as usize][sand.col as usize] = 'o';
                    continue;
                }
                if cave[(sand.row + 1) as usize][sand.col as usize] == 'o'
                    || cave[(sand.row + 1) as usize][sand.col as usize] == '#'
                {
                    //try left
                    if sand.col > 0 {
                        if cave[(sand.row + 1) as usize][(sand.col - 1) as usize] == '.' {
                            cave[sand.row as usize][sand.col as usize] = '.';
                            sand.row += 1;
                            sand.col -= 1;
                            cave[sand.row as usize][sand.col as usize] = 'o';
                            continue;
                        }
                    }
                    //try right
                    if (sand.col as usize) < cave[sand.row as usize].len() - 1 {
                        if cave[(sand.row as usize) + 1][(sand.col as usize) + 1] == '.' {
                            cave[sand.row as usize][sand.col as usize] = '.';
                            sand.row += 1;
                            sand.col += 1;
                            cave[sand.row as usize][sand.col as usize] = 'o';
                            continue;
                        }
                    }
                }
                //if here, no longer falling
                falling = false;
                sand_count += 1;
            }
        }

        sand_count.to_string()
    }
}

#[derive(Debug, Copy, Clone)]
struct Point {
    row: u32,
    col: u32,
}

impl Point {
    fn from_str(input: &str) -> Self {
        let pair = input.split(",").collect::<Vec<&str>>();
        Self {
            row: pair[1].parse::<u32>().unwrap(),
            col: pair[0].parse::<u32>().unwrap(),
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
            "24",
            Day14 {}.part_one(&util::input(14, util::InputType::Example))
        )
    }

    #[test]
    fn test_part_two() {
        assert_eq!(
            "93",
            Day14 {}.part_two(&util::input(14, util::InputType::Example))
        )
    }
}
