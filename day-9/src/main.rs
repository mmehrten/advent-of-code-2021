use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader, Error};

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

/// Finds all local minima in an input array of values, and returns the sum of their risk values, as well as the product of all basin sizes around the minima.
///
/// A local minima is any point in the array that is lower than its adjacent up, down, left, and right points.
///
/// A risk value is one plus the local minima value.
///
/// A basin is all points that lead into a local minima.
///
/// # Arguments
///
/// * `input_path - The input file path containing the array of values.
///
/// # Returns
///
/// The sum of the local minima's risk values
///
/// # Examples
///
/// ## Basic
///
/// The following array has 4 local minima, with a total risk value of 15:
/// ```
/// 2199943210 // width 10
/// 3987894921
/// 9856789892
/// 8767896789
/// 9899965678
/// ```
fn solution(input_path: &str) -> (i32, i32) {
    let reader = get_buf_reader(input_path);
    let mut lines = reader.lines();
    let mut inputs = Vec::new();

    // Method used to parse a single iteration of the input file
    let parse_line = |line: Option<Result<String, Error>>| {
        line.expect("Failed to parse line from file.")
            .expect("Failed to parse line from file.")
            .split("")
            .filter(|s| s != &"")
            .map(|s| {
                s.parse::<i32>()
                    .expect("Failed to parse integer from inputs.")
            })
            .collect::<Vec<i32>>()
    };

    // Parse just the first line to determine the overall width of the inputs
    inputs.extend(parse_line(lines.next()));
    let array_width = inputs.len();
    println!("Array width: {}", array_width);

    // Parse the remaining lines
    loop {
        let line = lines.next();
        if line.is_none() {
            break;
        }
        inputs.extend(parse_line(line));
    }

    struct Field {
        spaces: Vec<i32>,
        width: usize,
    }
    impl Field {
        /// Return the count of elements in the Field.
        fn len(&self) -> usize {
            self.spaces.len()
        }

        /// Return the value of the field at the given index.
        fn get(&self, idx: usize) -> i32 {
            self.spaces[idx]
        }

        /// Return the indexes of all points adjacent to the given point.
        fn neighbors(&self, idx: usize) -> Vec<usize> {
            let mut neighbors = Vec::new();
            // Check the value above us
            if idx >= self.width {
                neighbors.push(idx - self.width);
            }
            // Check the value to the left of us
            if idx % self.width != 0 {
                neighbors.push(idx - 1);
            }
            // Check the value to the right of us
            if idx % self.width != self.width - 1 {
                neighbors.push(idx + 1);
            }
            // Check the value below us
            if idx < self.spaces.len() - self.width {
                neighbors.push(idx + self.width);
            }
            neighbors
        }

        /// Return True if all neighbors of the index are greater than the index, False otherwise.
        fn is_minima(&self, idx: usize) -> bool {
            let this_val = self.spaces[idx];
            for neighbor in self.neighbors(idx) {
                if this_val >= self.spaces[neighbor] {
                    return false;
                }
            }
            true
        }

        /// Return all neighbors of the index that are greater than the given point, up to but not including the value 9.
        fn ascending_neighbors(&self, idx: usize) -> HashSet<usize> {
            let mut new_neighbors = HashSet::new();
            // Make this index a part of the neighbor set
            new_neighbors.insert(idx);

            // Check all adjacent points
            let this_val = self.get(idx);
            for neighbor in self.neighbors(idx) {
                let next_val = self.get(neighbor);
                if next_val > this_val && next_val != 9 {
                    // This is an ascending neighbor, so add it to the set and check its neighbors as well
                    new_neighbors.extend(self.ascending_neighbors(neighbor));
                }
            }
            new_neighbors
        }
    }

    let field = Field {
        spaces: inputs,
        width: array_width,
    };
    // Search every point in the array for local minima
    let mut risk_score = 0;
    let mut basin_sizes = Vec::new();
    for idx in 0..field.len() {
        if field.is_minima(idx) {
            risk_score += field.get(idx) + 1;
            let basin = field.ascending_neighbors(idx);
            basin_sizes.push(basin.len());
        }
    }
    basin_sizes.sort();
    basin_sizes.reverse();
    (
        risk_score,
        basin_sizes.iter().take(3).fold(1, |acc, &x| acc * x) as i32,
    )
}

/// Print the total risk value of an array.
///
/// Usage:
///
/// ```
/// $ aoc inputs/example.txt
/// Total risk value: 15
/// ```
fn main() {
    let args: Vec<String> = std::env::args().collect();
    let input_path = parse_file_path(&args);
    let sol = solution(input_path);
    println!("Total risk value: {:?}", sol);
}

#[cfg(test)]
mod test_solution {
    use crate::solution;

    #[test]
    fn example_correct() {
        assert_eq!(solution("inputs/example.txt"), (15, 1134));
    }

    #[test]
    fn question_correct() {
        assert_eq!(solution("inputs/challenge.txt"), (580, 856716));
    }
}
