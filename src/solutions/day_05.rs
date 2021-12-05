use std::collections::HashMap;

#[derive(Debug, Eq, PartialEq, Hash)]
struct Point {
    x: isize,
    y: isize,
}
impl Point {
    fn new(point: &str) -> Point {
        let values: Vec<isize> = point
            .split(",")
            .map(|value| value.parse().unwrap()).collect();
        Point {
            x: values[0],
            y: values[1],
        }
    }
}

#[derive(Debug, Eq, PartialEq, Hash)]
struct Line {
    start: Point,
    end: Point,
}
impl Line {
    fn new(line: &str) -> Line {
        let points: Vec<&str> = line.split(" -> ").collect();
        Line {
            start: Point::new(points[0]),
            end: Point::new(points[1]),
        }
    }

    fn is_diagonal(&self) -> bool {
        self.start.x != self.end.x && self.start.y != self.end.y
    }

    fn all_points(&self) -> Vec<Point> {
        let mut points = vec!();

        let (mut x, mut y) = (self.start.x, self.start.y);
        let step_x = if self.start.x < self.end.x { 1 } else { -1 };
        let step_y = if self.start.y < self.end.y { 1 } else { -1 };

        while x != self.end.x || y != self.end.y {
            points.push(Point { x, y });
            if x != self.end.x { x += step_x };
            if y != self.end.y { y += step_y };
        }
        points.push(Point { x, y });
        points
    }
}

fn process(input: &str) -> Vec<Line> {
    input
        .lines()
        .map(|line| Line::new(line))
        .collect()
}

pub fn solve_part_one(input: &str) -> usize {
    let lines = process(input);
    let mut covers = HashMap::new();

    lines.into_iter()
        .filter(|line| !line.is_diagonal())
        .for_each(|line| {
            line
                .all_points().into_iter()
                .for_each(|point| {
                        let cover = covers.entry(point).or_insert(0);
                        *cover += 1;
                    });
        });

    covers.into_iter()
        .fold(0, |intersections, (_point, cover)|
            if cover >= 2 { intersections + 1 } else { intersections }
        )
}

pub fn solve(input: &str) -> usize {
    let lines = process(input);
    let mut covers = HashMap::new();

    lines.into_iter()
        .for_each(|line| {
            line
                .all_points().into_iter()
                .for_each(|point| {
                        let cover = covers.entry(point).or_insert(0);
                        *cover += 1;
                    });
        });

    covers.into_iter()
        .fold(0, |intersections, (_point, cover)|
            if cover >= 2 { intersections + 1 } else { intersections }
        )
}
