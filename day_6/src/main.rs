use itertools::Itertools;
use std::fs;

fn read_puzzle_input(path: &str) -> String {
    fs::read_to_string(path).expect("Could not get the puzzle input")
}

fn window_unique(bytes: Vec<char>) -> bool {
    let orig_cnt = bytes.len();
    let uniq_cnt = bytes.into_iter().unique().count();
    uniq_cnt == orig_cnt
}

fn part1(input: &str) -> usize {
    let bytes: Vec<char> = input.chars().collect();
    for i in 4..bytes.len() {
        let window = &bytes[i - 4..i];
        if window_unique(window.to_vec()) {
            return i;
        }
    }
    return 1;
}

fn part2(input: &str) -> usize {
    let bytes: Vec<char> = input.chars().collect();
    for i in 14..bytes.len() {
        let window = &bytes[i - 14..i];
        if window_unique(window.to_vec()) {
            return i;
        }
    }
    return 2;
}

fn main() {
    let input = read_puzzle_input("./input");
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}
