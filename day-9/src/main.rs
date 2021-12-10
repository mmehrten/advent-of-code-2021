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

/// Finds all local minima in an input array of values, and returns the sum of their risk values.
/// 
/// A local minima is any point in the array that is lower than its adjacent up, down, left, and right points.
/// 
/// A risk value is one plus the local minima value.
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
fn solution(input_path: &str) -> i32 {
    let reader = get_buf_reader(input_path);
    let mut lines = reader.lines();
    let mut inputs = Vec::new();

    // Method used to parse a single iteration of the input file
    let parse_line = |line: Option<Result<String, Error>>| {
        line.expect("Failed to parse line from file.")
        .expect("Failed to parse line from file.")
        .split("")
        .filter(|s| s != &"")
        .map(|s| s.parse::<i32>().expect("Failed to parse integer from inputs."))
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

    // Search every point in the array for local minima
    let mut risk_score = 0;
    let mut to_write = Vec::new();
    let mut counted = 0;
    for idx in 0..inputs.len() {
        counted += 1;
        let this_val = &inputs[idx];
        to_write.push(this_val.to_string());

        // Check the value above us
        if idx >= array_width {
            // the value above us is exactly array_width spaces to the left
            let top_val = &inputs[idx - array_width];
            if this_val >= top_val {
                continue;
            }
        }

        // Check the value to the left of us
        if idx % array_width != 0 {
            let right_val = &inputs[idx - 1];
            if this_val >= right_val {
                continue;
            }
        }
        // Check the value to the right of us
        if idx % array_width != array_width - 1 {
            let right_val = &inputs[idx + 1];
            if this_val >= right_val {
                continue;
            }
        }

        // Check the value below us
        if idx < inputs.len() - array_width  {
            // the value above us is exactly array_width spaces to the right
            let bottom_val = &inputs[idx + array_width];
            if this_val >= bottom_val {
                continue;
            }
        }
        
        // We're less than all those points, so this is a local minima
        risk_score += *this_val + 1;
        to_write.pop();
        to_write.push("\x1b[93m".to_string() + this_val.to_string().as_str() + "\x1b[0m");
    }
    for idx in 0..to_write.len() {
        print!("{}", to_write[idx]);
        if idx % array_width == array_width - 1 {
            print!("\n");
        }
    }
    println!("Array points checked: {}", counted);

    risk_score
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
        assert_eq!(solution("inputs/example.txt"), 15);
    }

    #[test]
    fn question_correct() {
        assert_eq!(solution("inputs/challenge.txt"), 580);
    }
}
