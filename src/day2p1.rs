use std::fs;

pub fn run() -> String {
    let tags: String = fs::read_to_string("src/day2p1.txt").unwrap();

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

            if l % 2 == 0 {
                let b = s.as_bytes();
                let half = l / 2;
                let mut is_invalid = true;
                for x in 0..half {
                    if b[half - x - 1] != b[l - x - 1] {
                        is_invalid = false;
                        break;
                    }
                }
                if is_invalid {
                    println!("{:?}, {}", b, i);
                    invalid.push(i);
                }
            }
        }
    }

    let sum: u64 = invalid.iter().sum();
    return sum.to_string();
}
