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

#[derive(Debug, Clone)]
struct Packet {
    id: usize,
    version: usize,
    mode: Option<usize>,
    sub_packet_size: Option<usize>,
    value: Option<usize>,
    bits_read: usize,
}

impl Packet {
    fn comp(&self, others: &Vec<Packet>) -> usize {
        match self.id {
            // Sum
            0 => others.iter().map(|p| p.value.unwrap()).sum::<usize>(),
            // Product
            1 => others
                .iter()
                .map(|p| p.value.unwrap())
                .fold(1, |x, y| x * y),
            // Min
            2 => others.iter().map(|p| p.value.unwrap()).min().unwrap(),
            // Max
            3 => others.iter().map(|p| p.value.unwrap()).max().unwrap(),
            // Gt
            5 => {
                if &others[0].value.unwrap() > &others[1].value.unwrap() {
                    1
                } else {
                    0
                }
            }
            // Lt
            6 => {
                if &others[0].value.unwrap() < &others[1].value.unwrap() {
                    1
                } else {
                    0
                }
            }
            // Eq
            7 => {
                if &others[0].value.unwrap() == &others[1].value.unwrap() {
                    1
                } else {
                    0
                }
            }
            _ => 0,
        }
    }
}
struct Literal {
    value: usize,
    bits_read: usize,
}

#[derive(Debug)]
struct PacketSequence {
    it: IntoIter<String>,
}

