use std::cmp;
use std::fs::File;
use std::io::{BufRead, BufReader, Error};

#[derive(Debug, Clone, PartialEq)]
struct Point(i32, i32);

impl Point {
    fn transform(&self, v: &Vector) -> Point {
        match v {
            Vector::Up(delta_y) => Point(self.0, self.1 + delta_y),
            Vector::Right(delta_x) => Point(self.0 + delta_x, self.1),
        }
    }

    fn distance_from_origin(&self) -> i32 {
        self.0.abs() + self.1.abs()
    }

    fn distance_from_point(&self, p: &Point) -> i32 {
        (self.0 - p.0).abs() + (self.1 - p.1).abs()
    }
}

#[derive(Debug, PartialEq)]
enum Direction {
    Horizontal,
    Vertical,
}

#[derive(Debug, PartialEq)]
struct LineSegment(Point, Point);

impl LineSegment {
    fn direction(&self) -> Direction {
        if self.0 .0 == self.1 .0 {
            Direction::Vertical
        } else if self.0 .1 == self.1 .1 {
            Direction::Horizontal
        } else {
            panic!("Cannot handle diagonal line")
        }
    }

    fn intersection(&self, other: &LineSegment) -> Option<Point> {
        if self.direction() == other.direction() {
            None
        } else if self.direction() == Direction::Horizontal
            && other.0 .0 <= cmp::max(self.0 .0, self.1 .0)
            && other.0 .0 >= cmp::min(self.0 .0, self.1 .0)
            && self.0 .1 <= cmp::max(other.0 .1, other.1 .1)
            && self.0 .1 >= cmp::min(other.0 .1, other.1 .1)
        {
            Some(Point(other.0 .0, self.0 .1))
        } else if self.direction() == Direction::Vertical
            && self.0 .0 <= cmp::max(other.0 .0, other.1 .0)
            && self.0 .0 >= cmp::min(other.0 .0, other.1 .0)
            && other.0 .1 <= cmp::max(self.0 .1, self.1 .1)
            && other.0 .1 >= cmp::min(self.0 .1, self.1 .1)
        {
            Some(Point(self.0 .0, other.0 .1))
        } else {
            None
        }
    }

    fn length(&self) -> i32 {
        self.0.distance_from_point(&self.1)
    }

    /// returns true if point is on the line segment, false otherwise.
    fn includes(&self, p: &Point) -> bool {
        match self.direction() {
            Direction::Vertical => {
                p.0 == self.0 .0
                    && p.1 <= cmp::max(self.0 .1, self.1 .1)
                    && p.1 >= cmp::min(self.0 .1, self.1 .1)
            }
            Direction::Horizontal => {
                p.1 == self.0 .1
                    && p.0 <= cmp::max(self.0 .0, self.1 .0)
                    && p.0 >= cmp::min(self.0 .0, self.1 .0)
            }
        }
    }
}

enum Vector {
    Right(i32),
    Up(i32),
}

impl Vector {
    fn from_string(s: &str) -> Vector {
        let (dir, num) = s.split_at(1);
        let magnitude = num.parse().unwrap();
        match dir {
            "R" => Vector::Right(magnitude),
            "U" => Vector::Up(magnitude),
            "L" => Vector::Right(-magnitude),
            "D" => Vector::Up(-magnitude),
            _ => panic!("Cannot handle direction {}", dir),
        }
    }
}

/// Turn a string path into a vector of line segments
fn parse_directions(path: &str) -> Vec<LineSegment> {
    let mut result = Vec::new();
    path.split(',')
        .map(|s| Vector::from_string(s))
        .fold(Point(0, 0), |last_point, v| {
            let new_point = last_point.transform(&v);
            let new_segment = LineSegment(last_point, new_point.clone());
            result.push(new_segment);
            new_point
        });

    result
}

fn parse_file(path_to_input: &str) -> Result<(Vec<LineSegment>, Vec<LineSegment>), Error> {
    let file = File::open(path_to_input)?;
    let reader = BufReader::new(file);
    let mut lineses = reader
        .lines()
        .take(2)
        .map(|line| parse_directions(&line.unwrap()));
    let lines1 = lineses.next().unwrap();
    let lines2 = lineses.next().unwrap();
    Ok((lines1, lines2))
}

fn steps_to_point(lines: &Vec<LineSegment>, point: &Point) -> i32 {
    let mut steps = 0;
    for line in lines {
        if line.includes(point) {
            return steps + line.0.distance_from_point(point);
        } else {
            steps += line.length();
        }
    }
    steps
}

