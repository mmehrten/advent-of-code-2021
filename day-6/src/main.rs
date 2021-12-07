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

const NEW_FISH_TTR: i32 = 8;
const OLD_FISH_TTR: i32 = 6;

struct Fish {
    time_to_reproduce: i32,
}
impl Fish {
    fn age_one(&mut self) -> Option<Fish> {
        if self.time_to_reproduce == 0 {
            self.time_to_reproduce = OLD_FISH_TTR;
            return Some(Fish {
                time_to_reproduce: NEW_FISH_TTR,
            });
        }
        self.time_to_reproduce -= 1;
        None
    }
}

/// Return the number of lanternfish alive after X days given an initial population.
///
/// # Arguments
///
/// * `input_path - The input file path containing initial lanternfish ages.
/// * `days` - The number of days to count lanternfish over.
///
/// # Returns
///
/// The number of lanternfish after the given duration.
///
/// # Examples
///
/// ## Basic
///
/// Rules for lanternfish growth are as follows:
///
/// * Each lanternfish creates a new lanternfish once every 7 days.
/// * New lanternfish require two additional days for their first cycle (9 days).
///
/// So, suppose you have a lanternfish with an internal timer value of 3:
///
/// * After one day, its internal timer would become 2.
/// * After another day, its internal timer would become 1.
/// * After another day, its internal timer would become 0.
/// * After another day, its internal timer would reset to 6, and it would create a new lanternfish with an internal timer of 8.
/// * After another day, the first lanternfish would have an internal timer of 5, and the second lanternfish would have an internal timer of 7.
///
/// So, given initial ages of 3,4,3,1,2 - in 80 days, the population would be 5934.
fn solution(input_path: &str, days: usize) -> i32 {
    let reader = get_buf_reader(input_path);
    let mut population: Vec<Fish> = reader
        .lines()
        .map(|line| {
            line.expect("Failed to read line from file")
                .split(",")
                .map(|s| s.parse::<i32>().expect("Failed to parse age from file."))
                .map(|t| Fish {
                    time_to_reproduce: t,
                })
                .collect::<Vec<Fish>>()
        })
        .flatten()
        .collect();

    for _ in 0..days {
        for idx in 0..population.len() {
            let fish = &mut population[idx];
            match fish.age_one() {
                Some(new_fish) => population.push(new_fish),
                _ => (),
            }
        }
    }
    population.len() as i32
}

/// Print the number of lanternfish 80 days after an initial population.
///
/// Usage:
///
/// ```
/// $ aoc inputs/example.txt
/// Number of lanternfish after 80 days: 5934
/// ```
fn main() {
    let args: Vec<String> = std::env::args().collect();
    let input_path = parse_file_path(&args);
    let sol = solution(input_path, 80);
    println!("Number of lanternfish after 80 days: {:?}", sol);
}

#[cfg(test)]
mod test_solution {
    use crate::solution;

    #[test]
    fn example_correct() {
        assert_eq!(solution("inputs/example.txt", 80), 5934);
    }

    #[test]
    fn question_correct() {
        assert_eq!(solution("inputs/challenge.txt", 80), 365862);
    }
}
