use std::fs;

// could probably make some fancy tree structure...
// but I'm just gonna brute force this :)

pub fn run() -> i32 {
    let input = fs::read_to_string("src/day5p1.txt").unwrap();
    let input_split: Vec<&str> = input.split("\n\n").collect();
    let ranges = input_split[0];
    let ingredients = input_split[1];

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

    let mut fresh_ingredients = 0;
    for ing in ingredients.split("\n") {
        if ing.len() == 0 {
            continue;
        }
        let i = ing.parse::<u64>().unwrap();

        for ir in int_ranges.as_slice() {
            if i >= ir.0 && i <= ir.1 {
                fresh_ingredients += 1;
                break;
            }
        }
    }

    return fresh_ingredients;
}
