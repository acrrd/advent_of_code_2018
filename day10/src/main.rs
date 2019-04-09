use std::collections::HashSet;
use std::io::{self, Read};

#[derive(Debug, Clone, Copy)]
struct Point {
    x: i32,
    y: i32,
    dx: i32,
    dy: i32,
}

impl Point {
    fn new(x: i32, y: i32, dx: i32, dy: i32) -> Point {
        Point { x, y, dx, dy }
    }

    fn translate(self: &mut Self, step: i32) {
        self.x += self.dx * step;
        self.y += self.dy * step;
    }
}

#[derive(Debug, Clone, Copy)]
struct BBox {
    min: Point,
    max: Point,
}

fn parse_point(line: &str) -> Point {
    let ns: Vec<_> = line
        .split("<")
        .skip(1)
        .flat_map(|s| {
            s.split(">").take(1).flat_map(|s| {
                s.split(",")
                    .map(|s| s.trim().parse::<i32>().expect("Number"))
            })
        })
        .collect();

    Point::new(ns[0], ns[1], ns[2], ns[3])
}

fn get_bbox(points: &Vec<Point>) -> BBox {
    let max = std::i32::MAX;
    let min = std::i32::MIN;
    let bb = BBox {
        min: Point::new(max, max, max, max),
        max: Point::new(min, min, min, min),
    };

    points.iter().fold(bb, |mut bb, point| {
        if point.x < bb.min.x {
            bb.min.x = point.x;
        };

        if point.y < bb.min.y {
            bb.min.y = point.y;
            bb.min.dy = point.dy;
        };

        if point.x > bb.max.x {
            bb.max.x = point.x;
        };

        if point.y > bb.max.y {
            bb.max.y = point.y;
            bb.max.dy = point.dy;
        };

        bb
    })
}

fn parse_points(input: &str) -> Vec<Point> {
    input.lines().map(parse_point).collect()
}

fn print_grid(bbox: &BBox, points: &Vec<Point>) {
    let points: HashSet<(i32, i32)> = points.iter().map(|p| (p.x, p.y)).collect();
    for y in bbox.min.y..=bbox.max.y {
        for x in bbox.min.x..=bbox.max.x {
            print!("{}", if points.contains(&(x, y)) { "#" } else { "." });
        }
        println!("");
    }
}

fn main() -> io::Result<()> {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input)?;
    let input = input;

    let mut points = parse_points(&input);
    let bbox = get_bbox(&points);

    // while experimenting I found out that this work ¯\_(ツ)_/¯
    let steps = (bbox.max.y - bbox.min.y) / (bbox.max.dy.abs() + bbox.min.dy.abs());

    points.iter_mut().for_each(|p| p.translate(steps));
    let bbox = get_bbox(&points);
    print_grid(&bbox, &points);
    println!("Seconds: {}", steps);

    Ok(())
}
