use std::{
    collections::{HashSet, VecDeque},
    fs, i32, vec,
};

pub fn run() -> u64 {
    let input = fs::read_to_string("src/day10.txt").unwrap();

    let mut set: HashSet<Vec<i32>> = HashSet::new();
    let mut next_states: VecDeque<Vec<i32>> = VecDeque::new();
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

        // The actual algo
        let minimum_moves = bfs(&actions, state, &mut set, &mut next_states);
        total += minimum_moves;

        println!("Found solution for line {}: {}", idx, minimum_moves);
    }

    return total;
}

fn bfs(
    actions: &Vec<Vec<usize>>,
    state: Vec<i32>,
    set: &mut HashSet<Vec<i32>>,
    next_states: &mut VecDeque<Vec<i32>>,
) -> u64 {
    set.clear();
    set.insert(state.clone());
    next_states.clear();
    next_states.push_front(state);

    let mut depth = 1;
    while next_states.len() > 0 {
        for _ in 0..next_states.len() {
            let next_state = next_states.pop_front().unwrap();
            for action in actions {
                let mut local_state = next_state.clone();
                for idx in action {
                    local_state[*idx] -= 1;
                }
                if !set.contains(&local_state) {
                    // if all equal 0, then a solution has been found
                    let mut found = true;
                    let mut invalid = false;
                    for i in 0..local_state.len() {
                        let val = local_state[i];
                        if val > 0 {
                            found = false;
                        }
                        if val < 0 {
                            invalid = true;
                        }
                    }
                    if found {
                        return depth;
                    }
                    if !invalid {
                        set.insert(local_state.clone());
                        next_states.push_back(local_state);
                    }
                }
            }
        }
        depth += 1;
    }

    return 0;
}
