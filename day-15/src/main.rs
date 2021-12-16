use std::collections::VecDeque;
use std::fs::File;
use std::io::{BufRead, BufReader, Error};
use std::collections::{HashMap, HashSet, BinaryHeap};
use std::cmp::{Ordering, Ord};

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


#[derive(Debug)]
struct Visit<V> {
    vertex: V,
    distance: usize,
}

impl<V> Ord for Visit<V> {
    fn cmp(&self, other: &Self) -> Ordering {
        other.distance.cmp(&self.distance)
    }
}

impl<V> PartialOrd for Visit<V> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<V> PartialEq for Visit<V> {
    fn eq(&self, other: &Self) -> bool {
        self.distance.eq(&other.distance)
    }
}

impl<V> Eq for Visit<V> {}


struct Field {
    spaces: Vec<usize>,
    width: usize,
}
impl Field {
    /// Method used to parse a single iteration of the input file
    fn _parse_line(line: Result<String, Error>) -> Vec<usize> {
        line.expect("Failed to parse line from file.")
            .split("")
            .filter(|s| s != &"")
            .map(|s| {
                s.parse::<usize>()
                    .expect("Failed to parse integer from inputs.")
            })
            .collect::<Vec<usize>>()
    }

    /// Parse a Field from a BufReader of numbers.
    fn from_reader(reader: BufReader<File>) -> Field {
        let mut lines = reader.lines();
        let mut inputs = Vec::new();
        // Parse just the first line to determine the overall width of the inputs
        inputs.extend(Field::_parse_line(lines.next().unwrap()));
        let array_width = inputs.len();

        // Parse the remaining lines
        while let Some(line) = lines.next() {
            inputs.extend(Field::_parse_line(line));
        }
        Field {
            spaces: inputs,
            width: array_width,
        }
    }
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

    /// Count the number of valid traversals from the starting node to the ending node.
    ///
    /// Uses DFS to traverse all paths in the graph.
    fn get_min_cost_dijkstra(&self) -> usize  {
        let mut distances = HashMap::new();
        let mut visited = HashSet::new();
        let mut to_visit = BinaryHeap::new();
    
        distances.insert(0, 0);
        to_visit.push(Visit {
            vertex: 0,
            distance: 0,
        });
    
        while let Some(Visit { vertex, distance }) = to_visit.pop() {
            if !visited.insert(vertex) {
                // Already visited this node
                continue;
            }
    
            for neighbor in self.neighbors(vertex)  {
                let cost = self.get(neighbor);
                let new_distance = distance + cost;
                let is_shorter = distances
                    .get(&neighbor)
                    .map_or(true, |&current| new_distance < current);

                if is_shorter {
                    distances.insert(neighbor, new_distance);
                    to_visit.push(Visit {
                        vertex: neighbor,
                        distance: new_distance,
                    });
                }
            }
        }
        
        let ending_node_idx = self.len() - 1;
        *distances.get(&ending_node_idx).unwrap()
    }
}

/// Calculate the lowest cost path between the top left and bottom right corners of a grid.
///
/// Example grid:
///
/// ```
/// 1163751742
/// 1381373672
/// 2136511328
/// 3694931569
/// 7463417111
/// 1319128137
/// 1359912421
/// 3125421639
/// 1293138521
/// 2311944581
/// ```
///
/// # Arguments
///
/// * `input_path` - The input file path containing the grid to traverse.
///
/// # Returns
///
/// The cost of the lowest cost path.
fn solution(input_path: &str) -> usize {
    let reader = get_buf_reader(input_path);
    let f = Field::from_reader(reader);
    f.get_min_cost_dijkstra()
}

/// Print the cost of the lowest cost path of a grid traversal.
///
/// Usage:
///
/// ```
/// $ aoc inputs/example.txt
/// Cost of lowest cost path: 40
/// ```
fn main() {
    let args: Vec<String> = std::env::args().collect();
    let input_path = parse_file_path(&args);
    let sol = solution(input_path);
    println!("Cost of lowest cost path: {:?}", sol);
}

#[cfg(test)]
mod test_solution {
    use crate::solution;

    #[test]
    fn example_correct() {
        assert_eq!(solution("inputs/example.txt"), 40);
    }

    #[test]
    fn question_correct() {
        assert_eq!(solution("inputs/challenge.txt"), 0);
    }
}
