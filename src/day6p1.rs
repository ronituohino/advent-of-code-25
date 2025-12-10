use std::{fs, vec};

pub fn run() -> u64 {
    let input = fs::read_to_string("src/day6p1.txt").unwrap();
    let lines = input.split("\n");

    let mut numbers: Vec<Vec<u64>> = vec![];
    let mut operators: Vec<&str> = vec![];

    for (row, line) in lines.enumerate() {
        let mut column = 0;
        for entry in line.split(" ") {
            if entry.len() == 0 {
                continue;
            }
            if row == 0 {
                numbers.push(Vec::new());
            }

            if let Ok(num) = entry.parse::<u64>() {
                numbers[column].push(num);
            } else {
                operators.push(entry);
            }
            column += 1;
        }
    }

    let mut result = 0;
    for (idx, set) in numbers.iter().enumerate() {
        let op = operators[idx];

        if op == "*" {
            let mut total: u64 = set[0];
            for n in 1..set.len() {
                total *= set[n];
            }
            result += total;
        }
        if op == "+" {
            let total: u64 = set.iter().sum();
            result += total;
        }
    }

    return result;
}
