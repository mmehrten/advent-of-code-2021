use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
/// Parse the file path from command line arguments.
///
/// # Arguments
///
/// * `args` - the command line arguments
///
/// # Returns
///
/// A single command line argument - panics if zero or more than one argument are passed.
fn parse_file_path(args: &[String]) -> &str {
    if args.len() != 2 {
        panic!(
            "Expected one file path and an optional window size to run against, got: {} arguments",
            args.len() - 1
        );
    }
    let input_path = &args[1];
    input_path.as_str()
}

#[cfg(test)]
mod test_parse_file_path {
    use crate::parse_file_path;

    #[test]
    fn one_arg_ok() {
        assert_eq!(
            parse_file_path(&vec!["script_path".to_string(), "arg_text".to_string()][..]),
            "arg_text"
        );
    }

    #[test]
    #[should_panic]
    fn no_arg_fail() {
        parse_file_path(&Vec::new());
    }

    #[test]
    #[should_panic]
    fn many_arg_fail() {
        parse_file_path(
            &vec![
                "script_path".to_string(),
                "arg_text".to_string(),
                "extra_arg".to_string(),
            ][..],
        );
    }
}

/// Open an input path and return a buffered reader over the contents.
fn get_buf_reader(input_path: &str) -> BufReader<File> {
    // Create a buffer to read the file line by line
    let contents =
        File::open(input_path).expect(format!("Error reading file: {}", input_path).as_str());
    let reader = BufReader::new(contents);
    reader
}

#[cfg(test)]
mod test_get_buf_reader {
    use crate::get_buf_reader;

    #[test]
    #[should_panic]
    fn error_file_handled() {
        get_buf_reader("inputs/noexist.txt");
    }

    #[test]
    fn example_file_handled() {
        get_buf_reader("inputs/example.txt");
    }
}

#[derive(PartialEq, Eq, Hash)]
struct Point {
    x: usize,
    y: usize,
}
#[derive(PartialEq, Eq, Hash)]
struct Ray {
    start: Point,
    end: Point,
}
#[derive(PartialEq, Eq, Hash)]
enum Direction {
    Horizontal,
    Vertical,
    Diagonal
}
impl Ray {
    fn direction(&self) -> Direction {
        if self.start.x == self.end.x {
            return Direction::Vertical;
        }
        if self.start.y == self.end.y {
            return Direction::Horizontal;
        }
        Direction::Diagonal
    }

    fn intersection(&self, other: &Ray) -> Option<Vec<Point>> {
        match self.direction() {
            Direction::Vertical => {
                // We're vertical, so check if we contain them or cross them
                match other.direction() {
                    Direction::Vertical => {
                        // Check for contains
                        if self.start.x != other.start.x {
                            return None;
                        }
                        let largest_start = if self.start.y >= other.start.y {
                            self.start.y
                        } else {
                            other.start.y
                        };
                        let smallest_end = if self.end.y <= other.end.y {
                            self.end.y
                        } else {
                            other.end.y
                        };
                        if smallest_end < largest_start {
                            return None;
                        }

                        return Some(
                            (largest_start..smallest_end)
                                .map(|y| Point {
                                    x: self.start.x,
                                    y: y,
                                })
                                .collect(),
                        );
                    }
                    Direction::Horizontal => {
                        if !((self.start.y <= other.start.y && other.start.y <= self.end.y) // They lie in our vertical bounds
                            && (other.start.x <= self.start.x && self.start.x <= other.end.x))
                        {
                            return None;
                        }
                        return Some(vec![Point {
                            x: self.start.x,
                            y: other.start.y,
                        }]);
                    }
                    _ => panic!("Diagonal not supported")
                }
            }
            Direction::Horizontal => {
                // We're horizontal, so check if we contain them or cross them
                match other.direction() {
                    Direction::Vertical => {
                        // Check for cross
                        if (self.start.x <= other.start.x && other.start.x <= self.end.x) // They lie in our horizontal bounds
                        && (other.start.y <= self.start.y && self.start.y <= other.end.y)
                        // AND we lie  in their vertical bounds
                        {
                            return None;
                        }
                        return Some(vec![Point {
                            x: other.start.x,
                            y: self.start.y,
                        }]);
                    }
                    Direction::Horizontal => {
                        // Check for contains
                        if !(self.start.y == other.start.y) {
                            return None;
                        }
                        let largest_start = if self.start.x >= other.start.x {
                            self.start.x
                        } else {
                            other.start.x
                        };
                        let smallest_end = if self.end.x <= other.end.x {
                            self.end.x
                        } else {
                            other.end.x
                        };
                        if smallest_end < largest_start {
                            return None;
                        }

                        return Some(
                            (largest_start..smallest_end)
                                .map(|x| Point {
                                    x: x,
                                    y: self.start.y,
                                })
                                .collect(),
                        );
                    }
                    _ => panic!("Diagonal not supported")
                };
            }
            _ => panic!("Diagonal not supported")
        }
    }

    fn path(&self) -> Vec<Point> {
        match self.direction() {
            Direction::Vertical => (self.start.y..self.end.y + 1)
                .map(|y| Point {
                    x: self.start.x,
                    y: y,
                })
                .collect(),
            Direction::Horizontal => (self.start.x..self.end.x + 1)
                .map(|x| Point {
                    x: x,
                    y: self.start.y,
                })
                .collect(),
            Direction::Diagonal => {
                let mut points = Vec::new();
                let mut y = self.start.y as i32;
                let off = if self.start.y <= self.end.y {1} else {-1};
                for x in self.start.x..self.end.x + 1 {
                    points.push(Point {x: x, y: y as usize});
                    y = y + off;
                }
                points
            }
        }
    }

