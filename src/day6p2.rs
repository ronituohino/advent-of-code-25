use std::fs;

// Approach:
// save number with line indices
// then form similar number array as in previous problem

pub fn run() -> u64 {
    let input = fs::read_to_string("src/day6p2.txt").unwrap();
    let lines = input.split("\n");

    let mut numbers: Vec<Vec<u64>> = vec![];
    let mut operators: Vec<String> = vec![];

    let mut init = false;

    for line in lines {
        if !init {
            let len = line.len();
            for _ in 0..len {
                numbers.push(Vec::new());
                operators.push(String::from(" "))
            }
            init = true;
        }

        for (column, character) in line.chars().enumerate() {
            if character == ' ' {
                continue;
            }

            if let Ok(num) = character.to_string().parse::<u64>() {
                numbers[column].push(num);
            } else {
                operators[column] = character.to_string();
            }
        }
    }

    let mut result = 0;
    let mut op = String::from("*");
    let mut set: Vec<u64> = vec![];

    for (idx, arr) in numbers.iter().enumerate() {
        if arr.len() == 0 {
            continue;
        }

        let new_operator = operators[idx].clone();
        if new_operator != " " {
            // collapse set
            collapse(&mut result, op, &mut set);
            op = new_operator;
        }

        // add new items
        let mut new_num = String::from("");
        for i in 0..arr.len() {
            new_num.push_str(&arr[i].to_string());
        }
        set.push(new_num.parse::<u64>().unwrap());
    }

    // final collapse
    collapse(&mut result, op, &mut set);

    return result;
}

fn collapse(result: &mut u64, op: String, set: &mut Vec<u64>) {
    let l = set.len();
    if l > 0 {
        if op == "*" {
            let mut total: u64 = set[0];
            for n in 1..l {
                total *= set[n];
            }
            *result += total;
        }
        if op == "+" {
            let total: u64 = set.iter().sum();
            *result += total;
        }
        set.clear();
    }
}
