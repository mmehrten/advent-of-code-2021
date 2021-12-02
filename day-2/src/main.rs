use std::collections::VecDeque;
use std::fs::File;
use std::io::{BufRead, BufReader};

/// Record movements of forward, up, and down to retrieve the final (horizontal, depth) coordinates of the movements.
///
/// # Arguments
///
/// * `input_path - The input file path containing the movements
///
/// # Returns
///
/// The (horizontal, depth) coordinates of the final position.
///
/// # Examples
///
/// ## Basic
///
/// For example, the movements:
///
/// ```
/// forward 5
/// down 5
/// forward 8
/// up 3
/// down 8
/// forward 2
/// ```
///
/// Would produce a final position of (15, 10).
///
fn record_movements(input_path: &str) -> (i32, i32) {
    // Create a buffer to read the file line by line
    let contents =
        File::open(input_path).expect(format!("Error reading file: {}", input_path).as_str());
    let reader = BufReader::new(contents);

    for line in reader.lines() {
        let line = line.expect("Failed to parse line from file.");
    }
    (0, 0)
}

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

/// Record the final horizontal / depth position in a file of movements.
///
/// Usage:
///
/// ```
/// $ day-2 inputs/example.txt
/// Final coordinates: (15, 10), multiplied: 150
/// ```
fn main() {
    let args: Vec<String> = std::env::args().collect();
    let input_path = parse_file_path(&args);
    let (x, y) = record_movements(input_path);
    println!("Final coordinates: ({}, {}), multiplied: {}", x, y, x * y);
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

#[cfg(test)]
mod test_record_movements {
    use crate::record_movements;

    #[test]
    fn example_correct() {
        assert_eq!(record_movements("inputs/example.txt"), (15, 10));
    }

    #[test]
    fn question_correct_small_window() {
        assert_eq!(record_movements("inputs/challenge.txt"), (-1, -1));
    }

    #[test]
    #[should_panic]
    fn error_file_handled() {
        record_movements("inputs/noexist.txt");
    }
}
