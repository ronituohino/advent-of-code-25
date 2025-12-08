use std::fs;

// O(n) time!
pub fn run() -> String {
    let input = fs::read_to_string("src/day3p2.txt").unwrap();
    let mut total = 0;
    for line in input.split("\n") {
        let l = line.len();
        if l == 0 {
            continue;
        }

        let mut nums: [u8; 12] = [0; 12];

        for i in 0..l {
            let num = line[i..(i + 1)].parse::<u8>().unwrap();

            let mut remaining = l - i;
            if remaining > 12 {
                remaining = 12;
            }
            let start = 12 - remaining;

            let mut found = false;
            for n in start..12 {
                if found {
                    nums[n] = 0;
                    continue;
                }
                if num > nums[n] {
                    nums[n] = num;
                    found = true;
                }
            }
        }

        let num_str = nums.map(|x| x.to_string()).join("");
        let line_num = num_str.parse::<u64>().unwrap();
        total += line_num;
    }

    return total.to_string();
}
