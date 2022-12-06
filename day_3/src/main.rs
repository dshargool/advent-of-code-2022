use itertools::Itertools;
use std::fs;

fn get_priority(c: char) -> u32 {
    if c.is_uppercase() {
        return c.to_digit(36).expect("Can't convert uppercase to digit") + 17;
    } else {
        return c.to_digit(36).expect("Can't convert to digit") - 9;
    }
}

fn read_puzzle_input(path: &str) -> String {
    fs::read_to_string(path).expect("Could not get the puzzle input")
}

fn find_common_char(str1: &str, str2: &str) -> Option<char> {
    for c1 in str1.chars() {
        for c2 in str2.chars() {
            if c1 == c2 {
                return Some(c1);
            }
        }
    }
    return None;
}

fn part_one(input: &str) {
    let mut score = 0;
    for bag in input.lines() {
        let compartment = bag.split_at(bag.len() / 2);

        //println!("{:?}", compartment.0);
        //println!("{:?}", compartment.1);

        let common_char =
            find_common_char(compartment.0, compartment.1).expect("No matching value");
        score += get_priority(common_char);
    }
    println!("{:?}", score);
}

fn part_two(input: &str) {
    let mut score = 0;
    for lines in &input.lines().chunks(3) {
        for (elf1, elf2, elf3) in lines.tuples() {
            // Only add the characters that appear in all strings!
            let comm_char = elf1
                .chars()
                .into_iter()
                .filter(|d| elf2.contains(*d) && elf3.contains(*d))
                .nth(0)
                .unwrap();
            println!("{:?}", comm_char);
            score += get_priority(comm_char);
        }
    }
    println!("Score: {}", score);
}

fn main() {
    let input = read_puzzle_input("./input");
    part_one(&input);
    part_two(&input);
}
