use std::fs;

// O(n) time!
pub fn run() -> String {
    let input = fs::read_to_string("src/day3p1.txt").unwrap();
    let mut total = 0;
    for line in input.split("\n") {
        let l = line.len();
        if l == 0 {
            continue;
        }

        let mut prior: u8 = 0;
        let mut prior_set = false;
        let mut latter: u8 = 0;

        for i in 0..l {
            let num = line[i..(i + 1)].parse::<u8>().unwrap();

            if prior_set {
                latter = num;
                prior_set = false;
            } else if num > latter {
                latter = num;
            }

            if i != (l - 1) && num > prior {
                prior = num;
                prior_set = true;
            }
        }

        let line_num = format!("{}{}", prior, latter).parse::<u64>().unwrap();
        total += line_num;
    }

    return total.to_string();
}
