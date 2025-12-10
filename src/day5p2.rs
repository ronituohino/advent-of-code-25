use std::fs;

pub fn run() -> u64 {
    let input = fs::read_to_string("src/day5p2.txt").unwrap();
    let input_split: Vec<&str> = input.split("\n\n").collect();
    let ranges = input_split[0];
    let mut int_ranges = vec![];

    for range in ranges.split("\n") {
        if range.len() == 0 {
            continue;
        }
        let r: Vec<&str> = range.split("-").collect();
        let min = r[0].parse::<u64>().unwrap();
        let max = r[1].parse::<u64>().unwrap();

        int_ranges.push((min, max));
    }

    let mut finished = false;
    let mut i = 0;
    while !finished {
        for j in 0..i {
            // Approach:
            // - modify int_ranges to have ranges that do not overlap each other
            // - always keep 'previous' ranges as not overlapping
            let cur = int_ranges[i];
            let prev = int_ranges[j];

            if cur.0 >= prev.0 && cur.1 <= prev.1 {
                // cur is completely contained by prev range
                int_ranges.remove(i);
                i -= 1;
                break;
            }
            if cur.0 < prev.0 && cur.1 >= prev.0 && cur.1 <= prev.1 {
                // right side of cur is partially contained by prev
                int_ranges[i].1 = prev.0 - 1;
            }
            if cur.0 >= prev.0 && cur.0 <= prev.1 && cur.1 > prev.1 {
                // left side of cur is partially contained by prev
                int_ranges[i].0 = prev.1 + 1;
            }
            if cur.0 < prev.0 && cur.1 > prev.1 {
                // cur is larger than a prev range, and contains prev
                int_ranges.push((cur.0, prev.0 - 1));
                int_ranges.push((prev.1 + 1, cur.1));
                int_ranges.remove(i);
                i -= 1;
                break;
            }
            // in other cases cur and prev do not have overlap.
        }

        i += 1;
        if i == int_ranges.len() {
            finished = true;
        }
    }

    let mut fresh_ingredients = 0;
    for range in int_ranges {
        fresh_ingredients += range.1 - range.0 + 1;
    }

    return fresh_ingredients;
}
