use std::fs;

struct Cleaner {
    lower: u32,
    higher: u32,
}

fn read_puzzle_input(path: &str) -> String {
    fs::read_to_string(path).expect("Could not get the puzzle input")
}

fn contains_other(elf1: Cleaner, elf2: Cleaner) -> bool {
    if elf1.lower <= elf2.lower && elf1.higher >= elf2.higher {
        return true;
    }
    if elf1.lower >= elf2.lower && elf1.higher <= elf2.higher {
        return true;
    }
    return false;
}

fn has_overlap(elf1: Cleaner, elf2: Cleaner) -> bool {
    let elf1_range = elf1.lower..=elf1.higher;
    let elf2_range = elf2.lower..=elf2.higher;
    let overlap: Vec<u32> = elf2_range
        .into_iter()
        .filter(|v| elf1_range.contains(v))
        .collect();
    //println!("{:?}", overlap);
    overlap.len() > 0
}

fn part1(input: &str) {
    let mut score = 0;
    for line in input.lines() {
        let pair = line.split_once(",").unwrap();
        let elf1 = Cleaner {
            lower: pair.0.split_once("-").unwrap().0.parse().unwrap(),
            higher: pair.0.split_once("-").unwrap().1.parse().unwrap(),
        };
        let elf2 = Cleaner {
            lower: pair.1.split_once("-").unwrap().0.parse().unwrap(),
            higher: pair.1.split_once("-").unwrap().1.parse().unwrap(),
        };
        if contains_other(elf1, elf2) {
            score += 1;
        }
    }
    println!("{}", score);
}

fn part2(input: &str) {
    let mut score = 0;
    for line in input.lines() {
        let pair = line.split_once(",").unwrap();
        let elf1 = Cleaner {
            lower: pair.0.split_once("-").unwrap().0.parse().unwrap(),
            higher: pair.0.split_once("-").unwrap().1.parse().unwrap(),
        };
        let elf2 = Cleaner {
            lower: pair.1.split_once("-").unwrap().0.parse().unwrap(),
            higher: pair.1.split_once("-").unwrap().1.parse().unwrap(),
        };
        if has_overlap(elf1, elf2) {
            score += 1;
        }
    }
    println!("{}", score);
}

fn main() {
    let input = read_puzzle_input("./input");
    part1(&input);
    part2(&input);
}
