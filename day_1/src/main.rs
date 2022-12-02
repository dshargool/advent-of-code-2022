use std::fs;

#[derive(Debug)]
struct Elf {
    snacks: Vec<i32>,
}

impl Elf {
    pub fn new() -> Self {
        Self { snacks: Vec::new() }
    }
    pub fn snack_total(&self) -> i32 {
        let mut snack_total = 0;
        for snack in self.snacks.iter() {
            snack_total += snack;
        }
        return snack_total;
    }
}

fn read_puzzle_input(path: &str) -> Result<String, std::io::Error> {
    fs::read_to_string(path)
}

fn main() {
    let mut elves: Vec<Elf> = Vec::new();
    let foods = read_puzzle_input("./input").expect("Could not get the puzzle input");
    let mut elf = Elf::new();
    for line in foods.lines() {
        println!("{}", line.to_string());
        match line.parse::<i32>() {
            Err(_e) => {
                elves.push(elf);
                elf = Elf::new();
            }
            Ok(snack) => {
                elf.snacks.push(snack);
            }
        };
    }
    // Push the last elf onto our vector
    elves.push(elf);
    println!("{:?}", elves);
    /*
    let mut elf_snack_totals: Vec<i32> = Vec::new();
    for elf in elves.into_iter() {
        elf_snack_totals.push(elf.snack_total());
    }*/
    let mut top_total: Vec<i32> = vec![0, 0, 0];
    for elf in elves.into_iter() {
        if elf.snack_total() > top_total[0] {
            top_total[2] = top_total[1];
            top_total[1] = top_total[0];
            top_total[0] = elf.snack_total();
        } else if elf.snack_total() > top_total[1] {
            top_total[2] = top_total[1];
            top_total[1] = elf.snack_total();
        } else if elf.snack_total() > top_total[2] {
            top_total[2] = elf.snack_total();
        }
    }
    let sum: i32 = top_total.iter().sum();
    println!("{:?}", top_total);

    println!("{:?}", sum);
}
