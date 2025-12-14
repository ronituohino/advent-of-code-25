use std::{collections::HashMap, fs};

pub fn run() -> u64 {
    let input = fs::read_to_string("src/day7p2.txt").unwrap();

    let lines: Vec<&str> = input.split("\n").collect();
    let first_line = lines[0];
    let start = first_line.chars().position(|c| c == 'S').unwrap();
    let mut end_reached: HashMap<(usize, usize), u64> = HashMap::new();

    return simulate(&lines, 0, start, &mut end_reached);
}

fn simulate(
    lines: &Vec<&str>,
    current_row: usize,
    beam_index: usize,
    end_reached: &mut HashMap<(usize, usize), u64>,
) -> u64 {
    let mut p1 = 0;
    let mut p2 = 0;
    let mut reached_end = false;

    for i in current_row..lines.len() {
        if let Some(paths) = end_reached.get(&(i, beam_index)) {
            return *paths;
        }

        let line = lines[i];
        if line.len() == 0 {
            continue;
        }

        if i == lines.len() - 1 {
            reached_end = true;
        }

        let c = line.chars().nth(beam_index).unwrap();
        if c == '^' {
            if beam_index > 0 {
                p1 = simulate(lines, i + 1, beam_index - 1, end_reached);
                end_reached.insert((i + 1, beam_index - 1), p1);
            }
            if beam_index < line.len() - 1 {
                p2 = simulate(lines, i + 1, beam_index + 1, end_reached);
                end_reached.insert((i + 1, beam_index + 1), p2);
            }
            break;
        }
    }

    if reached_end {
        return 1;
    }

    return p1 + p2;
}
