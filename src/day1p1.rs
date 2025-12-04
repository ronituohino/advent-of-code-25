use std::fs;

#[derive(Debug)]
struct Dial {
    size: i32,
    position: i32,
}

impl Dial {
    fn turn(&mut self, direction: &char, distance: &i32) {
        match direction {
            'L' => {
                self.position = (self.position - distance) % self.size;
            }
            'R' => {
                self.position = (self.position + distance) % self.size;
            }
            _ => {}
        }
    }
}

pub fn run() -> String {
    let message: String = fs::read_to_string("src/day1p1.txt").unwrap();

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
        dial.turn(dir, dist);

        if dial.position == 0 {
            hits += 1;
        }
    }

    return hits.to_string();
}
