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

struct Fold {
    axis: String,
    at: usize,
}

struct DotMatrix {
    matrix: Vec<bool>,
    x_dim: usize,
    y_dim: usize,
    x_view_dim: usize,
    y_view_dim: usize,
}

/// Map a 2d matrix onto a 1d array
/// where each array segment of x elements represents one part of the y axis:
/// [(0,0), (1,0), (2,0), ..., (x_dim,0),
///  ...,
///  (0,y_dim), (1,y_dim), (2,y_dim), ..., (x_dim,y_dim)]
/// Therefore there are x "columns" representing the x axis, and
/// there are y "rows" representing the y axis.
impl DotMatrix {
    // Create a DotMatrix using a vector of x, y coordinates indicating the "points" that are turned on.
    fn from_points(points: Vec<(usize, usize)>) -> DotMatrix {
        let x_dim = points.iter().map(|t| t.0).max().unwrap() + 1;
        let y_dim = points.iter().map(|t| t.1).max().unwrap() + 1;
        let mut matrix = vec![false; x_dim * y_dim];
        for (x, y) in points {
            matrix[x + (y * x_dim)] = true;
        }
        DotMatrix {
            matrix: matrix,
            x_dim: x_dim,
            y_dim: y_dim,
            x_view_dim: x_dim,
            y_view_dim: y_dim,
        }
    }

    /// Print a representation of the DotMatrix.
    fn print(&self) {
        for y in 0..self.y_view_dim {
            for x in 0..self.x_view_dim {
                print!(
                    "{}",
                    if self.matrix[x + (y * self.x_dim)] {
                        "#"
                    } else {
                        "."
                    }
                );
            }
            print!("\n");
        }
    }

    fn _fold_x(&mut self, at: usize) {
        // flip everything at x > fold.at over to the left
        for y_row_offset in 0..self.y_view_dim {
            let y_row = self.x_dim * y_row_offset;
            for x_column in at..self.x_view_dim {
                // point will remain in dim column, but will be mapped to x_dim - x (left)
                let new_x_column = self.x_view_dim - x_column - 1;
                self.matrix[new_x_column + y_row] |= self.matrix[x_column + y_row];

                // Unset the original point since that's been moved over
                self.matrix[x_column + y_row] = false;
            }
        }
    }

    fn _fold_y(&mut self, at: usize) {
        // flip everything at y > fold.at up
        for x_column in 0..self.x_view_dim {
            for y_row_offset in at..self.y_view_dim {
                let y_row = self.x_dim * y_row_offset;
                let new_y_row = self.x_dim * (self.y_view_dim - y_row_offset - 1);
                // Swap y value into new row, maintaining column
                self.matrix[x_column + new_y_row] |= self.matrix[x_column + y_row];

                // Unset the original point since that's been moved over
                self.matrix[x_column + y_row] = false;
            }
        }
    }

    /// Fold the matrix along an axis at a given boundary.
    fn fold(&mut self, fold: &Fold) {
        if fold.axis == "x" {
            self._fold_x(fold.at);
            // Change the basis for future printing & folding
            self.x_view_dim = fold.at;
        } else {
            self._fold_y(fold.at);
            // Change the basis for future printing & folding
            self.y_view_dim = fold.at;
        }
    }

    /// Return the count of active points in the matrix.
    fn active_count(&self) -> usize {
        self.matrix.iter().filter(|p| **p).map(|_| 1).sum()
    }
}
/// Parse a set of points from an input, and follow a set of "fold" instructions to transform the points.
///
/// For example, the input:
///
/// ```
/// 6,10
/// 0,14
/// 9,10
/// 0,3
/// 10,4
/// 4,11
/// 6,0
/// 6,12
/// 4,1
/// 0,13
/// 10,12
/// 3,4
/// 3,0
/// 8,4
/// 1,10
/// 2,14
/// 8,10
/// 9,0
///
/// fold along y=7
/// fold along x=5
/// ```
///
/// Indicates mapping all points y > 7 down with a vertical reflection, followed by all points with
/// x > 5 left with a horizontal reflection.
///
/// # Arguments
///
/// * `input_path` - The input file path contianing the points and fold instructions.
/// * `num_folds` - The number of fold instructions to perform.
///
/// # Returns
///
/// The number of dots visible after N folds.
fn solution(input_path: &str, num_folds: usize) -> usize {
    let reader = get_buf_reader(input_path);

    let mut points = Vec::new();
    let mut folds = Vec::new();

    for line in reader.lines() {
        let line = line
            .expect("Failed to read line from file.")
            .trim()
            .replace("fold along ", "");
        let parts = line.split_once(",");
        if !parts.is_none() {
            let (left, right) = parts.unwrap();
            points.push((
                left.parse::<usize>().unwrap(),
                right.parse::<usize>().unwrap(),
            ));
        }
        let parts = line.split_once("=");
        if !parts.is_none() {
            let (left, right) = parts.unwrap();
            folds.push(Fold {
                axis: left.to_string(),
                at: right.parse::<usize>().unwrap(),
            });
        }
    }

    // Avoid passing num_folds more than specified in the input file
    let num_folds = if num_folds > folds.len() {
        folds.len()
    } else if num_folds == 0 {
        folds.len()
    } else {
        num_folds
    };

    let mut m = DotMatrix::from_points(points);
    for idx in 0..num_folds {
        let fold = &folds[idx];
        println!("Performing {}={} fold", fold.axis, fold.at);
        m.fold(fold);
    }

    if m.x_view_dim < 100 && m.y_view_dim < 100 {
        println!("Folded matrix:");
        m.print();
    }
    m.active_count()
}

/// Print the number of points visible after 1 fold.
///
/// Usage:
///
/// ```
/// $ aoc inputs/example.txt
/// Points after 1 fold: 17
/// ```
fn main() {
    let args: Vec<String> = std::env::args().collect();
    let input_path = parse_file_path(&args);
    let sol = solution(input_path, 1);
    println!("Points after 1 fold: {:?}", sol);
    let sol = solution(input_path, 0);
    println!("Points after all folds: {:?}", sol);
}

#[cfg(test)]
mod test_solution {
    use crate::solution;

    #[test]
    fn example_correct() {
        assert_eq!(solution("inputs/example.txt", 1), 17);
    }

    #[test]
    fn question_correct() {
        assert_eq!(solution("inputs/challenge.txt", 1), 720);
    }

    #[test]
    fn question_part2_correct() {
        assert_eq!(solution("inputs/challenge.txt", 0), 104);
    }
}
