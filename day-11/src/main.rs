use std::fs::File;
use std::io::{BufRead, BufReader};
use std::io::Error;
use std::collections::HashSet;

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

struct Field {
    spaces: Vec<usize>,
    width: usize,
}
static ACTIVATION_ENERGY: usize = 9;
impl Field {
    
    /// Return the count of elements in the Field.
    fn len(&self) -> usize {
        self.spaces.len()
    }

    /// Return the value of the field at the given index.
    fn get(&self, idx: usize) -> usize {
        self.spaces[idx]
    }

    /// Return the indexes of all points adjacent to the given point.
    fn neighbors(&self, idx: usize) -> Vec<usize> {
        let mut neighbors = Vec::new();
        let has_above = idx >= self.width;
        let has_left = idx % self.width != 0;
        let has_right = idx % self.width != self.width - 1;
        let has_below = idx < self.spaces.len() - self.width;

        // Check the value above us
        if has_above {
            neighbors.push(idx - self.width);
        }
        // Check the value to the left of us
        if has_left {
            neighbors.push(idx - 1);
        }
        // Check the value to the right of us
        if has_right {
            neighbors.push(idx + 1);
        }
        // Check the value below us
        if has_below {
            neighbors.push(idx + self.width);
        }
        // Check top left
        if has_above && has_left {
            neighbors.push(idx - 1 - self.width)
        }
        // Check top right
        if has_above && has_right {
            neighbors.push(idx + 1 - self.width)
        }
        // Check bottom left
        if has_below && has_left {
            neighbors.push(idx - 1 + self.width)
        }
        // Check bottom right
        if has_below && has_right {
            neighbors.push(idx + 1 + self.width)
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

    /// Parse a line of values into a vector for the field
    fn parse_line(line: Result<String, Error>) -> Vec<usize> {
        line.expect("Failed to parse line from file.")
            .split("")
            .filter(|s| s != &"")
            .map(|s| {
                s.parse::<usize>()
                    .expect("Failed to parse integer from inputs.")
            })
            .collect::<Vec<usize>>()
    }

    /// Parse a line of values into a vector for the field
    fn parse_line_into(&mut self, line: Result<String, Error>) {
        self.spaces.extend(Field::parse_line(line));
    }
    
    /// Increase the energy of all nodes by one.
    fn increase_total_energy(&mut self) {
        for idx in 0..self.len() {
            self.spaces[idx] += 1;
        }
    }
    
    /// Try to acticate the given node - if it activates, increase neighbors energy and try their activations as well.
    fn try_activate_node(&mut self, idx: usize, activations: &mut HashSet<usize>) {
        // If we've already triggered this node, or it's not ready to trigger, move one
        if self.spaces[idx] <= ACTIVATION_ENERGY || activations.contains(&idx) {
            return; 
        }

        // Activate this node, and all adjascent nodes
        activations.insert(idx);
        for neighbor in self.neighbors(idx) {
            // Since this node activated, the neighbor increases energy
            self.spaces[neighbor] += 1;
            // See if we can activate the neighbor now
            self.try_activate_node(neighbor, activations);
        }
    }

    /// Trigger activation of all available nodes in the field.
    fn try_activate_all(&mut self, activations: &mut HashSet<usize>) {
        for idx in 0..self.len() {
            self.try_activate_node(idx, activations);
        }
    }

    fn deactivate_node(&mut self, idx: usize) {
        self.spaces[idx] = 0;
    }
}

/// Predict the number of flashes in a population of dumbo octopuses after N iterations.
///
/// Each octopus flashes based on its energy level. The energy level of each octopus is a
/// value between 0 and 9.
/// 
/// The energy levels operate in steps, during a single step, the following occurs:
///  
///  * First, the energy level of each octopus increases by 1.
///  * Then, any octopus with an energy level greater than 9 flashes. 
///    * This increases the energy level of all adjacent octopuses by 1, including octopuses that are diagonally adjacent.
///    * If this causes an octopus to have an energy level greater than 9, it also flashes. 
///    * This process continues as long as new octopuses keep having their energy level increased beyond 9. (An octopus can only flash at most once per step.)
///  * Finally, any octopus that flashed during this step has its energy level set to 0, as it used all of its energy to flash.
///  
/// # Arguments
///
/// * `input_path - The input file path containing initial energy levels.
/// * `num_iterations - The number of iterations to process.
///
/// # Returns
///
/// The total number of flashes after N iterations.
fn solution(input_path: &str, num_iterations: usize) -> usize {
    let reader = get_buf_reader(input_path);
    let mut lines = reader.lines();
    let mut inputs = Vec::new();
    // Parse just the first line to determine the overall width of the inputs
    inputs.extend(Field::parse_line(lines.next().expect("")));
    let array_width = inputs.len();
    let mut field = Field {width: array_width, spaces: inputs};

    // Parse the remaining lines
    while let Some(line) = lines.next() {
        field.parse_line_into(line);
    }

    let mut activation_count = 0;
    for _ in 0..num_iterations {
        let mut activations = HashSet::new();
        field.increase_total_energy();
        field.try_activate_all(&mut activations);
        activation_count += activations.len();
        for idx in activations {
            field.deactivate_node(idx);
        }
    }
    activation_count
}

/// Print the total number of octopi activations after 100 steps, given an input of initial energy levels.
///
/// Usage:
///
/// ```
/// $ aoc inputs/example.txt
/// Total activation count after 100 steps: 1656
/// ```
fn main() {
    let args: Vec<String> = std::env::args().collect();
    let input_path = parse_file_path(&args);
    let sol = solution(input_path, 100);
    println!("Total activation count after 100 steps: {:?}", sol);
}

#[cfg(test)]
mod test_solution {
    use crate::solution;

    #[test]
    fn example_correct() {
        assert_eq!(solution("inputs/example.txt", 100), 1656);
    }

    #[test]
    fn question_correct() {
        assert_eq!(solution("inputs/challenge.txt", 100), 1613);
    }
}
