use std::fs;

pub fn run() -> i64 {
    let input = fs::read_to_string("src/day9.txt").unwrap();

    let mut red_coords: Vec<(i64, i64)> = vec![];
    for line in input.split("\n") {
        if line.len() == 0 {
            continue;
        }
        let coords: Vec<i64> = line.split(",").map(|n| n.parse::<i64>().unwrap()).collect();
        red_coords.push((coords[0], coords[1]));
    }

    let mut edges: Vec<((i64, i64), (i64, i64))> = vec![];
    for i in 0..red_coords.len() {
        let mut j = i + 1;
        if j >= red_coords.len() {
            j = 0;
        }
        let ixy = red_coords[i];
        let jxy = red_coords[j];
        edges.push((ixy, jxy));
    }

    let mut largest_area = 0;
    for i in 0..red_coords.len() {
        for j in i..red_coords.len() {
            let ixy = red_coords[i];
            let jxy = red_coords[j];

            let area = ((ixy.0 - jxy.0).abs() + 1) * ((ixy.1 - jxy.1).abs() + 1);
            if area > largest_area {
                // First check if this area is larger, then check if it's valid

                // Edge check with cross product

                largest_area = area;
            }
        }
    }

    return 0;
}