fn closest_intersection_steps(lines1: &Vec<LineSegment>, lines2: &Vec<LineSegment>) -> Option<i32> {
    let mut closest_total_steps = i32::max_value();
    for line1 in lines1 {
        for line2 in lines2 {
            if let Some(intersection) = line1.intersection(&line2) {
                if intersection != Point(0, 0) {
                    let total_steps = steps_to_point(lines1, &intersection)
                        + steps_to_point(lines2, &intersection);
                    if total_steps < closest_total_steps {
                        closest_total_steps = total_steps;
                    }
                }
            }
        }
    }

    if closest_total_steps == i32::max_value() {
        None
    } else {
        Some(closest_total_steps)
    }
}

fn pt2(path_to_input: &str) -> Result<Option<i32>, Error> {
    let (lines1, lines2) = parse_file(path_to_input)?;
    Ok(closest_intersection_steps(&lines1, &lines2))
}

fn closest_intersection_distance(
    lines1: &Vec<LineSegment>,
    lines2: &Vec<LineSegment>,
) -> Option<i32> {
    let mut closest_distance = i32::max_value();
    for line1 in lines1 {
        for line2 in lines2 {
            if let Some(intersection) = line1.intersection(&line2) {
                let distance = intersection.distance_from_origin();
                if distance < closest_distance && distance != 0 {
                    closest_distance = distance;
                }
            }
        }
    }
    if closest_distance == i32::max_value() {
        None
    } else {
        Some(closest_distance)
    }
}

fn pt1(path_to_input: &str) -> Result<Option<i32>, Error> {
    let (lines1, lines2) = parse_file(path_to_input)?;
    Ok(closest_intersection_distance(&lines1, &lines2))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_directions_test() {
        let path = "R75,D30,R83";
        assert_eq!(
            parse_directions(path),
            vec![
                LineSegment(Point(0, 0), Point(75, 0)),
                LineSegment(Point(75, 0), Point(75, -30)),
                LineSegment(Point(75, -30), Point(158, -30))
            ]
        );
    }

    #[test]
    fn direction_test() {
        let horizontal_line = LineSegment(Point(-1, 1), Point(5, 1));
        assert_eq!(Direction::Horizontal, horizontal_line.direction());

        let vertical_line = LineSegment(Point(-1, 1), Point(-1, -7));
        assert_eq!(Direction::Vertical, vertical_line.direction());
    }

    #[test]
    #[should_panic]
    fn diagonal_direction_test() {
        let diagonal_line = LineSegment(Point(-1, 1), Point(5, -7));
        diagonal_line.direction();
    }

    #[test]
    fn intersection_test() {
        let horizontal_line = LineSegment(Point(-1, 1), Point(5, 1));
        let another_horizontal_line = LineSegment(Point(-1, 2), Point(5, 2));
        let vertical_line = LineSegment(Point(-1, 1), Point(-1, -7));
        assert_eq!(
            Some(Point(-1, 1)),
            horizontal_line.intersection(&vertical_line)
        );
        assert_eq!(
            Some(Point(-1, 1)),
            vertical_line.intersection(&horizontal_line)
        );
        assert_eq!(None, horizontal_line.intersection(&another_horizontal_line));
    }

    #[test]
    fn closest_intersection_test() {
        let lines1 = parse_directions("R75,D30,R83,U83,L12,D49,R71,U7,L72");
        let lines2 = parse_directions("U62,R66,U55,R34,D71,R55,D58,R83");
        assert_eq!(closest_intersection_distance(&lines1, &lines2), Some(159));
    }

    #[test]
    fn closest_intersection_test2() {
        let lines1 = parse_directions("R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51");
        let lines2 = parse_directions("U98,R91,D20,R16,D67,R40,U7,R15,U6,R7");
        assert_eq!(closest_intersection_distance(&lines1, &lines2), Some(135));
    }

    #[test]
    fn pt1_test() {
        assert_eq!(pt1("input").unwrap().unwrap(), 1519);
    }

    #[test]
    fn closest_intersection_steps_test() {
        let lines1 = parse_directions("R75,D30,R83,U83,L12,D49,R71,U7,L72");
        let lines2 = parse_directions("U62,R66,U55,R34,D71,R55,D58,R83");
        assert_eq!(closest_intersection_steps(&lines1, &lines2), Some(610));
    }

    #[test]
    fn closest_intersection_steps_test2() {
        let lines1 = parse_directions("R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51");
        let lines2 = parse_directions("U98,R91,D20,R16,D67,R40,U7,R15,U6,R7");
        assert_eq!(closest_intersection_steps(&lines1, &lines2), Some(410));
    }

    #[test]
    fn pt2_test() {
        assert_eq!(pt2("input").unwrap().unwrap(), 14358);
    }
}
