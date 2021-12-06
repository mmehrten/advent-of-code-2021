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

struct Board {
    dim: usize,
    slots: HashMap<String, usize>,
    map: Vec<bool>,
}

impl Board {
    /// The sum of all tiles in the board that were not called.
    fn unmarked_sum(&mut self) -> i32 {
        let mut sum = 0;
        for (call, idx) in &self.slots {
            if !self.map[*idx] {
                sum += call.parse::<i32>().unwrap();
            }
        }
        sum
    }

    /// Return True if the board has a horizontal winning row.
    fn has_horizontal(&mut self) -> bool {
        // Check map[0:5], map[5:10], map[10:15], map[15:20], map[20:25]
        for base in (0..self.map.len()).step_by(self.dim) {
            let mut has_win = true;
            for idx in base..base + self.dim {
                if !self.map[idx] {
                    has_win = false;
                    break;
                }
            }
            if has_win {
                return has_win;
            }
        }
        false
    }
    /// Return True if the board has a vertical winning column.
    fn has_vertical(&mut self) -> bool {
        // Check map[0:5:20], map[1:5:21], map[2:5:22], map[3:5:23], map[4:5:24]
        for base in (0..self.dim) {
            let mut has_win = true;
            for idx in (base..self.dim * self.dim).step_by(self.dim) {
                if !self.map[idx] {
                    has_win = false;
                    break;
                }
            }
            if has_win {
                return has_win;
            }
        }
        false
    }
    /// Return True if the board has a winning diagonal.
    fn has_diagonal(&mut self) -> bool {
        // Check map[0, 6, 12, 18, 24], map[4, 8, 12, 16, 20]
        let mut has_win_l = true;
        for (offset, idx) in (0..self.dim * self.dim).step_by(self.dim).enumerate() {
            let idx = &(idx + offset);
            if !self.map[*idx] {
                has_win_l = false;
                break;
            }
        }

        let mut has_win_r = true;
        for (offset, idx) in (self.dim - 1..self.dim * self.dim - self.dim)
            .step_by(self.dim)
            .enumerate()
        {
            let idx = &(idx - offset);
            if !self.map[*idx] {
                has_win_r = false;
                break;
            }
        }

        has_win_l || has_win_r
    }
    /// Return True if the board has horizontal or vertical wins - ignores diagonal wins.
    fn has_win(&mut self) -> bool {
        self.has_horizontal()
            || self.has_vertical()
    }
}

#[cfg(test)]
mod test_board {
    use crate::Board;
    use std::collections::HashMap;

    #[test]
    fn unmarked_sum() {
        let mut slots = HashMap::new();
        let mut map = Vec::new();
        slots.insert("1".to_string(), 0);
        slots.insert("1234".to_string(), 1);
        map.push(true);
        map.push(false);

        let mut b = Board {
            dim: 2,
            slots: slots,
            map: map,
        };

        assert_eq!(b.unmarked_sum(), 1234);
    }

    #[test]
    fn no_horiz_win() {
        let slots = HashMap::new();
        let mut map = Vec::new();
        map.push(true);
        map.push(false);
        map.push(false);

        map.push(false);
        map.push(false);
        map.push(false);

        map.push(false);
        map.push(false);
        map.push(false);

        let mut b = Board {
            dim: 3,
            slots: slots,
            map: map,
        };

        assert!(!b.has_horizontal());
        assert!(!b.has_win());
    }

    #[test]
    fn horiz_win() {
        let slots = HashMap::new();
        let mut map = Vec::new();
        map.push(true);
        map.push(true);
        map.push(true);

        map.push(false);
        map.push(false);
        map.push(false);

        map.push(false);
        map.push(false);
        map.push(false);

        let mut b = Board {
            dim: 3,
            slots: slots,
            map: map,
        };

        assert!(b.has_horizontal());
        assert!(b.has_win());
    }

    #[test]
    fn no_vert_win() {
        let slots = HashMap::new();
        let mut map = Vec::new();
        map.push(true);
        map.push(false);
        map.push(false);

        map.push(false);
        map.push(false);
        map.push(false);

        map.push(false);
        map.push(false);
        map.push(false);

        let mut b = Board {
            dim: 3,
            slots: slots,
            map: map,
        };

        assert!(!b.has_vertical());
        assert!(!b.has_win());
    }

    #[test]
    fn vert_win() {
        let slots = HashMap::new();
        let mut map = Vec::new();
        map.push(true);
        map.push(false);
        map.push(false);

        map.push(true);
        map.push(false);
        map.push(false);

        map.push(true);
        map.push(false);
        map.push(false);

        let mut b = Board {
            dim: 3,
            slots: slots,
            map: map,
        };

        assert!(b.has_vertical());
        assert!(b.has_win());
    }

    #[test]
    fn no_diag_win() {
        let slots = HashMap::new();
        let mut map = Vec::new();
        map.push(true);
        map.push(false);
        map.push(false);

        map.push(false);
        map.push(false);
        map.push(false);

        map.push(false);
        map.push(false);
        map.push(false);

        let mut b = Board {
            dim: 3,
            slots: slots,
            map: map,
        };

        assert!(!b.has_diagonal());
        assert!(!b.has_win());
    }

    #[test]
    fn diag_win_l() {
        let slots = HashMap::new();
        let mut map = Vec::new();
        map.push(true);
        map.push(false);
        map.push(false);

        map.push(false);
        map.push(true);
        map.push(false);

        map.push(false);
        map.push(false);
        map.push(true);

        let mut b = Board {
            dim: 3,
            slots: slots,
            map: map,
        };

        assert!(b.has_diagonal());
        assert!(!b.has_win());
    }

