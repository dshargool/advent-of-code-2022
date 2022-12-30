use std::fs;

fn read_puzzle_input(path: &str) -> String {
    fs::read_to_string(path).expect("Could not get the puzzle input")
}

fn parse_array_str(input: &str) -> Vec<Vec<u32>> {
    let mut result: Vec<Vec<u32>> = Vec::new();
    for line in input.lines() {
        result.push(Vec::new());
        let mut new_row = result.pop().unwrap();
        for ch in line.chars() {
            new_row.push(ch.to_digit(10).unwrap())
        }
        result.push(new_row);
    }
    return result;
}

fn is_visible_top(input: &Vec<Vec<u32>>, row: usize, col: usize) -> bool {
    let mut tallest = 0;
    for i in 0..row {
        if input[i][col] > tallest {
            tallest = input[i][col];
        }
    }
    return input[row][col] > tallest;
}

fn is_visible_bottom(input: &Vec<Vec<u32>>, row: usize, col: usize) -> bool {
    let mut tallest = 0;
    for i in (row + 1..input.len()).rev() {
        if input[i][col] > tallest {
            tallest = input[i][col];
        }
    }
    return input[row][col] > tallest;
}
fn is_visible_left(input: &Vec<Vec<u32>>, row: usize, col: usize) -> bool {
    let mut tallest = 0;
    for i in 0..col {
        if input[row][i] > tallest {
            tallest = input[row][i];
        }
    }
    return input[row][col] > tallest;
}
fn is_visible_right(input: &Vec<Vec<u32>>, row: usize, col: usize) -> bool {
    let mut tallest = 0;
    for i in (col + 1..input[0].len()).rev() {
        if input[row][i] > tallest {
            tallest = input[row][i];
        }
    }
    return input[row][col] > tallest;
}

fn is_visible(input: &Vec<Vec<u32>>) -> u32 {
    let mut visible_count = 0;
    // i is iterating over rows
    // j is iterating over columns in those rows
    let mut result: Vec<Vec<bool>> = vec![vec![false; input.len()]; input.len()];

    for i in 0..input.len() {
        for j in 0..input[0].len() {
            if i == 0 || i == input.len() - 1 || j == 0 || j == input[0].len() - 1 {
                result[i][j] = true;
            }
            if result[i][j] {
                visible_count += 1;
            }
        }
    }
    for i in 1..input.len() - 1 {
        for j in 1..input[0].len() - 1 {
            if is_visible_top(&input, i, j)
                || is_visible_bottom(&input, i, j)
                || is_visible_left(&input, i, j)
                || is_visible_right(&input, i, j)
            {
                visible_count += 1;
            }
        }
    }
    return visible_count;
}

fn num_visible_up(input: &Vec<Vec<u32>>, row: usize, col: usize) -> u32 {
    let mut num_vis = 0;
    if row == 0 {
        return 0;
    }
    for i in (0..row).rev() {
        if input[i][col] < input[row][col] {
            num_vis += 1;
        } else {
            num_vis += 1;
            return num_vis;
        }
    }
    return num_vis;
}
fn num_visible_down(input: &Vec<Vec<u32>>, row: usize, col: usize) -> u32 {
    let mut num_vis = 0;
    if row == input.len() {
        return 0;
    }
    for i in row + 1..input.len() {
        if input[i][col] < input[row][col] {
            num_vis += 1;
        } else {
            num_vis += 1;
            return num_vis;
        }
    }
    return num_vis;
}
fn num_visible_left(input: &Vec<Vec<u32>>, row: usize, col: usize) -> u32 {
    let mut num_vis = 0;
    if col == 0 {
        return 0;
    }
    for i in (0..col).rev() {
        if input[row][i] < input[row][col] {
            num_vis += 1;
        } else {
            num_vis += 1;
            return num_vis;
        }
    }
    return num_vis;
}
fn num_visible_right(input: &Vec<Vec<u32>>, row: usize, col: usize) -> u32 {
    if col == input[0].len() {
        return 0;
    }
    let mut num_vis = 0;
    for i in col + 1..input[0].len() {
        if input[row][i] < input[row][col] {
            num_vis += 1;
        } else {
            num_vis += 1;
            return num_vis;
        }
    }
    return num_vis;
}

fn scenic_score(input: &Vec<Vec<u32>>) -> u32 {
    let mut max_score = 0;
    for i in 0..input.len() {
        for j in 0..input[0].len() {
            let score = num_visible_up(input, i, j)
                * num_visible_down(input, i, j)
                * num_visible_left(input, i, j)
                * num_visible_right(input, i, j);
            if score > max_score {
                max_score = score;
            }
        }
    }
    return max_score;
}

fn part1(input: &str) {
    println!("Part1 Count: {}", is_visible(&parse_array_str(input)));
}

fn part2(input: &str) {
    println!("Part2 Count: {}", scenic_score(&parse_array_str(input)));
}

fn main() {
    let input = read_puzzle_input("./input");
    part1(&input);
    part2(&input);
}
