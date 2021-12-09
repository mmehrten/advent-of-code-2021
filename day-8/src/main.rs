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

fn sort_string<S>(s: S) -> String
where
    S: Into<String>,
{
    let s = s.into();
    let mut parts = s.trim().split("").collect::<Vec<&str>>();
    parts.sort();
    parts.join("")
}

fn clean_input(line: &str) -> Vec<String> {
    line.split(" ")
        .map(sort_string)
        .filter(|s| s != "")
        .collect()
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
    let mut digit_sum = 0;
    for line in reader.lines() {
        let line = line.expect("Failed to parse line from file.");
        let (digits, outputs) = line
            .split_once(" | ")
            .expect("Failed to parse input line into digits.");

        let digits: Vec<String> = clean_input(digits);
        let outputs: Vec<String> = clean_input(outputs);

        let mut digit_map = HashMap::new();
        for digit in &digits {
            match digit.len() {
                2 => {
                    let _ = digit_map.insert(1, digit);
                }
                3 => {
                    let _ = digit_map.insert(7, digit);
                }
                4 => {
                    let _ = digit_map.insert(4, digit);
                }
                7 => {
                    let _ = digit_map.insert(8, digit);
                }
                _ => (),
            }
        }

        let mut seven_segments: Vec<String> = Vec::new();
        for _ in 0..7 {
            seven_segments.push("-".to_string());
        }

        let mut counter = HashMap::new();
        for digit in &digits {
            let chars: Vec<String> = digit
                .split("")
                .map(|s| s.to_string())
                .filter(|s| s != "")
                .collect();
            for c in chars {
                counter.entry(c).and_modify(|v| *v += 1).or_insert(1);
            }
        }
        // a comes from 7 - 1
        for chr in digit_map.get(&7).unwrap().split("") {
            match digit_map.get(&1).unwrap().contains(chr) {
                false => seven_segments[0] = chr.to_string(),
                _ => (),
            }
        }

        for (chr, count) in counter {
            match count {
                4 => seven_segments[4] = chr, // e has 9 occurrences
                6 => seven_segments[1] = chr, // b has 9 occurrences
                9 => seven_segments[5] = chr, // f has 9 occurrences
                8 => {
                    // both c and a have 8 occurrences, so choose the char that's not mapped to a already
                    if seven_segments[0] != chr {
                        seven_segments[2] = chr;
                    }
                }
                _ => (),
            }
        }

        // We're now just missing d, g
        //
        // The digit definitions are as follows:
        //
        // 0 = a + b + c + e + f + g
        // 1 = c + f
        // 2 = a + c + d + e + g
        // 3 = a + c + d + f + g
        // 4 = b + c + d + f
        // 5 = a + b + d + f + g
        // 6 = a + b + d + e + f + g
        // 7 = a + c + f
        // 8 = a + b + c + d + e + f + g
        // 9 = a + b + c + d + f + g
        //
        // We also have the following relationships:
        //
        // e + g = 8 - 7 - 4
        // c + f = 1
        // b + d = 4 - 1
        // a = 7 - 1

        // So, we can find d and g by subbing in our known values for e and b

        // g = 8 - 7 - 4 - e
        for chr in digit_map.get(&8).unwrap().split("") {
            if digit_map.get(&7).unwrap().contains(chr)
                || digit_map.get(&4).unwrap().contains(chr)
                || chr == seven_segments[4]
            {
                continue;
            }
            seven_segments[6] = chr.to_string();
        }

        // d = 4 - 1 - b
        for chr in digit_map.get(&4).unwrap().split("") {
            if digit_map.get(&1).unwrap().contains(chr) || chr == seven_segments[1] {
                continue;
            }
            seven_segments[3] = chr.to_string();
        }

        // ```
        //  aaaa
        // b    c
        // b    c
        //  dddd
        // e    f
        // e    f
        //  gggg
        // ```

        // We now have all seven segments mapped successfully! Let's fill in digit_map:
        let segments: Vec<Vec<usize>> = vec![
            vec![0, 1, 2, 4, 5, 6],    // 0 = a + b + c + e + f + g
            vec![2, 5],                // 1 = c + f
            vec![0, 2, 3, 4, 6],       // 2 = a + c + d + e + g
            vec![0, 2, 3, 5, 6],       // 3 = a + c + d + f + g
            vec![1, 2, 3, 5],          // 4 = b + c + d + f
            vec![0, 1, 3, 5, 6],       // 5 = a + b + d + f + g
            vec![0, 1, 3, 4, 5, 6],    // 6 = a + b + d + e + f + g
            vec![0, 2, 5],             // 7 = a + c + f
            vec![0, 1, 2, 3, 4, 5, 6], // 8 = a + b + c + d + e + f + g
            vec![0, 1, 2, 3, 5, 6],    // 9 = a + b + c + d + f + g
        ];

        let segment_strings = segments.iter().enumerate().map(|(num, seg)| {
            let s = sort_string(
                seg.iter()
                    .map(|s| seven_segments[*s].as_str())
                    .collect::<Vec<&str>>()
                    .join(""),
            );
            (num, s)
        });
        let mut digit_map = HashMap::new();
        for (num, s) in segment_strings {
            digit_map.insert(s, num);
        }
        // for (k, v) in digit_map {
        //     println!("{}: {}", k, v);
        // }
        let mut digit = "".to_string();
        for o in outputs {
            match digit_map.get(&o) {
                Some(value) => digit += value.to_string().as_str(),
                _ => panic!("Failed to find digit in mapping: {}", o),
            }
        }
        digit_sum += digit.parse::<i32>().expect("Malformed final output.");
    }
    digit_sum
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
        assert_eq!(solution("inputs/example.txt"), 61229);
    }

    #[test]
    fn question_correct() {
        assert_eq!(solution("inputs/challenge.txt"), 1073431);
    }
}
