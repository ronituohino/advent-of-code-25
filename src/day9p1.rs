use std::fs;

pub fn run() -> i64 {
    let input = fs::read_to_string("src/day9.txt").unwrap();

    let mut all_coords: Vec<(i64, i64)> = vec![];
    for line in input.split("\n") {
        if line.len() == 0 {
            continue;
        }
        let coords: Vec<i64> = line.split(",").map(|n| n.parse::<i64>().unwrap()).collect();
        all_coords.push((coords[0], coords[1]));
    }

    let mut largest_area = 0;
    for i in 0..all_coords.len() {
        for j in i..all_coords.len() {
            let ixy = all_coords[i];
            let jxy = all_coords[j];

            let area = ((ixy.0 - jxy.0).abs() + 1) * ((ixy.1 - jxy.1).abs() + 1);
            if area > largest_area {
                largest_area = area;
            }
        }
    }

    return largest_area;
}
