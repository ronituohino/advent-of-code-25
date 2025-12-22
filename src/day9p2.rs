use std::{
    cmp::{max, min},
    fs,
};

#[derive(Eq, Hash, PartialEq)]
struct Point {
    x: i64,
    y: i64,
}

pub fn run() -> i64 {
    let input = fs::read_to_string("src/day9.txt").unwrap();

    let mut red_coords: Vec<Point> = vec![];
    for line in input.split("\n") {
        if line.len() == 0 {
            continue;
        }
        let coords: Vec<i64> = line.split(",").map(|n| n.parse::<i64>().unwrap()).collect();
        red_coords.push(Point {
            x: coords[0],
            y: coords[1],
        });
    }

    let mut edges: Vec<(&Point, &Point)> = vec![];
    for i in 0..red_coords.len() {
        let mut j = i + 1;
        if j >= red_coords.len() {
            j = 0;
        }
        let ixy = &red_coords[i];
        let jxy = &red_coords[j];
        edges.push((ixy, jxy));
    }

    let mut largest_area = 0;
    for i in 0..red_coords.len() {
        for j in i..red_coords.len() {
            let ixy = &red_coords[i];
            let jxy = &red_coords[j];

            // First check if this area is larger, then check if it's valid
            let area = ((ixy.x - jxy.x).abs() + 1) * ((ixy.y - jxy.y).abs() + 1);
            if area > largest_area
                && is_rect_in_polygon(
                    &Rect {
                        min_x: min(ixy.x, jxy.x),
                        max_x: max(ixy.x, jxy.x),
                        min_y: min(ixy.y, jxy.y),
                        max_y: max(ixy.y, jxy.y),
                    },
                    &edges,
                )
            {
                println!("Found new largest area: {}", area);
                largest_area = area;
            }
        }
    }

    return largest_area;
}

#[derive(Debug)]
struct Rect {
    min_x: i64,
    min_y: i64,
    max_x: i64,
    max_y: i64,
}

fn is_rect_in_polygon(rect: &Rect, edges: &Vec<(&Point, &Point)>) -> bool {
    let corners = [
        Point {
            x: rect.min_x,
            y: rect.min_y,
        },
        Point {
            x: rect.max_x,
            y: rect.min_y,
        },
        Point {
            x: rect.max_x,
            y: rect.max_y,
        },
        Point {
            x: rect.min_x,
            y: rect.max_y,
        },
    ];

    // All 4 corners must be inside polygon
    for corner in corners {
        if !is_point_in_polygon(corner, edges) {
            return false;
        }
    }

    // The edges also have to be inside polygon
    // -- This is a bit of a stupid approach,
    // but I really can't get edge intersection code to work :< --
    for x in rect.min_x..rect.max_x {
        if !is_point_in_polygon(Point { x, y: rect.min_y }, edges) {
            return false;
        }
    }
    for x in rect.min_x..rect.max_x {
        if !is_point_in_polygon(Point { x, y: rect.max_y }, edges) {
            return false;
        }
    }
    for y in rect.min_y..rect.max_y {
        if !is_point_in_polygon(Point { x: rect.min_x, y }, edges) {
            return false;
        }
    }
    for y in rect.min_y..rect.max_y {
        if !is_point_in_polygon(Point { x: rect.max_x, y }, edges) {
            return false;
        }
    }

    return true;
}

fn is_point_in_polygon(point: Point, edges: &Vec<(&Point, &Point)>) -> bool {
    // Point-in-Polygon even-odd approach
    let mut is_inside = false;
    for (pi, pj) in edges {
        if is_on_edge(&point, pi, pj) {
            return true;
        }

        // Check if the point's Y coordinate is between the Y coordinates of the edge's endpoints
        // and if the point is to the left of the line segment connecting the endpoints.
        if ((pi.y > point.y) != (pj.y > point.y))
            && (point.x < (pj.x - pi.x) * (point.y - pi.y) / (pj.y - pi.y) + pi.x)
        {
            is_inside = !is_inside;
        }
    }

    is_inside
}

fn is_on_edge(p: &Point, a: &Point, b: &Point) -> bool {
    let within_bounds = p.x >= min(a.x, b.x)
        && p.x <= max(a.x, b.x)
        && p.y >= min(a.y, b.y)
        && p.y <= max(a.y, b.y);

    if !within_bounds {
        return false;
    }

    // 2D Cross product to check collinearity: (y2-y1)(x-x1) == (y-y1)(x2-x1)
    return (b.y - a.y) * (p.x - a.x) == (p.y - a.y) * (b.x - a.x);
}
