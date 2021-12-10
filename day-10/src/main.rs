use std::collections::VecDeque;
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

const OPENERS: [&str; 4] = ["(", "{", "[", "<"];
const CLOSERS: [&str; 4] = [")", "}", "]", ">"];
const MALFORMED_SCORES: [usize; 4] = [3, 1197, 57, 25137];
const INCOMPLETE_SCORES: [usize; 4] = [1, 3, 2, 4];

/// Return the syntax error score and the "middle" autocomplete score in a given file of (), [], {}, <> characters.
///
/// A syntax error is any malformed / unclosed combination of opening and closing characters.
///
/// Each malformed line gets a syntax error score based on the first incorrect character, with point values being:
///  
///  * ): 3 points.
///  * ]: 57 points.
///  * }: 1197 points.
///  * >: 25137 points.
///
/// The overall syntax error score is the sum of the scores for each line, with point values being:
///
/// Each incomplete line gets an autocomplete score based on the characters needed to complete the line,
///
///  * ): 1 point.
///  * ]: 2 points.
///  * }: 3 points.
///  * >: 4 points.
///
/// Starting with a total score of 0, then, for each character, multipling the total score by 5 and increasing the total score by the point value given for the character.
///
/// # Arguments
///
/// * `input_path - The input file path containing the characters to check.
///
/// # Returns
///
/// The syntax error score.
///
/// # Examples
///
/// ## Basic
///
/// The syntax lines below have an overall score of 26397, and the middle-most incomplete score of 288957:
/// ```
/// [({(<(())[]>[[{[]{<()<>>
/// [(()[<>])]({[<{<<[]>>(
/// {([(<{}[<>[]}>{[]{[(<()>
/// (((({<>}<{<{<>}{[]{[]{}
/// [[<[([]))<([[{}[[()]]]
/// [{[{({}]{}}([{[{{{}}([]
/// {<[[]]>}<{[{[{[]{()[[[]
/// [<(<(<(<{}))><([]([]()
/// <{([([[(<>()){}]>(<<{{
/// <{([{{}}[<[[[<>{}]]]>[]]
/// ```
fn solution(input_path: &str) -> (usize, usize) {
    let reader = get_buf_reader(input_path);
    let lines = reader.lines();
    let mut syntax_score = 0;
    let mut incomplete_scores = Vec::new();
    for line in lines {
        let line = line
            .expect("Failed to parse line from file.")
            .split("")
            .map(|s| s.trim().to_string())
            .filter(|s| s != &"")
            .collect::<Vec<String>>();
        let mut char_deque = VecDeque::new();
        let mut is_malformed = false;
        for c in line {
            for (idx, open) in OPENERS.iter().enumerate() {
                if c != *open {
                    continue;
                }
                char_deque.push_back(CLOSERS[idx]);
                break;
            }
            for (idx, close) in CLOSERS.iter().enumerate() {
                if c != *close {
                    continue;
                }
                let expected_close = char_deque.pop_back();
                if expected_close.is_none() || expected_close.unwrap() != *close {
                    let malformed_score = MALFORMED_SCORES[idx];
                    syntax_score += malformed_score;
                    is_malformed = true;
                }
                break;
            }
        }

        if char_deque.len() == 0 || is_malformed {
            continue;
        }

        let mut incomplete_score = 0;
        while char_deque.len() != 0 {
            let c = char_deque.pop_back().unwrap();
            for (idx, close) in CLOSERS.iter().enumerate() {
                if c != *close {
                    continue;
                }
                incomplete_score = (5 * incomplete_score) + INCOMPLETE_SCORES[idx];
            }
        }
        incomplete_scores.push(incomplete_score);
    }
    incomplete_scores.sort();
    (syntax_score, incomplete_scores[incomplete_scores.len() / 2])
}

/// Print the syntax error score in a given input file.
///
/// Usage:
///
/// ```
/// $ aoc inputs/example.txt
/// Malformed score: 26397
/// ```
fn main() {
    let args: Vec<String> = std::env::args().collect();
    let input_path = parse_file_path(&args);
    let sol = solution(input_path);
    println!("Malformed score: {:?}", sol);
}

#[cfg(test)]
mod test_solution {
    use crate::solution;

    #[test]
    fn example_correct() {
        assert_eq!(solution("inputs/example.txt"), (26397, 288957));
    }

    #[test]
    fn question_correct() {
        assert_eq!(solution("inputs/challenge.txt"), (296535, 4245130838));
    }
}
