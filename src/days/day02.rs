use crate::days::Problem;

pub struct Day02 {}

impl Problem for Day02 {
    fn part_one(&self, input: &str) -> String {
        let mut score = 0;
        for line in input.split("\n").collect::<Vec<&str>>().iter() {
            if !line.is_empty() {
                score += Round::from(line).round_score;
            }
        }

        score.to_string()
    }

    fn part_two(&self, input: &str) -> String {
        let mut score = 0;
        for line in input.split("\n").collect::<Vec<&str>>().iter() {
            if !line.is_empty() {
                score += RoundTwo::from(line).round_score;
            }
        }

        score.to_string()
    }
}

struct RoundTwo {
    round_score: u32,
}

impl RoundTwo {
    fn from(input: &str) -> Self {
        let opponent_move = Move::from(input.split(" ").collect::<Vec<&str>>()[0]);
        let your_move: Move;
        let winner: Winner;

        match input.split(" ").collect::<Vec<&str>>()[1] {
            "X" => {
                //lose
                match opponent_move.shape {
                    Shape::Rock => {
                        your_move = Move {
                            shape: Shape::Scissors,
                            move_score: 3,
                        };
                    }
                    Shape::Paper => {
                        your_move = Move {
                            shape: Shape::Rock,
                            move_score: 1,
                        };
                    }
                    Shape::Scissors => {
                        your_move = Move {
                            shape: Shape::Paper,
                            move_score: 2,
                        };
                    }
                };
                winner = Winner::Opponent;
            }
            "Y" => {
                //draw
                match opponent_move.shape {
                    Shape::Rock => {
                        your_move = Move {
                            shape: Shape::Rock,
                            move_score: 1,
                        };
                    }
                    Shape::Paper => {
                        your_move = Move {
                            shape: Shape::Paper,
                            move_score: 2,
                        };
                    }
                    Shape::Scissors => {
                        your_move = Move {
                            shape: Shape::Scissors,
                            move_score: 3,
                        };
                    }
                };
                winner = Winner::Draw;
            }
            "Z" => {
                //win
                match opponent_move.shape {
                    Shape::Rock => {
                        your_move = Move {
                            shape: Shape::Paper,
                            move_score: 2,
                        };
                    }
                    Shape::Paper => {
                        your_move = Move {
                            shape: Shape::Scissors,
                            move_score: 3,
                        };
                    }
                    Shape::Scissors => {
                        your_move = Move {
                            shape: Shape::Rock,
                            move_score: 1,
                        };
                    }
                };
                winner = Winner::You;
            }
            _ => panic!("Error: Unknown expected result"),
        };

        Self {
            round_score: match winner {
                Winner::You => 6 + your_move.move_score,
                Winner::Opponent => 0 + your_move.move_score,
                Winner::Draw => 3 + your_move.move_score,
            },
        }
    }
}

enum Winner {
    You,
    Opponent,
    Draw,
}

struct Round {
    round_score: u32,
}

impl Round {
    fn from(input: &str) -> Self {
        let opponent_move = Move::from(input.split(" ").collect::<Vec<&str>>()[0]);
        let your_move = Move::from(input.split(" ").collect::<Vec<&str>>()[1]);

        let winner = match opponent_move.shape {
            Shape::Rock => match your_move.shape {
                Shape::Rock => Winner::Draw,
                Shape::Paper => Winner::You,
                Shape::Scissors => Winner::Opponent,
            },
            Shape::Paper => match your_move.shape {
                Shape::Rock => Winner::Opponent,
                Shape::Paper => Winner::Draw,
                Shape::Scissors => Winner::You,
            },
            Shape::Scissors => match your_move.shape {
                Shape::Rock => Winner::You,
                Shape::Paper => Winner::Opponent,
                Shape::Scissors => Winner::Draw,
            },
        };

        let mut total_score = 0;

        match winner {
            Winner::You => {
                total_score += your_move.move_score + 6;
            }
            Winner::Opponent => {
                total_score += your_move.move_score + 0;
            }
            Winner::Draw => {
                total_score += your_move.move_score + 3;
            }
        }

        Self {
            round_score: total_score,
        }
    }
}

enum Shape {
    Rock,
    Paper,
    Scissors,
}

struct Move {
    shape: Shape,
    move_score: u32,
}

impl Move {
    fn from(input: &str) -> Self {
        match input {
            "A" => Self {
                shape: Shape::Rock,
                move_score: 1,
            },
            "B" => Self {
                shape: Shape::Paper,
                move_score: 2,
            },
            "C" => Self {
                shape: Shape::Scissors,
                move_score: 3,
            },
            "X" => Self {
                shape: Shape::Rock,
                move_score: 1,
            },
            "Y" => Self {
                shape: Shape::Paper,
                move_score: 2,
            },
            "Z" => Self {
                shape: Shape::Scissors,
                move_score: 3,
            },
            _ => panic!("Error: Unknown move"),
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
            "15",
            Day02 {}.part_one(&util::input(2, util::InputType::Example))
        )
    }

    #[test]
    fn test_part_two() {
        assert_eq!(
            "12",
            Day02 {}.part_two(&util::input(2, util::InputType::Example))
        )
    }
}
