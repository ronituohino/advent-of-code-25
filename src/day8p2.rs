use std::cmp::{Ordering, Reverse};
use std::vec;
use std::{
    collections::{BinaryHeap, HashMap},
    fs,
};

#[derive(Debug, Copy, Clone, PartialEq)]
struct BoxPair {
    box_a: usize,
    box_b: usize,
    distance: f64,
}
impl Eq for BoxPair {}
impl Ord for BoxPair {
    fn cmp(&self, other: &Self) -> Ordering {
        // Notice that we flip the ordering creating a min-heap.
        // In case of a tie we compare id's.
        other
            .distance
            .total_cmp(&self.distance)
            .then_with(|| self.box_a.cmp(&other.box_b))
    }
}
impl PartialOrd for BoxPair {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn all_in_same_circuit(id_to_circuit: &Vec<u64>) -> bool {
    let x = id_to_circuit[0];
    for c in id_to_circuit {
        if *c == 0 || *c != x {
            return false;
        }
    }
    return true;
}

pub fn run() -> i64 {
    let mut id_to_box: Vec<(i64, i64, i64)> = vec![];
    let mut id_to_circuit: Vec<u64> = vec![];
    let mut running_circuit_number = 1;
    let mut distances_between_boxes: BinaryHeap<BoxPair> = BinaryHeap::new();

    let input = fs::read_to_string("src/day8.txt").unwrap();

    for line in input.split("\n") {
        if line.len() == 0 {
            continue;
        }
        let coords = line
            .split(",")
            .map(|x| x.parse::<i64>().unwrap())
            .collect::<Vec<i64>>();
        id_to_box.push((coords[0], coords[1], coords[2]));
        id_to_circuit.push(0);
    }

    for idx in 0..id_to_box.len() {
        for jdx in idx..id_to_box.len() {
            if idx == jdx {
                continue;
            }

            let ib = id_to_box[idx];
            let jb = id_to_box[jdx];

            let dist = (((ib.0 - jb.0).pow(2) + (ib.1 - jb.1).pow(2) + (ib.2 - jb.2).pow(2))
                as f64)
                .sqrt();
            distances_between_boxes.push(BoxPair {
                box_a: idx,
                box_b: jdx,
                distance: dist,
            });
        }
    }

    let mut iterations = 0;
    let mut last_boxes_to_connect: Option<BoxPair> = None;
    while !all_in_same_circuit(&id_to_circuit) {
        let bp = distances_between_boxes.pop().unwrap();

        match (id_to_circuit[bp.box_a], id_to_circuit[bp.box_b]) {
            (0, 0) => {
                // create new circuit
                id_to_circuit[bp.box_a] = running_circuit_number;
                id_to_circuit[bp.box_b] = running_circuit_number;
                running_circuit_number += 1;
                last_boxes_to_connect = Some(bp);
            }
            (a, 0) if a > 0 => {
                // set b to connect to a
                id_to_circuit[bp.box_b] = a;
                last_boxes_to_connect = Some(bp);
            }
            (0, b) if b > 0 => {
                // set a to connect to b
                id_to_circuit[bp.box_a] = b;
                last_boxes_to_connect = Some(bp);
            }
            (a, b) if a > 0 && b > 0 && a != b => {
                // set all to same, ugly O(n) :<
                for i in 0..id_to_circuit.len() {
                    if id_to_circuit[i] == b {
                        id_to_circuit[i] = a;
                    }
                }
                last_boxes_to_connect = Some(bp);
            }
            (_, _) => (),
        };

        iterations += 1;
    }

    println!("Connections done: {}", iterations);

    if let Some(b) = last_boxes_to_connect {
        return id_to_box[b.box_a].0 * id_to_box[b.box_b].0;
    }
    return 0;
}
