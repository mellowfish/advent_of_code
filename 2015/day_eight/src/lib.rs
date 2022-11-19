struct SleightaBase {
    list: Vec<ParsedString>
}

impl SleightaBase {
    fn new(input: String) -> Self {
        let mut list : Vec<ParsedString> = Vec::new();

        for line in input.lines().map(str::trim) {
            if line.is_empty() { continue };

            list.push(ParsedString::new(line.to_string()))
        }

        Self { list }
    }

    fn compression(&self) -> usize {
        self.list.iter().map(ParsedString::compression).sum()
    }
}

struct ParsedString {
    original: String,
    parsed: Vec<char>,
    encoded: Vec<char>,
}

impl ParsedString {
    fn new(original: String) -> Self {
        let last_index = original.len() - 1;
        let mut parsed : Vec<char> = Vec::new();
        let mut encoded : Vec<char> = Vec::new();
        todo!();
        let mut characters = original.chars().enumerate();
        loop {
            match characters.next() {
                Some((index, character)) => {
                    if index == 0 || index == last_index {
                        continue;
                    }
                    match character {
                        '\\' => {
                            let (_, second_character) = characters.next().unwrap();
                            match second_character {
                                '\\' | '"' => {
                                    parsed.push(second_character)
                                },
                                'x' => {
                                    let first_digit = characters.next().unwrap().1.to_digit(16).unwrap();
                                    let second_digit = characters.next().unwrap().1.to_digit(16).unwrap();
                                    parsed.push(char::from_u32(first_digit * 16 + second_digit).unwrap());
                                }
                                _ => panic!("Unknown escaped character: {second_character}")
                            }
                        },
                        single_character => { parsed.push(single_character) }
                    }
                },
                None => { break; }
            }
        }

        Self { original, parsed, encoded }
    }

    fn parsed_string(&self) -> String {
        String::from_iter(self.parsed.iter())
    }

    fn original_chars(&self) -> Vec<u32> {
        self.original.chars().map(|char| char as u32).collect()
    }

    fn parsed_chars(&self) -> Vec<u32> {
        self.parsed.iter().map(|&char| char as u32).collect()
    }

    fn compression(&self) -> usize {
        // println!("{} : {}", self.original, self.original.len());
        // println!("{} : {}", self.parsed_string(), self.parsed.len());
        // println!("Difference : {}", self.original.len() - self.parsed.len());
        self.original.len() - self.parsed.len()
    }
}

#[cfg(test)]
mod tests {
    use std::fs;
    use super::*;

    #[test]
    fn empty_string() {
        let example_one = ParsedString::new(String::from(r#""""#));
        assert_eq!(example_one.original_chars().len(), 2);
        assert_eq!(example_one.parsed_chars().len(), 0);
    }

    #[test]
    fn simple_string() {
        let example_one = ParsedString::new(String::from(r#""abc""#));
        assert_eq!(example_one.original_chars().len(), 5);
        assert_eq!(example_one.parsed_chars().len(), 3);
    }

    #[test]
    fn escaped_quote_string() {
        let example_one = ParsedString::new(String::from(r#""aaa\"aaa""#));
        assert_eq!(example_one.original_chars().len(), 10);
        assert_eq!(example_one.parsed_chars().len(), 7);
    }

    #[test]
    fn hex_encoded_string() {
        let example_one = ParsedString::new(String::from(r#""\x27""#));
        assert_eq!(example_one.original_chars().len(), 6);
        assert_eq!(example_one.parsed_chars().len(), 1);
    }

    #[test]
    fn hex_encoded_string_two() {
        let example_one = ParsedString::new(String::from(r#""\xa8""#));
        assert_eq!(example_one.original_chars().len(), 6);
        assert_eq!(example_one.parsed_chars().len(), 1);
    }

    #[test]
    fn sleighta_base() {
        let example_input = String::from(r#"
        ""
        "abc"
        "aaa\"aaa"
        "\x27"
        "#);

        assert_eq!(SleightaBase::new(example_input).compression(), 12);
    }

    #[test]
    fn part_one() {
        assert_eq!(
            SleightaBase::new(fs::read_to_string("input.txt").unwrap()).compression(),
            1371 // too low
        )
    }
}
