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
/// Both the oxygen generator rating and the CO2 scrubber rating are values that can be found in your diagnostic report - finding them is the tricky part. Both values are located using a similar process that involves filtering out values until only one remains. Before searching for either rating value, start with the full list of binary numbers from your diagnostic report and consider just the first bit of those numbers. Then:
/// 
/// * Keep only numbers selected by the bit criteria for the type of rating value for which you are searching. Discard numbers which do not match the bit criteria.
/// * If you only have one number left, stop; this is the rating value for which you are searching.
/// * Otherwise, repeat the process, considering the next bit to the right.
/// 
/// The bit criteria depends on which type of rating value you want to find:
/// 
/// * To find oxygen generator rating, determine the most common value (0 or 1) in the current bit position, and keep only numbers with that bit in that position. If 0 and 1 are equally common, keep values with a 1 in the position being considered.
/// * To find CO2 scrubber rating, determine the least common value (0 or 1) in the current bit position, and keep only numbers with that bit in that position. If 0 and 1 are equally common, keep values with a 0 in the position being considered.
/// 
/// For example, to determine the oxygen generator rating value using the same example diagnostic report from above:
/// 
/// * Start with all 12 numbers and consider only the first bit of each number. There are more 1 bits (7) than 0 bits (5), so keep only the 7 numbers with a 1 in the first position: 11110, 10110, 10111, 10101, 11100, 10000, and 11001.
/// * Then, consider the second bit of the 7 remaining numbers: there are more 0 bits (4) than 1 bits (3), so keep only the 4 numbers with a 0 in the second position: 10110, 10111, 10101, and 10000.
/// * In the third position, three of the four numbers have a 1, so keep those three: 10110, 10111, and 10101.
/// * In the fourth position, two of the three numbers have a 1, so keep those two: 10110 and 10111.
/// * In the fifth position, there are an equal number of 0 bits and 1 bits (one each). So, to find the oxygen generator rating, keep the number with a 1 in that position: 10111.
/// * As there is only one number left, stop; the oxygen generator rating is 10111, or 23 in decimal.
/// 
/// Then, to determine the CO2 scrubber rating value from the same example above:
/// 
/// Start again with all 12 numbers and consider only the first bit of each number. There are fewer 0 bits (5) than 1 bits (7), so keep only the 5 numbers with a 0 in the first position: 00100, 01111, 00111, 00010, and 01010.
/// Then, consider the second bit of the 5 remaining numbers: there are fewer 1 bits (2) than 0 bits (3), so keep only the 2 numbers with a 1 in the second position: 01111 and 01010.
/// In the third position, there are an equal number of 0 bits and 1 bits (one each). So, to find the CO2 scrubber rating, keep the number with a 0 in that position: 01010.
/// As there is only one number left, stop; the CO2 scrubber rating is 01010, or 10 in decimal.
/// 
/// Finally, to find the life support rating, multiply the oxygen generator rating (23) by the CO2 scrubber rating (10) to get 230.
fn read_power_report(input_path: &str) -> (i32, i32) {
    let reader = get_buf_reader(input_path);
    // Create an array to count zero bits in each number - only two options so if zero is more than half of the lines,
    // then zero is the most common bit
    let mut zero_byte_counts = Vec::new();
    let mut line_count = 0;

    struct ByteCounter {
        position: usize,
        followers: Vec<String>,
        zero_count: i32,
        one_count: i32,
        to_zero: Box<Option<ByteCounter>>,
        to_one: Box<Option<ByteCounter>>,
    }

    let mut starting_node = ByteCounter {
        position: 0,
        followers: Vec::new(),
        zero_count: 0,
        one_count: 0,
        to_zero: Box::new(None),
        to_one: Box::new(None),
    };
    for line in reader.lines() {
        line_count += 1;
        let line = line.expect("Failed to parse line from file.");
        
        // TODO: This violates Rust memory management, but moving the starting_node ownership every iteration of the loop.
        // Unsure how to re-set the starting point each iteration to begin at the top of the graph...
        // let mut this_node = starting_node;

        for idx in 0..line.len() {
            let current_byte = line
                .get(idx..idx + 1)
                .expect("Failed to parse byte from line");

            // Handle arbitrary length binary numbers in the input file
            if idx + 1 > zero_byte_counts.len() {
                zero_byte_counts.push(0);
            }

            // this_node.followers.push(line);
            let mut new_follower = ByteCounter {
                position: idx,
                followers: Vec::new(),
                zero_count: 0,
                one_count: 0,
                        to_zero: Box::new(None),
                to_one: Box::new(None),
            };
            match current_byte {
                "0" => {
                    zero_byte_counts[idx] += 1;
                    // if this_node.to_zero.is_none() {
                    //     this_node.to_zero = Box::new(Some(new_follower));
                    // }
                    // this_node = this_node.to_zero;
                }
                "1" => {
                    // if this_node.to_one.is_none() {
                    //     this_node.to_one = Box::new(Some(new_follower));
                    // }
                    // this_node = this_node.to_one;
                }
                _ => panic!("Unexpected byte: {}", current_byte),
            }
            
        }
    }

    // Convert most common bytes to gamma & epsilon
    let mut gamma: String = String::new();
    let mut eps: String = String::new();
    let mut o2: String = String::new();
    let mut co: String = String::new();
    let o2_node = starting_node;
    let co_node = starting_node;
    for idx in 0..zero_byte_counts.len() {
        // // More ones at this depth than zeros, so choose the ones for O2 and the zeros for CO
        // if o2_node.one_count >= o2_node.zero_count {
        //     o2_node = o2_node.to_one;
        // } else {
        //     o2_node = o2_node.to_zero;            
        // }
        // if co_node.zero_count >= co_node.one_count {
        //     co_node = co_node.to_zero;
        // } else {
        //     co_node = co_node.to_one;            
        // }

        // // If we've made it to the last node in the tree, or we only have one option left, we know what number to choose
        // if o2_node.followers.len() == 1 || o2_node.position == idx {
        //     o2 = o2_node.followers[0];
        // }
        // if co_node.followers.len() == 1 || co_node.position == idx  {
        //     co = co_node.followers[0];
        // }
        if zero_byte_counts[idx] > line_count / 2 {
            gamma.push('0');
            eps.push('1');
        } else {
            gamma.push('1');
            eps.push('0');
        }
    }

    // Convert byte strings to decimal
    (
        i32::from_str_radix(gamma.as_str(), 2).expect("Failed to parse byte string as integer"),
        i32::from_str_radix(eps.as_str(), 2).expect("Failed to parse byte string as integer"),
    )
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
    println!("Power rates: ({}, {}), multiplied: {}", x, y, x * y);
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
        assert_eq!(read_power_report("inputs/challenge.txt"), (654, 3441));
    }
}
