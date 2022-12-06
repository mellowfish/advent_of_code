use std::collections::HashSet;

struct DataStream {
    bytes: Vec<char>
}

impl DataStream {
    fn new(input: &str) -> Self {
        Self { bytes: input.chars().collect() }
    }

    fn first_packet_byte_number(&self) -> usize {
        self.byte_number_for_run_of_unique_bytes(4)
    }

    fn first_message_byte_number(&self) -> usize {
        self.byte_number_for_run_of_unique_bytes(14)
    }

    fn byte_number_for_run_of_unique_bytes(&self, count: usize) -> usize {
        let offset = count - 1;
        for index in offset..self.bytes.len() {
            let unique_characters : HashSet<&char> = HashSet::from_iter(self.bytes[(index - offset)..=index].iter());
            if unique_characters.len() == count {
                return index + 1
            }
        }

        panic!("No packet start detected!");
    }
}

#[cfg(test)]
mod tests {
    use std::fs;
    use super::*;

    #[test]
    fn part_one_examples() {
        assert_eq!(DataStream::new("mjqjpqmgbljsphdztnvjfqwrcgsmlb").first_packet_byte_number(), 7);
        assert_eq!(DataStream::new("bvwbjplbgvbhsrlpgdmjqwftvncz").first_packet_byte_number(), 5);
        assert_eq!(DataStream::new("nppdvjthqldpwncqszvftbrmjlhg").first_packet_byte_number(), 6);
        assert_eq!(DataStream::new("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg").first_packet_byte_number(), 10);
        assert_eq!(DataStream::new("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw").first_packet_byte_number(), 11);
    }

    #[test]
    fn part_one() {
        assert_eq!(
            DataStream::new(fs::read_to_string("input.txt").unwrap().as_str()).first_packet_byte_number(),
            1034
        )
    }

    #[test]
    fn part_two_examples() {
        assert_eq!(DataStream::new("mjqjpqmgbljsphdztnvjfqwrcgsmlb").first_message_byte_number(), 19);
        assert_eq!(DataStream::new("bvwbjplbgvbhsrlpgdmjqwftvncz").first_message_byte_number(), 23);
        assert_eq!(DataStream::new("nppdvjthqldpwncqszvftbrmjlhg").first_message_byte_number(), 23);
        assert_eq!(DataStream::new("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg").first_message_byte_number(), 29);
        assert_eq!(DataStream::new("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw").first_message_byte_number(), 26);
    }

    #[test]
    fn part_two() {
        assert_eq!(
            DataStream::new(fs::read_to_string("input.txt").unwrap().as_str()).first_message_byte_number(),
            2472
        )
    }
}
