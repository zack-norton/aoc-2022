use crate::days::Problem;

pub struct Day08 {}

impl Problem for Day08 {
    fn part_one(&self, input: &str) -> String {
        let mut grid: Vec<Vec<u32>> = vec![];
        for line in input.lines() {
            grid.push(line.chars().map(|x| x.to_digit(10).unwrap()).collect())
        }

        let length = grid.len();
        let width = grid[0].len();
        let mut visible = 0;

        for i in 1..length - 1 {
            for j in 1..width - 1 {
                let mut isVisible = false;
                //check left
                let mut col = j;
                let mut checking = true;
                while !isVisible && col > 0 && checking {
                    if grid[i][col - 1] >= grid[i][j] {
                        checking = false;
                    } else {
                        col = col - 1;
                    }
                }
                if checking {
                    isVisible = true;
                }

                //check up
                let mut row = i;
                let mut checking = true;
                while !isVisible && row > 0 && checking {
                    if grid[row - 1][j] >= grid[i][j] {
                        checking = false;
                    } else {
                        row = row - 1;
                    }
                }
                if checking {
                    isVisible = true;
                }

                //check right
                let mut col = j;
                let mut checking = true;
                while !isVisible && col < width - 1 && checking {
                    if grid[i][col + 1] >= grid[i][j] {
                        checking = false;
                    } else {
                        col = col + 1;
                    }
                }
                if checking {
                    isVisible = true;
                }

                //check down
                let mut row = i;
                let mut checking = true;
                while !isVisible && row < length - 1 && checking {
                    if grid[row + 1][j] >= grid[i][j] {
                        checking = false;
                    } else {
                        row = row + 1;
                    }
                }
                if checking {
                    isVisible = true;
                }

                if isVisible {
                    visible += 1;
                }
            }
        }

        visible += 2 * length;
        visible += 2 * (width - 2);

        visible.to_string()
    }

    fn part_two(&self, input: &str) -> String {
        let mut grid: Vec<Vec<u32>> = vec![];
        for line in input.lines() {
            grid.push(line.chars().map(|x| x.to_digit(10).unwrap()).collect())
        }

        let length = grid.len();
        let width = grid[0].len();
        let mut max_scenic = 0;

        for i in 0..length {
            for j in 0..width {
                //left score
                let mut left_score = 0;
                let mut blocked = false;
                if j > 0 {
                    let mut col = j as i32 - 1;
                    while col >= 0 && !blocked {
                        left_score += 1;
                        if grid[i][col as usize] >= grid[i][j] {
                            blocked = true;
                        }

                        col -= 1;
                    }
                }

                //up score
                let mut up_score = 0;
                let mut blocked = false;
                if i > 0 {
                    let mut row = i as i32 - 1;
                    while row >= 0 && !blocked {
                        if grid[row as usize][j] >= grid[i][j] {
                            blocked = true;
                        }
                        up_score += 1;
                        row -= 1;
                    }
                }

                //right score
                let mut right_score = 0;
                let mut blocked = false;
                if j < width - 1 {
                    let mut col = j + 1;
                    while col <= length - 1 && !blocked {
                        if grid[i][col] >= grid[i][j] {
                            blocked = true;
                        }
                        right_score += 1;
                        col += 1;
                    }
                }

                //down score
                let mut down_score = 0;
                let mut blocked = false;
                if i < length - 1 {
                    let mut row = i + 1;
                    while row <= width - 1 && !blocked {
                        if grid[row][j] >= grid[i][j] {
                            blocked = true;
                        }
                        down_score += 1;
                        row += 1;
                    }
                }

                let scenic_score = left_score * up_score * right_score * down_score;
                max_scenic = std::cmp::max(scenic_score, max_scenic);
            }
        }

        max_scenic.to_string()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::util;

    #[test]
    fn test_part_one() {
        assert_eq!(
            "21",
            Day08 {}.part_one(&util::input(8, util::InputType::Example))
        )
    }

    #[test]
    fn test_part_two() {
        assert_eq!(
            "8",
            Day08 {}.part_two(&util::input(8, util::InputType::Example))
        )
    }
}
