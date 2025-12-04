use std::fs;

#[derive(Debug)]
struct Dial {
    size: i32,
    position: i32,
}

impl Dial {
    fn turn(&mut self, direction: &char, distance: &i32) -> i32 {
        let mut remaining_distance = distance.clone();
        let mut hits = 0;
        // dumb but works because distance are small :D
        if direction == &'L' {
            while remaining_distance > 0 {
                self.position -= 1;
                if self.position < 0 {
                    self.position = 99;
                }
                if self.position == 0 {
                    hits += 1
                }
                remaining_distance -= 1;
            }
        }
        if direction == &'R' {
            while remaining_distance > 0 {
                self.position += 1;
                if self.position > 99 {
                    self.position = 0;
                }
                if self.position == 0 {
                    hits += 1
                }
                remaining_distance -= 1;
            }
        }
        return hits;
    }
}

pub fn run() -> String {
    let message: String = fs::read_to_string("src/day1p2.txt").unwrap();

    let mut dial = Dial {
        size: 100,
        position: 50,
    };

    let mut hits = 0;

    for s in message.split('\n') {
        let len = s.len();
        if len == 0 {
            continue;
        }
        let dir = &s.chars().nth(0).unwrap();
        let dist = &s[1..len].parse::<i32>().unwrap();
        hits += dial.turn(dir, dist);
    }

    return hits.to_string();
}
