use std::fs;

fn read_puzzle_input(path: &str) -> String {
    fs::read_to_string(path).expect("Could not get the puzzle input")
}

struct Round {
    elf: RPS,
    mine: RPS,
}

#[derive(PartialEq, Clone, Copy, Debug)]
enum RPS {
    None,
    Rock,
    Paper,
    Scissors,
}

#[derive(PartialEq, Debug)]
enum Outcome {
    None,
    Lose,
    Tie,
    Win,
}
fn rps_to_outcome(val: RPS) -> Outcome {
    match val {
        RPS::Rock => Outcome::Lose,
        RPS::Paper => Outcome::Tie,
        RPS::Scissors => Outcome::Win,
        RPS::None => Outcome::None,
    }
}

fn convert_letter_to_score(letter: &str) -> RPS {
    match letter {
        "A" | "X" => RPS::Rock,
        "B" | "Y" => RPS::Paper,
        "C" | "Z" => RPS::Scissors,
        &_ => RPS::None,
    }
}

impl Round {
    pub fn new(input: &str) -> Self {
        let input = input.split(" ").collect::<Vec<&str>>();

        Self {
            elf: convert_letter_to_score(input[0]),
            mine: convert_letter_to_score(input[1]),
        }
    }
    fn my_game_score(&self) -> i32 {
        if self.mine == self.elf {
            return 3;
        }
        if self.mine == RPS::Rock && self.elf == RPS::Scissors {
            return 6;
        }
        if self.mine == RPS::Paper && self.elf == RPS::Rock {
            return 6;
        }
        if self.mine == RPS::Scissors && self.elf == RPS::Paper {
            return 6;
        }
        return 0;
    }

    fn set_hand_shape(&mut self) {
        let desired_outcome = rps_to_outcome(self.mine);
        let mut shape: RPS = RPS::None;
        if desired_outcome == Outcome::Win {
            if self.elf == RPS::Rock {
                shape = RPS::Paper;
            }
            if self.elf == RPS::Paper {
                shape = RPS::Scissors;
            }
            if self.elf == RPS::Scissors {
                shape = RPS::Rock;
            }
        } else if desired_outcome == Outcome::Tie {
            shape = self.elf;
        } else if desired_outcome == Outcome::Lose {
            if self.elf == RPS::Rock {
                shape = RPS::Scissors;
            } else if self.elf == RPS::Paper {
                shape = RPS::Rock;
            } else {
                shape = RPS::Paper;
            }
        }
        self.mine = shape;
    }
}

fn main() {
    let input = read_puzzle_input("./input");
    let mut score = 0;

    for line in input.lines() {
        let mut round = Round::new(line);
        round.set_hand_shape();
        score += round.my_game_score();
        score += round.mine as i32;
    }
    println!("{:?}", score)
}
