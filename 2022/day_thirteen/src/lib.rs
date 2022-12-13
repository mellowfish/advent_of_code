use std::borrow::Borrow;
use std::cmp::Ordering;

#[derive(Clone, Eq, PartialEq)]
enum PacketPart {
    Number(u8),
    List(Vec<PacketPart>)
}

impl PacketPart {
    fn new(input: &str) -> Self {
        let mut chars : Vec<char> = input.chars().rev().collect();

        Self::parse_list(chars.as_mut())
    }

    fn number_from_digit(digit: char) -> u8 {
        digit as u8 - '0' as u8
    }

    fn parse_single_digit_number(digit: char) -> Self {
        Self::Number(Self::number_from_digit(digit))
    }

    fn parse_double_digit_number(first: char, second: char) -> Self {
        Self::Number(Self::number_from_digit(first) * 10 + Self::number_from_digit(second))
    }

    fn parse_list(chars: &mut Vec<char>) -> Self {
        // dbg!(&chars);

        let mut list : Vec<Self> = vec![];
        let mut current = chars.pop().unwrap();

        if current != '[' {
            panic!("Expected '[' got: '{current}'")
        }

        while !chars.is_empty() {
            current = *chars.last().unwrap();
            match current {
                '0'..='9' => {
                    let first_digit = chars.pop().unwrap();
                    current = *chars.last().unwrap();
                    match current {
                        '0'..='9' => {
                            list.push(Self::parse_double_digit_number(first_digit, chars.pop().unwrap()))
                        },
                        ',' | ']' => {
                            list.push(Self::parse_single_digit_number(first_digit))
                        },
                        _ => panic!("Unexpected char: {current}")
                    }

                }, // simple number
                '[' => { list.push(Self::parse_list(chars)) }, // start of nested list
                ']' => {
                    chars.pop();
                    return Self::List(list)
                }, // current list is blank
                _ => panic!("Unexpected char: {current}")
            }
            current = chars.pop().unwrap();
            match current {
                ',' => {}, // another item in list
                ']' => { return Self::List(list) }, // end of current list
                _ => panic!("unexpected char: {current}")
            }
        }

        panic!("Unexpected end of input!")
    }
}

struct PacketPair {
    left: Vec<PacketPart>,
    right: Vec<PacketPart>
}

impl PacketPair {
    fn new(input: &str) -> Self {
        let mut packets : Vec<PacketPart> = input.lines().map(|line| PacketPart::new(line)).collect();
        if let PacketPart::List(right) = packets.pop().unwrap() {
            if let PacketPart::List(left) = packets.pop().unwrap() {
                return Self { left, right }
            }
        }
        panic!("Failed to parse input:\n{input}")
    }

    fn to_ord(&self) -> Ordering {
        if self.is_in_order() {
            Ordering::Less
        } else {
            Ordering::Greater
        }
    }

    fn is_in_order(&self) -> bool {
        if let Some(success) = Self::lists_are_in_order(&self.left, &self.right) {
            success
        } else {
            panic!("Couldn't decide!")
        }
    }

    fn lists_are_in_order(left: &Vec<PacketPart>, right: &Vec<PacketPart>) -> Option<bool> {
        let mut index = 0;

        while index < left.len() && index < right.len() {
            match left[index].borrow() {
                PacketPart::Number(left_item) => {
                    match right[index].borrow() {
                        PacketPart::Number(right_item) => {
                            if left_item < right_item {
                                return Some(true);
                            }
                            if right_item < left_item {
                                return Some(false);
                            }
                        },
                        PacketPart::List(right_items) => {
                            let left_items : Vec<PacketPart> = vec![PacketPart::Number(*left_item)];
                            match Self::lists_are_in_order(&left_items, right_items) {
                                Some(success) => { return Some(success) },
                                None => {}
                            }
                        }
                    }
                },
                PacketPart::List(left_items) => {
                    match right[index].borrow() {
                        PacketPart::Number(right_item) => {
                            let right_items : Vec<PacketPart> = vec![PacketPart::Number(*right_item)];
                            match Self::lists_are_in_order(left_items, &right_items) {
                                Some(success) => { return Some(success) },
                                None => {}
                            }
                        },
                        PacketPart::List(right_items) => {
                            match Self::lists_are_in_order(left_items, right_items) {
                                Some(success) => { return Some(success) },
                                None => {}
                            }
                        }
                    }
                }
            }

            index += 1;
        }

        if right.len() < left.len() {
            Some(false)
        } else if left.len() < right.len() {
            Some(true)
        } else {
            None
        }
    }
}

struct DistressSignalDiagnostic {
    packet_pairs: Vec<PacketPair>
}

impl DistressSignalDiagnostic {
    fn new(input: &str) -> Self {
        Self { packet_pairs: input.split("\n\n").map(|pair_input| PacketPair::new(pair_input)).collect() }
    }

    fn diagnostic_code(&self) -> usize {
        self.packet_pairs.iter().enumerate().filter(|(_index, pair)| pair.is_in_order()).map(|(index, _)| index + 1).sum()
    }
}

struct DistressSignal {
    packets: Vec<PacketPart>
}

impl DistressSignal {
    fn new(input: &str) -> Self {
        let mut packets : Vec<PacketPart> = input.lines().filter(|line| !line.is_empty()).map(|line| PacketPart::new(line)).collect();

        packets.push(PacketPart::new("[[2]]"));
        packets.push(PacketPart::new("[[6]]"));
        packets.sort_by(|left_part, right_part| {
            match left_part {
                PacketPart::List(left_items) => {
                    match right_part {
                        PacketPart::List(right_items) => {
                            let left : Vec<PacketPart> = left_items.to_vec();
                            let right : Vec<PacketPart> = right_items.to_vec();

                            PacketPair { left, right }.to_ord()
                        },
                        _ => panic!()
                    }
                },
                _ => panic!()
            }
        });

        Self { packets }
    }

    fn decoder_key(&self) -> usize {
        self.position_of_packet("[[2]]") * self.position_of_packet("[[6]]")
    }

    fn position_of_packet(&self, input: &str) -> usize {
        self.packets.iter().position(|packet| packet.eq(&PacketPart::new(input))).unwrap() + 1
    }
}

#[cfg(test)]
mod tests {
    use std::fs;
    use super::*;

    #[test]
    fn part_one_example() {
        let signal = DistressSignalDiagnostic::new(fs::read_to_string("example_input.txt").unwrap().as_str());
        assert_eq!(signal.diagnostic_code(), 13);
    }

    #[test]
    fn part_one() {
        let signal = DistressSignalDiagnostic::new(fs::read_to_string("input.txt").unwrap().as_str());
        assert_eq!(signal.diagnostic_code(), 6187);
    }

    #[test]
    fn part_two_example() {
        let signal = DistressSignal::new(fs::read_to_string("example_input.txt").unwrap().as_str());
        assert_eq!(signal.decoder_key(), 140);
    }

    #[test]
    fn part_two() {
        let signal = DistressSignal::new(fs::read_to_string("input.txt").unwrap().as_str());
        assert_eq!(signal.decoder_key(), 23520);
    }
}
