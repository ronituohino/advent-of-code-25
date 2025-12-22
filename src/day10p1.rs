use std::{
    collections::{HashSet, VecDeque},
    fs, vec,
};

pub fn run() -> u64 {
    let input = fs::read_to_string("src/day10.txt").unwrap();

    let mut set: HashSet<Vec<bool>> = HashSet::new();
    let mut next_states: VecDeque<Vec<bool>> = VecDeque::new();
    let mut total = 0;

    for line in input.split("\n") {
        if line.len() == 0 {
            continue;
        }
        let entries: Vec<&str> = line.split(" ").collect();

        let mut goal = entries[0];
        goal = &goal[1..goal.len() - 1];
        let mut state: Vec<bool> = vec![];
        for c in goal.chars() {
            // reverse state
            if c == '.' {
                state.push(true);
            } else {
                state.push(false);
            }
        }

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
    }

    return total;
}

fn bfs(
    actions: &Vec<Vec<usize>>,
    state: Vec<bool>,
    set: &mut HashSet<Vec<bool>>,
    next_states: &mut VecDeque<Vec<bool>>,
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
                    local_state[*idx] = !local_state[*idx];
                }
                if !set.contains(&local_state) {
                    // if all true, then a solution has been found
                    if local_state.iter().all(|s| *s) {
                        return depth;
                    }
                    set.insert(local_state.clone());
                    next_states.push_back(local_state);
                }
            }
        }
        depth += 1;
    }

    return 0;
}
