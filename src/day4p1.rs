use std::fs;

pub fn run() -> i32 {
    let input = fs::read_to_string("src/day4p1.txt").unwrap();
    let mut map = vec![];

    let mut h = 0;
    let mut w = 0;

    for line in input.split("\n") {
        if line.len() == 0 {
            continue;
        }
        w = line.len();
        h += 1;
        for c in line.chars() {
            if c == '@' {
                map.push(1);
            } else {
                map.push(0);
            }
        }
    }

    let mut total_positions = 0;

    for y in 0..h {
        for x in 0..w {
            let c = map[y * w + x];
            if c == 0 {
                continue;
            }

            let mut total = 0;

            let y_t = y > 0;
            let y_b = y < (h - 1);
            let x_l = x > 0;
            let x_r = x < (w - 1);

            if y_t {
                if x_l {
                    total += map[(y - 1) * w + (x - 1)];
                }
                if x_r {
                    total += map[(y - 1) * w + (x + 1)];
                }
                total += map[(y - 1) * w + x];
            }

            if y_b {
                if x_l {
                    total += map[(y + 1) * w + (x - 1)];
                }
                if x_r {
                    total += map[(y + 1) * w + (x + 1)];
                }
                total += map[(y + 1) * w + x];
            }

            if x_l {
                total += map[y * w + (x - 1)];
            }

            if x_r {
                total += map[y * w + (x + 1)];
            }

            if total < 4 {
                total_positions += 1;
            }
        }
    }

    return total_positions;
}
