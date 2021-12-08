use std::collections::HashMap;
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

/// Counts the number of occurrences of digits 1, 4, 7, and 8 in an encoded input file.
///
/// Encoding is a random string of characters, where each group of characters represents the representation
/// of the digit in a seven-segment display:
///
/// ```
///  aaaa    
/// b    c
/// b    c  
///  dddd   
/// e    f
/// e    f  
///  gggg   
/// ```
///
/// E.g. cf here would represent a one.
///
/// The input file contains many different encodings, where a/b/c etc. are randomly mapped to a digit segment:
///
/// ```
/// be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb
/// 1      8   9/6/0   9/6/0  4   2/3/5 9/6/0  2/3/5 2/3/5  7
/// ```
///
/// This string
/// # Arguments
///
/// * `input_path - The input file path containing the encoded data
///
/// # Returns
///
/// The occurences of 1, 4, 7, and 8 in the output data.
///
/// # Examples
///
/// ## Basic
///
/// In the following example, we see that there are 26 instances of 1, 4, 7, and 8.
///
/// ```
/// be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe
/// edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc
/// fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg
/// fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb
/// aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea
/// fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb
/// dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe
/// bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef
/// egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb
/// gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce
/// ```
fn solution(input_path: &str) -> i32 {
    let reader = get_buf_reader(input_path);
    let mut digit_count = 0;
    for line in reader.lines() {
        let line = line.expect("Failed to parse line from file.");
        let (digits, outputs) = line
            .split_once(" | ")
            .expect("Failed to parse input line into digits.");

        let sort_string = |s: &str| {
            let mut parts = s.trim().split("").collect::<Vec<&str>>();
            parts.sort();
            parts.join("")
        };
        
        let mut digit_map = HashMap::new();
        for digit in digits
            .split(" ")
            .map(sort_string)
            .filter(|s| s != "")
        {
            println!("{}", digit);
            match digit.len() {
                2 => {
                    let _ = digit_map.insert(digit, 1);
                }
                3 => {
                    let _ = digit_map.insert(digit, 7);
                }
                4 => {
                    let _ = digit_map.insert(digit, 4);
                }
                7 => {
                    let _ = digit_map.insert(digit, 8);
                }
                _ => (),
            }
        }

        for output in outputs
            .split(" ")
            .map(sort_string)
            .filter(|s| s != "")
        {
            match digit_map.contains_key(&output) {
                true => digit_count += 1,
                false => (),
            }
        }
    }
    digit_count
}

/// Print the count of digits in an encoded input.
///
/// Usage:
///
/// ```
/// $ aoc inputs/example.txt
/// Number of 1, 4, 7, 8 digits: 26
/// ```
fn main() {
    let args: Vec<String> = std::env::args().collect();
    let input_path = parse_file_path(&args);
    let sol = solution(input_path);
    println!("Number of 1, 4, 7, 8 digits: {:?}", sol);
}

#[cfg(test)]
mod test_solution {
    use crate::solution;

    #[test]
    fn example_correct() {
        assert_eq!(solution("inputs/example.txt"), 26);
    }

    #[test]
    fn question_correct() {
        assert_eq!(solution("inputs/challenge.txt"), 504);
    }
}
