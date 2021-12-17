use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::vec::IntoIter;

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

#[derive(Debug)]
struct Packet {
    id: usize,
    version: usize,
    mode: Option<usize>,
    sub_packet_size: Option<usize>,
    value: Option<usize>,
    bits_read: usize,
}

impl Packet {
    fn hex() -> HashMap<String, String> {
        [
            ("0", "0000"),
            ("1", "0001"),
            ("2", "0010"),
            ("3", "0011"),
            ("4", "0100"),
            ("5", "0101"),
            ("6", "0110"),
            ("7", "0111"),
            ("8", "1000"),
            ("9", "1001"),
            ("A", "1010"),
            ("B", "1011"),
            ("C", "1100"),
            ("D", "1101"),
            ("E", "1110"),
            ("F", "1111"),
        ]
        .iter()
        .map(|(a, b)| (a.to_string(), b.to_string()))
        .collect()
    }

    fn parse_to_int(it: &mut IntoIter<String>, take: usize) -> usize {
        let parts = it.by_ref().take(take).collect::<Vec<String>>();
        let bytestr = parts.join("");
        // println!("Parsed bytes: {}", bytestr);
        usize::from_str_radix(bytestr.as_str(), 2).expect("Failed to parse bytes as int.")
    }

    fn parse_literal(it: &mut IntoIter<String>) -> (usize, usize) {
        let mut bits_read = 0;
        let mut has_more_to_read = true;
        let mut target_bits = Vec::new();
        while has_more_to_read {
            has_more_to_read = Packet::parse_to_int(it, 1) == 1;
            target_bits.extend(it.by_ref().take(4).collect::<Vec<String>>());
            bits_read += 5;
        }
        let target_size = target_bits.len();
        (
            Packet::parse_to_int(&mut target_bits.into_iter(), target_size),
            bits_read,
        )
    }

    fn parse_packet(it: &mut IntoIter<String>) -> Packet {
        let mut bits_read = 0;
        let version = Packet::parse_to_int(it, 3);
        let id = Packet::parse_to_int(it, 3);
        // println!("Found id {} version {}", id, version);
        bits_read += 6;
        match id {
            4 => {
                // println!("Found literal");
                let (value, read) = Packet::parse_literal(it);
                bits_read += read;
                println!(
                    "Literal: {}/{} value: {}, read: {}",
                    id, version, value, bits_read
                );
                Packet {
                    id: id,
                    version: version,
                    mode: None,
                    sub_packet_size: None,
                    value: Some(value),
                    bits_read: bits_read,
                }
            }
            _ => {
                // println!("Found operator");
                let ptype = Packet::parse_to_int(it, 1);
                if ptype == 0 {
                    let size = Packet::parse_to_int(it, 15);
                    println!(
                        "Operator0: {}/{} size: {}, read: {}",
                        id, version, size, bits_read
                    );
                    Packet {
                        id: id,
                        version: version,
                        mode: Some(0),
                        sub_packet_size: Some(size),
                        value: None,
                        bits_read: bits_read + 16,
                    }
                } else {
                    let size = Packet::parse_to_int(it, 11);
                    println!(
                        "Operator1: {}/{} size: {}, read: {}",
                        id, version, size, bits_read
                    );
                    Packet {
                        id: id,
                        version: version,
                        mode: Some(1),
                        sub_packet_size: Some(size),
                        value: None,
                        bits_read: bits_read + 12,
                    }
                }
            }
        }
    }

    fn parse_mode0(it: &mut IntoIter<String>, size: usize) -> usize {
        // println!("Parsing mode 0 packet of size: {} bits", size);
        let mut to_read = size;
        let mut version_sum = 0;
        while to_read > 0 {
            let p = Packet::parse_packet(it);
            // println!("Read bits parsing packet: {} out of {}", p.bits_read, to_read);
            to_read -= p.bits_read;
            version_sum += p.version;
        }
        version_sum
    }

