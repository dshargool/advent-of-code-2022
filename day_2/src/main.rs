use std::fs;

fn read_puzzle_input(path: &str) -> String {
    fs::read_to_string(path).expect("Could not get the puzzle input")
}

struct Round {
    elf: RPS,
    mine: RPS,
}

#[derive(PartialEq)]
enum RPS {
    None,
    Rock,
    Paper,
    Scissors,
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
}

fn main() {
    let input = read_puzzle_input("./input");
    let mut score = 0;

    for line in input.lines() {
        let round = Round::new(line);
        score += round.my_game_score();
        score += round.mine as i32;
    }
    println!("{:?}", score)
}
