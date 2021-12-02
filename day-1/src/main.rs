use std::fs::File;
use std::io::{BufRead, BufReader};

/// Parse an input file path, counting the number of numeric increases in the file.
///
/// # Arguments
///
/// * `input_path` - the OS fully qualified path to the file containing the input data.
///
/// # Returns
///
/// The count of lines whose numeric value are greater than the preceding value.
///
/// # Examples
///
/// For example, suppose you had the following input file:
///
/// ```
/// 199
/// 200
/// 208
/// 210
/// 200
/// 207
/// 240
/// 269
/// 260
/// 263
/// ```
/// We count the number of times a line increases from the previous line. (There is no measurement before the first measurement.)
/// In this example, the changes are as follows:
///
/// ```
/// 199 (N/A - no previous measurement)
/// 200 (increased)
/// 208 (increased)
/// 210 (increased)
/// 200 (decreased)
/// 207 (increased)
/// 240 (increased)
/// 269 (increased)
/// 260 (decreased)
/// 263 (increased)
/// ```
///
/// In this example, there are 7 lines that are larger than the previous, so we return 7.
fn count_numeric_increases(input_path: &str) -> i32 {
    // Create a buffer to read the file line by line
    let contents =
        File::open(input_path).expect(format!("Error reading file: {}", input_path).as_str());
    let reader = BufReader::new(contents);

    // Read each line, skipping the first, and checking for increases
    let mut prev_number: Option<i32> = None;
    let mut count_increases = 0;
    for line in reader.lines() {
        let line = line.expect("Failed to parse line from file.");
        let number = line
            .parse::<i32>()
            .expect("Error parsing number from file.");

        if prev_number.is_some() && number > prev_number.unwrap() {
            count_increases += 1;
        }
        prev_number = Some(number);
    }
    count_increases
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
        panic!("Expected one file path to run against, got: {}", args.len());
    }
    let input_path = &args[1];
    input_path.as_str()
}

/// Count the number of lines in a file of numeric values whose value increases from the preceding line.
///
/// Usage:
///
/// ```
/// $ day-1 inputs/challenge.txt
/// Found 1446 increases
/// ```
fn main() {
    let args: Vec<String> = std::env::args().collect();
    let input_path = parse_file_path(&args);
    println!("Found {} increases", count_numeric_increases(input_path));
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
mod test_count_numeric_increases {
    use crate::count_numeric_increases;

    #[test]
    fn example_correct() {
        assert_eq!(count_numeric_increases("inputs/example.txt"), 7);
    }

    #[test]
    fn question_correct() {
        assert_eq!(count_numeric_increases("inputs/challenge.txt"), 1446);
    }

    #[test]
    #[should_panic]
    fn error_file_handled() {
        count_numeric_increases("inputs/noexist.txt");
    }
}
