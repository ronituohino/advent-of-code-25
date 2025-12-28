use std::{collections::HashSet, fs, i32, vec};

pub fn run() -> usize {
    let input = fs::read_to_string("src/day10.txt").unwrap();

    let mut set: HashSet<Vec<i32>> = HashSet::new();
    let mut total = 0;

    for (idx, line) in input.split("\n").enumerate() {
        if line.len() == 0 {
            continue;
        }
        let entries: Vec<&str> = line.split(" ").collect();

        let goal = entries[entries.len() - 1];
        let state: Vec<i32> = goal[1..(goal.len() - 1)]
            .split(",")
            .map(|a| a.parse::<i32>().unwrap())
            .collect();

        let mut actions: Vec<Vec<usize>> = vec![];
        for i in 1..(entries.len() - 1) {
            let action = entries[i];
            let action_indices: Vec<usize> = action[1..(action.len() - 1)]
                .split(",")
                .map(|a| a.parse::<usize>().unwrap())
                .collect();
            actions.push(action_indices);
        }

        // Make actions have the longest action first
        actions.sort_by(|a, b| b.len().cmp(&a.len()));

        set.clear();
        let moves = dfs(&actions, state, 1, &mut set);
        total += moves;

        println!("Found solution for line {}: {}", idx, moves);
    }

    return total;
}

fn dfs(
    actions: &Vec<Vec<usize>>,
    state: Vec<i32>,
    depth: usize,
    set: &mut HashSet<Vec<i32>>,
) -> usize {
    for action in actions {
        let mut new_state = state.clone();
        for idx in action {
            new_state[*idx] -= 1;
        }

        if set.contains(&new_state) {
            continue;
        } else {
            set.insert(new_state.clone());
        }

        // if all equal 0, then a solution has been found
        let mut found = true;
        let mut invalid = false;
        for val in &new_state {
            if *val > 0 {
                found = false;
            }
            if *val < 0 {
                invalid = true;
                break;
            }
        }
        if !invalid {
            if found {
                return depth;
            }

            let res = dfs(actions, new_state.clone(), depth + 1, set);
            if res != 0 {
                return res;
            }
        }
    }

    return 0;
}
