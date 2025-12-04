use std::fs;

pub fn run() -> String {
    let tags: String = fs::read_to_string("src/day2p2.txt").unwrap();

    let mut invalid: Vec<u64> = Vec::new();
    for range in tags.split(",") {
        if range.len() == 0 {
            continue;
        }

        let s = range
            .split("-")
            .map(|x| x.parse::<u64>().unwrap())
            .collect::<Vec<u64>>();
        let min = s[0];
        let max = s[1];

        for i in min..=max {
            let s = i.to_string();
            let l = s.len();

            if l < 2 {
                continue;
            }

            let seq_max_len = l / 2;
            let mut seq_lengths = vec![1];

            for x in 2..=seq_max_len {
                if l % x == 0 {
                    seq_lengths.push(x);
                }
            }

            seq_lengths.reverse();

            for x in seq_lengths {
                let mut is_invalid = true;

                let subs_a = &s[0..x];
                for offset in 1..(l / x) {
                    let subs_b = &s[(offset * x)..((offset + 1) * x)];
                    if subs_a != subs_b {
                        is_invalid = false;
                        break;
                    }
                }

                if is_invalid {
                    invalid.push(i);
                    break;
                }
            }
        }
    }

    let sum: u64 = invalid.iter().sum();
    return sum.to_string();
}
