use std::collections::VecDeque;
use std::fs::File;
use std::io::{BufRead, BufReader};

/// Parse an input file path, counting the number of numeric increases in the file.
///
/// # Arguments
///
/// * `input_path` - the OS fully qualified path to the file containing the input data.
/// * `window_size` - the number of lines to include in a sliding comparison
///
/// # Returns
///
/// The count of lines whose numeric value are greater than the preceding value.
///
/// # Examples
///
/// ## `window_size = 1`
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
///
/// ## `window_size = 3`
///
/// Considering a sliding window, we can compare sets of lines rather than individual lines:
///
/// ```
/// 199  A      
/// 200  A B    
/// 208  A B C  
/// 210    B C D
/// 200  E   C D
/// 207  E F   D
/// 240  E F G  
/// 269    F G H
/// 260      G H
/// 263        H
/// ```
///
/// Start by comparing the first and second three-measurement windows.
/// The measurements in the first window are marked A (199, 200, 208); their sum is 199 + 200 + 208 = 607.
/// The second window is marked B (200, 208, 210); its sum is 618.
/// The sum of measurements in the second window is larger than the sum of the first, so this first comparison increased.
///  
/// In this example, the sum of each three-measurement window is as follows:
///
/// ```
/// A: 607 (N/A - no previous sum)
/// B: 618 (increased)
/// C: 618 (no change)
/// D: 617 (decreased)
/// E: 647 (increased)
/// F: 716 (increased)
/// G: 769 (increased)
/// H: 792 (increased)
/// ```
///
/// Leading to 5 windows with an increase.
fn count_numeric_increases(input_path: &str, window_size: usize) -> i32 {
    // Create a buffer to read the file line by line
    let contents =
        File::open(input_path).expect(format!("Error reading file: {}", input_path).as_str());
    let reader = BufReader::new(contents);

    // Read each number into a window, removing stale window elements as we traverse the file
    let mut window: VecDeque<i32> = VecDeque::new();
    let mut count_increases = 0;

    for line in reader.lines() {
        let line = line.expect("Failed to parse line from file.");
        let number = line
            .parse::<i32>()
            .expect("Error parsing number from file.");

        // If the window is the expected size, then we've parsed at least window_size numbers out of the file and can compare
        if window.len() == window_size {
            // Get the size of the old window
            let old_size: i32 = window.iter().sum();
            // Drop the oldest element, ignoring errors because we know it's there
            let stale = window.pop_front().unwrap();
            // Get the size of the new window
            let new_size: i32 = old_size - stale + number;
            if new_size > old_size {
                count_increases += 1;
            }
        }
        // Update the window with the latest value
        window.push_back(number);
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
fn parse_file_path(args: &[String]) -> (&str, usize) {
    if !(args.len() == 2 || args.len() == 3) {
        panic!(
            "Expected one file path and an optional window size to run against, got: {} arguments",
            args.len() - 1
        );
    }
    let input_path = &args[1];
    if args.len() == 2 {
        return (input_path.as_str(), 1);
    }
    let window_size = &args[2]
        .parse::<usize>()
        .expect("Failed to parse window size.");
    (input_path.as_str(), *window_size)
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
    let (input_path, window_size) = parse_file_path(&args);
    println!(
        "Found {} increases",
        count_numeric_increases(input_path, window_size)
    );
}

#[cfg(test)]
mod test_parse_file_path {
    use crate::parse_file_path;

    #[test]
    fn one_arg_ok() {
        assert_eq!(
            parse_file_path(&vec!["script_path".to_string(), "arg_text".to_string()][..]),
            ("arg_text", 1)
        );
    }

    #[test]
    #[should_panic]
    fn no_arg_fail() {
        parse_file_path(&Vec::new());
    }

    #[test]
    fn window_arg_ok() {
        assert_eq!(
            parse_file_path(
                &vec![
                    "script_path".to_string(),
                    "arg_text".to_string(),
                    "5".to_string()
                ][..]
            ),
            ("arg_text", 5)
        );
    }

    #[test]
    #[should_panic]
    fn bad_window_arg_fail() {
        parse_file_path(
            &vec![
                "script_path".to_string(),
                "arg_text".to_string(),
                "extra_arg".to_string(),
            ][..],
        );
    }

    #[test]
    #[should_panic]
    fn many_arg_fail() {
        parse_file_path(
            &vec![
                "script_path".to_string(),
                "arg_text".to_string(),
                "5".to_string(),
                "extra_arg".to_string(),
            ][..],
        );
    }
}

#[cfg(test)]
mod test_count_numeric_increases {
    use crate::count_numeric_increases;

    #[test]
    fn example_correct_small_window() {
        assert_eq!(count_numeric_increases("inputs/example.txt", 1), 7);
    }

    #[test]
    fn question_correct_small_window() {
        assert_eq!(count_numeric_increases("inputs/challenge.txt", 1), 1446);
    }

    #[test]
    fn example_correct_med_window() {
        assert_eq!(count_numeric_increases("inputs/example.txt", 3), 5);
    }

    #[test]
    fn question_correct_med_window() {
        assert_eq!(count_numeric_increases("inputs/challenge.txt", 3), 1486);
    }

    #[test]
    #[should_panic]
    fn error_file_handled() {
        count_numeric_increases("inputs/noexist.txt", 1);
    }
}