    fn parse_mode1(it: &mut IntoIter<String>, size: usize) -> usize {
        let mut version_sum = 0;
        for _ in 0..size {
            let p = Packet::parse_packet(it);
            version_sum += p.version;
            // If this packet contains operator packets, then parse these sub-packets as well
            match p.mode {
                Some(0) => version_sum += Packet::parse_mode0(it, p.sub_packet_size.unwrap()),
                Some(1) => version_sum += Packet::parse_mode1(it, p.sub_packet_size.unwrap()),
                _ => (),
            }
        }
        version_sum
    }

    fn from_line(line: String) -> usize {
        let hex = Packet::hex();
        let bits = line
            .split("")
            .filter(|s| s != &"")
            .flat_map(|c| hex.get(c).unwrap().split(""))
            // .map(|b| if b == "0" {false} else {true})
            .filter(|s| s != &"")
            .map(|c| c.to_string())
            .collect::<Vec<String>>();

        println!("Bits total: {}", bits.len());
        let mut bits = bits.into_iter();
        let p = Packet::parse_packet(&mut bits);
        let version_sum = {
            match p.mode {
                None => p.version,
                Some(0) => p.version + Packet::parse_mode0(&mut bits, p.sub_packet_size.unwrap()),
                Some(1) => p.version + Packet::parse_mode1(&mut bits, p.sub_packet_size.unwrap()),
                _ => 0,
            }
        };
        println!("Total version sum: {}", version_sum);
        version_sum
    }
}
/// Parse a packet of binary into hex:
///
/// Packet structure:
///
/// * All numbers are encoded as binary with the most significant bit first
///   * For example, 100 represents the number 4
/// * Header:
///   * Three bits for the packet version (int)
///   * Three bits for the packet type ID (int)
/// * ID 4: a literal value
///   * A single binary number
///   * Number is padded with leading zeroes until its length is a multiple of four bits
///   * Then it is broken into groups of four bits
///   * Each group is prefixed by a 1 bit except the last group, which is prefixed by a 0 bit.
///   * These groups of five bits immediately follow the packet header
///   * For example, the hexadecimal string D2FE28 becomes:
///     110 - version
///     100 - ID = 4
///     10111 - First number
///     11110 - Second number
///     00101 - Last number
///     000 - Padding
/// * All other IDs represent an operator
///   * An operator packet contains one or more packets.
///   * An operator packet can use one of two modes indicated by the bit immediately after the packet header
///   * Mode 0: The next 15 bits are a number that represents the total length in bits of the sub-packets contained by this packet.
///   * Mode 1: The next 11 bits are a number that represents the number of sub-packets immediately contained by this packet.
///   * For example, the hexadecimal string 38006F45291200 becomes:
///     001 - version
///     110 - ID - 6
///     0 - Mode 0
///     000000000011011 - Sub packet is 27 bits
///     110 - Version
///     100 - ID = 4
///     01010 - First/last number
///     010 - Version
///     100 - ID = 4
///     10001 - First number
///     00100 - Last number / end of sub-packet
///     0000000 - Padding
///
/// # Arguments
///
/// * `input_path` - The input file path containing the packets to parse.
///
/// # Returns
///
/// The sum of all version numbers in the packets.
fn solution(input_path: &str) -> Vec<usize> {
    get_buf_reader(input_path)
        .lines()
        .map(|line| {
            let line = line.expect("Failed to parse line from file.");
            println!("----------------");
            println!("Starting hex: {}", line);
            Packet::from_line(line)
        })
        .collect::<Vec<usize>>()
}

/// Print the packet version sums for each packet in the input file.
///
/// Usage:
///
/// ```
/// $ aoc inputs/example.txt
/// Packet version sums: [6, 9, 14, 16, 12, 23, 31]
/// ```
fn main() {
    let args: Vec<String> = std::env::args().collect();
    let input_path = parse_file_path(&args);
    let sol = solution(input_path);
    println!("Packet version sums: {:?}", sol);
}

#[cfg(test)]
mod test_solution {
    use crate::solution;

    #[test]
    fn example_correct() {
        assert_eq!(
            solution("inputs/example.txt"),
            vec![6, 9, 14, 16, 12, 23, 31]
        );
    }

    #[test]
    fn question_correct() {
        assert_eq!(solution("inputs/challenge.txt"), vec![852]);
    }
}
