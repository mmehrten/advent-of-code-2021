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

const NEW_FISH_TTR: usize = 8;
const OLD_FISH_TTR: usize = 6;

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
fn solution(input_path: &str, days: usize) -> usize {
    let reader = get_buf_reader(input_path);
    let population: Vec<usize> = reader
        .lines()
        .map(|line| {
            line.expect("Failed to read line from file")
                .split(",")
                .map(|s| s.parse::<usize>().expect("Failed to parse age from file."))
                .collect::<Vec<usize>>()
        })
        .flatten()
        .collect();

    fn add_key<K, V>(hash_map: &mut HashMap<K, V>, key: K, value: V)
    where
        V: std::ops::Add<Output = V>,
        K: Eq,
        K: PartialEq,
        K: std::hash::Hash,
        V: Copy,
    {
        match hash_map.get(&key) {
            Some(current_val) => {
                let tmp = hash_map.insert(key, value + *current_val);
            }
            _ => {
                let tmp = hash_map.insert(key, value);
            }
        }
    }

    let mut pop_by_time: HashMap<usize, usize> = HashMap::new();
    for fish_ttr in population {
        add_key(&mut pop_by_time, fish_ttr, 1);
    }

    for day in 0..days {
        let mut new_pop: HashMap<usize, usize> = HashMap::new();
        for (ttr, current) in pop_by_time {
            if ttr == 0 {
                // Each fish at ttr 0 reproduces - create this many NEW_FISH
                add_key(&mut new_pop, NEW_FISH_TTR, current);
                // Each fish at this new TTR ages out into an OLD_FISH timer
                add_key(&mut new_pop, OLD_FISH_TTR, current);
                continue;
            }
            // Otherwise, age this population
            add_key(&mut new_pop, ttr - 1, current);
        }

        pop_by_time = new_pop;
    }
    pop_by_time.values().sum()
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
    let days = 256;
    let sol = solution(input_path, days);
    println!("Number of lanternfish after {} days: {:?}", days, sol);
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
