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

/// Parse the gamma and epsilon power factors from a binary power report.
///
/// # Arguments
///
/// * `input_path - The input file path containing the power report.
///
/// # Returns
///
/// The (gamma rate, epsilon rate) of the power report
///
/// # Examples
///
/// ## Basic
///
/// For example, the power report:
///
/// ```
/// 00100
/// 11110
/// 10110
/// 10111
/// 10101
/// 01111
/// 00111
/// 11100
/// 10000
/// 11001
/// 00010
/// 01010
/// ```
///
/// Considering only the first bit of each number, there are five 0 bits and seven 1 bits. Since the most common bit is 1, the first bit of the gamma rate is 1.
/// 
/// The most common second bit of the numbers in the diagnostic report is 0, so the second bit of the gamma rate is 0.
/// 
/// The most common value of the third, fourth, and fifth bits are 1, 1, and 0, respectively, and so the final three bits of the gamma rate are 110.
/// 
/// So, the gamma rate is the binary number 10110, or 22 in decimal.
/// 
/// The epsilon rate is calculated in a similar way; rather than use the most common bit, the least common bit from each position is used. So, the epsilon rate is 01001, or 9 in decimal. 
/// 
/// Therefore, the we Would produce a final power factors of (22, 9).
///
fn read_power_report(input_path: &str) -> (i32, i32) {
    let reader = get_buf_reader(input_path);
    let (mut gamma, mut eps) = (0, 0);
    for line in reader.lines() {
        let line = line.expect("Failed to parse line from file.");
    }
    (gamma, eps)
}

/// Record the gamma / epsilon rate of the power report.
///
/// Usage:
///
/// ```
/// $ day-3 inputs/example.txt
/// Power rates: (22, 9), multiplied: 198
/// ```
fn main() {
    let args: Vec<String> = std::env::args().collect();
    let input_path = parse_file_path(&args);
    let (x, y) = read_power_report(input_path);
    println!(
        "Power rates: ({}, {}), multiplied: {}",
        x,
        y,
        x * y
    );
}



#[cfg(test)]
mod test_read_power_report {
    use crate::read_power_report;

    #[test]
    fn example_correct() {
        assert_eq!(read_power_report("inputs/example.txt"), (22, 9));
    }

    #[test]
    fn question_correct() {
        assert_eq!(read_power_report("inputs/challenge.txt"), (1845, 916));
    }
}