    fn contains(&self, other: Point) -> bool {
        match self.direction() {
            Direction::Vertical => self.start.x == other.x && self.start.y <= other.y && other.y <= self.end.y,
            Direction::Horizontal => self.start.y == other.y && self.start.x <= other.x && other.x <= self.end.x,
            _ => panic!("Diagonal not supported")
        }
    }
}


/// TODO
///
/// # Arguments
///
/// * `input_path - The input file path TODO
///
/// # Returns
///
/// TODO
///
/// # Examples
///
/// ## Basic
///
/// You come across a field of hydrothermal vents on the ocean floor!
/// These vents constantly produce large, opaque clouds, so it would be best to avoid them if possible.
///
/// They tend to form in lines; the submarine helpfully produces a list of nearby lines of vents (your puzzle input)
/// for you to review. For example:
///
/// ```
/// 0,9 -> 5,9
/// 8,0 -> 0,8
/// 9,4 -> 3,4
/// 2,2 -> 2,1
/// 7,0 -> 7,4
/// 6,4 -> 2,0
/// 0,9 -> 2,9
/// 3,4 -> 1,4
/// 0,0 -> 8,8
/// 5,5 -> 8,2
/// ```
///
/// Each line of vents is given as a line segment in the format x1,y1 -> x2,y2 where x1,y1 are the coordinates
/// of one end the line segment and x2,y2 are the coordinates of the other end. These line segments include the
/// Bapoints at both ends. In other words:
///
/// * An entry like 1,1 -> 1,3 covers points 1,1, 1,2, and 1,3.
/// * An entry like 9,7 -> 7,7 covers points 9,7, 8,7, and 7,7.
///
/// For now, only consider horizontal and vertical lines: lines where either x1 = x2 or y1 = y2.
///
/// So, the horizontal and vertical lines from the above list would produce the following diagram:
///
/// ```
/// .......1..
/// ..1....1..
/// ..1....1..
/// .......1..
/// .112111211
/// ..........
/// ..........
/// ..........
/// ..........
/// 222111....
/// ```
///
/// In this diagram, the top left corner is 0,0 and the bottom right corner is 9,9.
/// Each position is shown as the number of lines which cover that point or . if no line covers that point.
/// The top-left pair of 1s, for example, comes from 2,2 -> 2,1; the very bottom row is formed by the overlapping
/// lines 0,9 -> 5,9 and 0,9 -> 2,9.
///
/// To avoid the most dangerous areas, you need to determine the number of points where at least two lines overlap.
/// In the above example, this is anywhere in the diagram with a 2 or larger - a total of 5 points.
///
/// Consider only horizontal and vertical lines. At how many points do at least two lines overlap?
fn solution(input_path: &str, ignore_diagonal: bool) -> usize {
    let reader = get_buf_reader(input_path);
    let lines = reader.lines();

    let input_stream: Vec<usize> = lines
        .map(|line| line.unwrap())
        .map(|line| {
            line.split(" -> ")
                .map(|x| x.to_string())
                .collect::<Vec<String>>()
        })
        .flatten()
        .map(|x: String| x.split(',').map(|x| x.to_string()).collect::<Vec<String>>())
        .flatten()
        .filter(|x| x.trim() != "")
        .map(|x| x.parse::<usize>().expect("Failed to parse input as usize."))
        .collect();

    let mut rays: Vec<Ray> = input_stream
        .iter()
        .as_slice()
        .chunks(4)
        .map(|s| {
            let mut start = (s[0], s[1]);
            let mut end = (s[2], s[3]);
            if start > end {
                start = (s[2], s[3]);
                end = (s[0], s[1]);
            }
            Ray {
                start: Point {
                    x: start.0,
                    y: start.1,
                },
                end: Point { x: end.0, y: end.1 },
            }
        })
        .filter(|ray| if ignore_diagonal {ray.direction() != Direction::Diagonal } else {true})
        .collect();

    let mut overlaps: HashMap<Point, i32> = HashMap::new();
    for ray in rays {
        for point in ray.path() {
            let mut val = 0;
            if overlaps.contains_key(&point) {
                val = *overlaps.get(&point).unwrap();
            }
            val += 1;
            overlaps.insert(point, val);
        }
    }
    overlaps.values().filter(|x| **x >= 2).count()
}

/// Read an input of rays (two points in space) and output the number of integer points where horizontal or vertical rays overlap at least twice, as well as including diagonal lines.
///
/// Usage:
///
/// ```
/// $ aoc inputs/example.txt
/// Lines overlapping at least twice without diagonals: 5
/// Lines overlapping at least twice with diagonals: 12
/// ```
fn main() {
    let args: Vec<String> = std::env::args().collect();
    let input_path = parse_file_path(&args);
    let sol = solution(input_path, true);
    println!("Lines overlapping at least twice without diagonals: {:?}", sol);
    let sol = solution(input_path, false);
    println!("Lines overlapping at least twice with diagonals: {:?}", sol);
}

#[cfg(test)]
mod test_solution {
    use crate::solution;

    #[test]
    fn example_correct() {
        assert_eq!(solution("inputs/example.txt", true), 5);
        assert_eq!(solution("inputs/example.txt", false), 12);
    }

    #[test]
    fn question_correct() {
        assert_eq!(solution("inputs/challenge.txt", true), 8111);
        assert_eq!(solution("inputs/challenge.txt", false), 22088);
    }
}
