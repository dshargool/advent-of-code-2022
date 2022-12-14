use std::fs;

struct Actions {
    start: usize,
    end: usize,
    amount: u32,
}

fn read_puzzle_input(path: &str) -> String {
    fs::read_to_string(path).expect("Could not get the puzzle input")
}

fn get_stacks(input: &str) -> Vec<Vec<char>> {
    let mut stacks: Vec<Vec<char>> = Vec::new();
    for line in input.lines() {
        if line.contains("move") {
            break;
        }

        let byte_str: Vec<char> = line.chars().collect();

        for i in 0..byte_str.len() {
            if byte_str[i] == '[' {
                // println!("Char: {}, Pos: {}", byte_str[i + 1], i / 4);
                while stacks.len() <= i / 4 {
                    stacks.push(Vec::new());
                }
                stacks[i / 4].push(byte_str[i + 1]);
            }
        }
    }
    let mut rev_stacks: Vec<Vec<char>> = Vec::new();
    for mut stack in stacks {
        stack.reverse();
        rev_stacks.push(stack);
    }
    return rev_stacks;
}

fn parse_move_line(line: &str) -> Actions {
    let words: Vec<&str> = line.split_whitespace().collect();
    let action = Actions {
        amount: words[1].parse().unwrap(),
        start: words[3].parse().unwrap(),
        end: words[5].parse().unwrap(),
    };
    return action;
}

fn move_crates(input: &str, mut stacks: Vec<Vec<char>>) -> String {
    for line in input.lines() {
        if line.contains("move") {
            let actions = parse_move_line(line);
            for _i in 0..actions.amount as usize {
                let item = stacks[actions.start - 1].pop().unwrap();
                stacks[actions.end - 1].push(item);
            }
        }
    }
    println!("{:?}", stacks);
    let mut result_str: String = String::new();
    for mut col in stacks {
        result_str.push_str(&col.pop().unwrap().to_string());
    }
    return result_str;
}
fn move_multi_crates(input: &str, mut stacks: Vec<Vec<char>>) -> String {
    for line in input.lines() {
        if line.contains("move") {
            let actions = parse_move_line(line);
            let mut items: Vec<char> = Vec::new();
            for _i in 0..actions.amount as usize {
                let item = stacks[actions.start - 1].pop().unwrap();
                items.push(item);
            }
            for _i in 0..actions.amount as usize {
                stacks[actions.end - 1].push(items.pop().unwrap());
            }
        }
    }
    println!("{:?}", stacks);
    let mut result_str: String = String::new();
    for mut col in stacks {
        result_str.push_str(&col.pop().unwrap().to_string());
    }
    return result_str;
}

fn part1(input: &str) -> String {
    let stacks = get_stacks(input);
    println!("{:?}", stacks);
    return move_crates(input, stacks);
}

fn part2(input: &str) -> String {
    let stacks = get_stacks(input);
    println!("{:?}", stacks);
    return move_multi_crates(input, stacks);
}

fn main() {
    println!("Hello, world!");
    let input = read_puzzle_input("./input");
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}