    #[test]
    fn diag_win_r() {
        let slots = HashMap::new();
        let mut map = Vec::new();
        map.push(false);
        map.push(false);
        map.push(true);

        map.push(false);
        map.push(true);
        map.push(false);

        map.push(true);
        map.push(false);
        map.push(false);

        let mut b = Board {
            dim: 3,
            slots: slots,
            map: map,
        };

        assert!(b.has_diagonal());
        assert!(!b.has_win());
    }
}

struct Solution {
    board: Board,
    rounds_to_win: usize,
    winning_result: i32,
}

impl Solution {
    /// The "score" of the result is defined as the sum of all unplayed tiles, times the tile that gave us the win.
    fn score(&mut self) -> i32 {
        self.board.unmarked_sum()
            * self.winning_result
    }
}

/// Parse a bingo game as inputs and report a winning board, as well as the worst-losing board, scores.
///
/// # Arguments
///
/// * `input_path - The input file path containing the bingo game.
///
/// # Returns
///
/// The score of the winning board and worst-losing board.
///
/// # Examples
///
/// ## Basic
/// Bingo input has the format:
///
/// ```
/// 7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1
///
/// 22 13 17 11  0
///  8  2 23  4 24
/// 21  9 14 16  7
///  6 10  3 18  5
///  1 12 20 15 19
///
///  3 15  0  2 22
///  9 18 13 17  5
/// 19  8  7 25 23
/// 20 11 10 24  4
/// 14 21 16 12  6
///
/// 14 21 17 24  4
/// 10 16 15  9 19
/// 18  8 23 26 20
/// 22 11 13  6  5
///  2  0 12  3  7
/// ```
///
/// Where the first row indicates the order of bingo calls.
///
/// Returns the score of the winning board can now be calculated.
/// The score is calculated by:
///
/// *  The sum of all unmarked numbers on the winning board
/// *  Multiplied by the number that caused the board to win
///
/// So in this case 188 * 24 = 4512 for the best board, and 148 * 13 = 1924 for the worst
fn solution(input_path: &str) -> (i32, i32) {
    let reader = get_buf_reader(input_path);
    let mut lines = reader.lines().map(|l| l.unwrap());
    let mut calls: Vec<String> = lines
        .next()
        .expect("Failed to parse moves from input")
        // .expect("Failed to parse moves from input")
        .split(",")
        .map(|x| x.to_string())
        .collect();

    let mut winning_scores: Vec<Solution> = Vec::new();

    let mut board_repr = Vec::new();
    let mut board_dim: Option<usize> = None; // Set on first iteration
    let mut expected_size: Option<usize> = None;
    for line in lines {
        let entry: Vec<String> = line
            .split(" ")
            .filter(|x| x.trim() != "")
            .map(|x| x.to_string())
            .collect();
        if entry.len() == 0 {
            continue;
        }
        // Set board dimensions on first iteration
        if expected_size.is_none() {
            board_dim = Some(entry.len());
            expected_size = Some(entry.len() * entry.len());
        }

        board_repr.extend(entry);
        // If we haven't met the proper dimension, keep parsing inputs
        if board_repr.len() != expected_size.unwrap() {
            continue;
        }

        // We've got a full board, so now we can parse into our Board struct
        let mut slots = HashMap::new();
        let mut map = Vec::new();
        for (idx, key) in board_repr.iter().enumerate() {
            slots.insert(key.clone().to_string(), idx);
            map.push(false);
        }

        let mut board = Board {
            dim: board_dim.unwrap(),
            slots: slots,
            map: map,
        };
        // Now parse all the moves that were called into the board
        for (to_win, call) in calls.iter().enumerate() {
            match board.slots.get(call) {
                // If this move is in our board, let's add it and check if we've got bingo
                Some(idx) => {
                    board.map[*idx] = true;
                    // If we have bingo, we're done! Let's add a potential solution and move to the next board
                    if board.has_win() {
                        winning_scores.push(Solution {
                            board: board,
                            rounds_to_win: to_win,
                            winning_result: call.parse::<i32>().unwrap(),
                        });
                        break;
                    }
                }
                _ => (),
            }
        }

        // We've parsed all the called moves into this board, create a new entry
        board_repr = Vec::new();
    }

    // All boards are processed, check for the winning board
    let mut best_score = 0;
    let mut best_turn_count: Option<usize> = None;
    let mut worst_score = 0;
    let mut worst_turn_count: Option<usize> = None;
    for mut sol in winning_scores {
        if best_turn_count.is_none() || worst_turn_count.is_none() {
            best_score = sol.score();
            best_turn_count = Some(sol.rounds_to_win);
            worst_score = sol.score();
            worst_turn_count = Some(sol.rounds_to_win);
        }

        if sol.rounds_to_win < best_turn_count.unwrap() {
            best_score = sol.score();
            best_turn_count = Some(sol.rounds_to_win);
        }
        if sol.rounds_to_win > worst_turn_count.unwrap() {
            worst_score = sol.score();
            worst_turn_count = Some(sol.rounds_to_win);
        }
    }
    // TODO: Can we express this as a map / reduce instead?
    // winning_scores.map(|x| x.score()).max().unwrap();
    (best_score, worst_score)
}

/// TODO
///
/// Usage:
///
/// ```
/// $ day-TODO inputs/example.txt
/// TODO
/// ```
fn main() {
    let args: Vec<String> = std::env::args().collect();
    let input_path = parse_file_path(&args);
    let sol = solution(input_path);
    println!("Winning score: {:?}", sol);
}

#[cfg(test)]
mod test_solution {
    use crate::solution;

    #[test]
    fn example_correct() {
        assert_eq!(solution("inputs/example.txt"), (4512, 1924));
    }

    #[test]
    fn question_correct() {
        assert_eq!(solution("inputs/challenge.txt"), (35670, 22704));
    }
}
