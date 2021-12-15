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

/// Parse a polymer creation template and return the final polymer chain after N steps.
///
/// Templates have the form:
///
/// ```
/// NNCB
///
/// CH -> B
/// HH -> N
/// CB -> H
/// NH -> C
/// HB -> C
/// HC -> B
/// HN -> C
/// NN -> C
/// BH -> H
/// NC -> B
/// NB -> B
/// BN -> B
/// BB -> N
/// BC -> B
/// CC -> N
/// CN -> C
/// ```
///
/// Where the first line `NNCB` is the polymer template,
/// and the subsequent lines are insertion rules indicating that pairs
/// of letters should have new characters inserted between them (eg. `CH` becomes `CBH`).
///
/// These rules can be applied multiple times to the starting string to create a final
/// polymer chain.
///
/// # Arguments
///
/// * `input_path` - The input file path containing the polymer rules.
/// * `num_steps` - The number of times to apply insertion rules
///
/// # Returns
///
/// The quantity of the most common element minus the quantity of the least common element after N steps.
fn solution(input_path: &str, num_steps: usize) -> usize {
    let reader = get_buf_reader(input_path);
    let mut lines = reader.lines();

    // Parse the polymer starting string into a list of single characters
    let polymer = lines
        .next()
        .expect("Empty file found.")
        .expect("Empty file found.")
        .split("")
        .filter(|s| s != &"")
        .map(|s| s.to_string())
        .collect::<Vec<String>>();

    // Count all pairs in the current string
    // This is where we will store the running totals of character occurrences,
    // as well as occurences of pairs of characters
    let mut pair_counts = HashMap::new();
    for idx in 0..polymer.len() - 1 {
        let match_pair = polymer[idx].clone() + &polymer[idx + 1];
        pair_counts
            .entry(match_pair)
            .and_modify(|v| *v += 1)
            .or_insert(1);
        pair_counts
            .entry(polymer[idx].clone())
            .and_modify(|v| *v += 1)
            .or_insert(1);
        // pair_counts.entry(polymer[idx + 1].clone()).and_modify(|v| *v += 1).or_insert(1);
    }
    pair_counts
        .entry(polymer[polymer.len() - 1].clone())
        .and_modify(|v| *v += 1)
        .or_insert(1);
    println!("{:?}", pair_counts);

    // Parse the mapping rules
    let mut mappings = HashMap::new();
    while let Some(line) = lines.next() {
        let line = line
            .expect("Failed to read line from file")
            .trim()
            .to_string();
        if line == "" {
            continue;
        }
        let mut parts = line
            .split(" -> ")
            .map(|s| s.to_string())
            .collect::<Vec<String>>();
        let to_insert = parts.pop().expect("Invalid mapping line");
        let match_pair = parts.pop().expect("Invalid mapping line");
        mappings.insert(match_pair, to_insert);
    }

    // Now apply the mapping rules
    for _ in 0..num_steps {
        // Clone the original pairs to store as a reference for modified values
        // Otherwise we update the counts as we iterate which produces inconsistent values
        let mut pair_counts_mut = pair_counts.clone();
        for (match_pair, to_insert) in &mappings {
            if !pair_counts.contains_key(match_pair) {
                continue;
            }

            // General Rust TODO: It would be really nice to avoid all of this cloning.
            // This seems like a code smell that indicates we're not building our ownership hierarchy as well
            // as we could be...
            
            // When we divide this monomer with count N, the resulting two monomers will have count N as well
            let current_count_pair = pair_counts.get(match_pair).unwrap().clone();

            // Build the two new monomers
            let (left_part, right_part) = match_pair.split_at(1);
            let left = left_part.to_string() + &to_insert;
            let right = to_insert.clone() + &right_part;

            // Update the counts for monomers
            pair_counts_mut
                .entry(left)
                .and_modify(|v| *v += current_count_pair)
                .or_insert(current_count_pair);
            pair_counts_mut
                .entry(right)
                .and_modify(|v| *v += current_count_pair)
                .or_insert(current_count_pair);
            // Decrement the original pair that we had, since that monomer is gone now
            pair_counts_mut
                .entry(match_pair.clone())
                .and_modify(|v| *v -= current_count_pair);
            // In addition to the two new monomers, we'll also get N of the newly inserted value
            pair_counts_mut
                .entry(to_insert.clone())
                .and_modify(|v| *v += current_count_pair)
                .or_insert(current_count_pair);
        }
        // We're done modifying, so we can store the modified counts back in the original variable
        pair_counts = pair_counts_mut;
    }

    // Get the counts of each building-block (excluding monomers)
    let pair_counts = {
        let mut v = Vec::new();
        for (key, value) in pair_counts {
            if key.len() != 1 {
                continue;
            }
            v.push(value);
        }
        v
    };
    pair_counts.iter().max().unwrap() - pair_counts.iter().min().unwrap()
}

/// Parse a set of polymer building instructions, and print the quantity of the most
/// common element minus the quantity of the least common element after 10 steps.
///
/// Usage:
///
/// ```
/// $ aoc inputs/example.txt
/// Count of most common - count of least common: 1588
/// ```
fn main() {
    let args: Vec<String> = std::env::args().collect();
    let input_path = parse_file_path(&args);
    let sol = solution(input_path, 10);
    println!("Count of most common - count of least common: {:?}", sol);
}

#[cfg(test)]
mod test_solution {
    use crate::solution;

    #[test]
    fn example_correct() {
        assert_eq!(solution("inputs/example.txt", 40), 2188189693529);
    }

    #[test]
    fn question_correct() {
        assert_eq!(solution("inputs/challenge.txt", 40), 4807056953866);
    }
}
