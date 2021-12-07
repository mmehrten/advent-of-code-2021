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

/// Determine the closest common value between a set of numbers, and the overall difference between the values and the common value.
///
/// # Arguments
///
/// * `input_path - The input file path containing integers to align.
///
/// # Returns
///
/// The closest common value, and the total distance of the points from the common value.
///
/// # Examples
///
/// ## Basic
///
/// For examples, given the numbers 16,1,2,0,4,2,7,1,2,14, the closest common value between them is 2,
/// with a total overall difference of 37 (16 - 2 + ... + 14 - 2).
fn solution(input_path: &str) -> (i32, i32) {
    let reader = get_buf_reader(input_path);
    let lines = reader.lines();
    for line in lines {
        let line = line.expect("Failed to parse line from file.");
    }
    (0, 0)
}

/// Output the number that is closest to a given set of numbers
///
/// Usage:
///
/// ```
/// $ aoc inputs/example.txt
/// Closest number: 2, total distance: 37
/// ```
fn main() {
    let args: Vec<String> = std::env::args().collect();
    let input_path = parse_file_path(&args);
    let (num, dist) = solution(input_path);
    println!("Closest number: {}, total distance: {}", num, dist);
}

#[cfg(test)]
mod test_solution {
    use crate::solution;

    #[test]
    fn example_correct() {
        assert_eq!(solution("inputs/example.txt"), (2, 37));
    }

    #[test]
    fn question_correct() {
        assert_eq!(solution("inputs/challenge.txt"), (0, 0));
    }
}
