use std::fs;

pub fn run() -> i32 {
    let input = fs::read_to_string("src/day7p1.txt").unwrap();

    let mut beams: Vec<bool> = vec![];
    let mut total_splits = 0;
    let mut line_length = 0;
    let mut init = false;

    for line in input.split("\n") {
        if line.len() == 0 {
            continue;
        }
        if !init {
            line_length = line.len();
            for _ in 0..line_length {
                beams.push(false);
            }
            init = true;
        }

        for (idx, c) in line.chars().enumerate() {
            if c == 'S' {
                beams[idx] = true;
            }
            if c == '^' && beams[idx] {
                if idx > 0 {
                    beams[idx - 1] = true;
                }
                if idx < line_length - 1 {
                    beams[idx + 1] = true;
                }
                beams[idx] = false;
                total_splits += 1;
            }
        }
    }
    return total_splits;
}