impl PacketSequence {
    /// Return a mapping of hexadecimal characters to their base 2 encoding as strings.
    fn _hex() -> HashMap<String, String> {
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

    /// Parse an integer from a vector of bits.
    fn _parse_int(bytes: Vec<String>) -> usize {
        let bytestr = bytes.join("");
        // println!("Parsed bytes: {}", bytestr);
        usize::from_str_radix(bytestr.as_str(), 2).expect("Failed to parse bytes as int.")
    }

    /// Take a single integer of size `take` bytes from the iterator of bits.
    fn _take_int(&mut self, take: usize) -> usize {
        let parts = self.it.by_ref().take(take).collect::<Vec<String>>();
        PacketSequence::_parse_int(parts)
    }

    /// Take a literal value with 5 bit encoding from the iterator of unknown total size.
    fn _take_literal(&mut self) -> Literal {
        let mut bits_read = 0;
        let mut has_more_to_read = true;
        let mut target_bits = Vec::new();
        while has_more_to_read {
            has_more_to_read = self._take_int(1) == 1;
            target_bits.extend(self.it.by_ref().take(4).collect::<Vec<String>>());
            bits_read += 5;
        }
        Literal {
            value: PacketSequence::_parse_int(target_bits),
            bits_read: bits_read,
        }
    }

    /// Take a packet out of the PacketSequence.
    fn _take_packet(&mut self) -> Packet {
        let mut bits_read = 0;
        let version = self._take_int(3);
        let id = self._take_int(3);
        bits_read += 6;
        match id {
            4 => {
                let lit = self._take_literal();
                bits_read += lit.bits_read;
                Packet {
                    id: id,
                    version: version,
                    mode: None,
                    sub_packet_size: None,
                    value: Some(lit.value),
                    bits_read: bits_read,
                }
            }
            _ => {
                let ptype = self._take_int(1);
                let to_read = if ptype == 0 { 15 } else { 11 };
                let size = self._take_int(to_read);
                bits_read += to_read + 1;
                Packet {
                    id: id,
                    version: version,
                    mode: Some(ptype),
                    sub_packet_size: Some(size),
                    value: None,
                    bits_read: bits_read,
                }
            }
        }
    }

    /// Take all of the packets that a mode 0 packet contains.
    fn _take_mode_0_packets(&mut self, parent: &Packet) -> (usize, Vec<Packet>) {
        let size = parent.sub_packet_size.unwrap();
        let mut to_read = size;
        let mut packets = Vec::new();
        let mut comp_packets = Vec::new();
        while to_read > 0 {
            let p = self._take_packet();
            let mode = p.mode;
            to_read -= p.bits_read;
            packets.push(p.clone());
            match mode {
                Some(0) => {
                    let (value, sub_pack) = self._take_mode_0_packets(&p);
                    to_read -= sub_pack.iter().map(|p| p.bits_read).sum::<usize>();
                    packets.extend(sub_pack);
                    // Create a fake top level packet for comparison with parent
                    comp_packets.push(Packet {
                        id: 999,
                        value: Some(value),
                        version: 999,
                        mode: None,
                        sub_packet_size: None,
                        bits_read: 0,
                    });
                }
                Some(1) => {
                    let (value, sub_pack) = self._take_mode_1_packets(&p);
                    to_read -= sub_pack.iter().map(|p| p.bits_read).sum::<usize>();
                    packets.extend(sub_pack);
                    // Create a fake top level packet for comparison with parent
                    comp_packets.push(Packet {
                        id: 999,
                        value: Some(value),
                        version: 999,
                        mode: None,
                        sub_packet_size: None,
                        bits_read: 0,
                    });
                }
                _ => comp_packets.push(p),
            }
        }
        (parent.comp(&comp_packets), packets)
    }

    /// Take all of the packets that a mode 1 packet contains.
    fn _take_mode_1_packets(&mut self, parent: &Packet) -> (usize, Vec<Packet>) {
        let size = parent.sub_packet_size.unwrap();
        let mut packets = Vec::new();
        let mut comp_packets = Vec::new();
        for _ in 0..size {
            let p = self._take_packet();
            let mode = p.mode;
            packets.push(p.clone());
            // If this packet contains operator packets, then parse these sub-packets as well
            match mode {
                Some(0) => {
                    let (value, sub_pack) = self._take_mode_0_packets(&p);
                    packets.extend(sub_pack);
                    // Create a fake top level packet for comparison with parent
                    comp_packets.push(Packet {
                        id: 999,
                        value: Some(value),
                        version: 999,
                        mode: None,
                        sub_packet_size: None,
                        bits_read: 0,
                    });
                }
                Some(1) => {
                    let (value, sub_pack) = self._take_mode_1_packets(&p);
                    packets.extend(sub_pack);
                    // Create a fake top level packet for comparison with parent
                    comp_packets.push(Packet {
                        id: 999,
                        value: Some(value),
                        version: 999,
                        mode: None,
                        sub_packet_size: None,
                        bits_read: 0,
                    });
                }
                _ => {
                    println!("Mode 1 containing literal");
                    comp_packets.push(p)
                }
            }
        }
        println!("Mode 1 comp len: {}", comp_packets.len());
        (parent.comp(&comp_packets), packets)
    }

    /// Parse all of the packets that are contained in a hex encoded string.
    fn new(hex: String) -> PacketSequence {
        let hex_mapping = PacketSequence::_hex();
        let bits = hex
            .split("")
            .filter(|s| s != &"")
            .flat_map(|c| hex_mapping.get(c).unwrap().split(""))
            // .map(|b| if b == "0" {false} else {true})
            .filter(|s| s != &"")
            .map(|c| c.to_string())
            .collect::<Vec<String>>();
        PacketSequence {
            it: bits.into_iter(),
        }
    }

    /// Take all packets out of the PacketSequence and evaluate their total value.
    fn evaluate(&mut self) -> usize {
        let parent = self._take_packet();
        match parent.mode {
            Some(0) => {
                let (val, _) = self._take_mode_0_packets(&parent);
                val
            }
            Some(1) => {
                let (val, _) = self._take_mode_1_packets(&parent);
                val
            }
            _ => parent.value.unwrap(),
        }
    }
}
/// Parse a packet of binary into hex, using an unnecessarily complex encoding scheme.
/// # Arguments
///
/// * `input_path` - The input file path containing the packets to parse.
///
/// # Returns
///
/// The evaluated packet data.
fn solution(input_path: &str) -> Vec<usize> {
    get_buf_reader(input_path)
        .lines()
        .map(|line| {
            let line = line.expect("Failed to parse line from file.");
            println!("----------------");
            println!("Starting hex: {}", line);
            let mut seq = PacketSequence::new(line);
            seq.evaluate()
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
    println!("Evaluated packets: {:?}", sol);
}

#[cfg(test)]
mod test_solution {
    use crate::solution;

    #[test]
    fn example_correct() {
        assert_eq!(
            solution("inputs/example.txt"),
            vec![2021, 1, 3, 15, 46, 46, 54],
        );
    }

    #[test]
    fn question_correct() {
        assert_eq!(solution("inputs/challenge.txt"), vec![19348959966392]);
    }
}
