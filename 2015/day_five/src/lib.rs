fn is_vowel(character: char) -> bool {
    match character {
        'a' | 'e' | 'i' | 'o' | 'u' => true,
        _ => false
    }
}

fn part_one_is_nice(input: &str) -> bool {
    let mut vowel_count = 0;
    let mut has_double_letter = false;
    let mut no_banned_pairings = true;
    let mut previous_character : char = '!'; // initial value not used

    for (index, character) in input.chars().enumerate() {
        if is_vowel(character) {
            vowel_count += 1;
        }
        if index > 0 {
            match (previous_character, character) {
                (x, y) if x == y => {
                    has_double_letter = true
                },
                ('a', 'b') | ('c', 'd') | ('p', 'q') | ('x', 'y') => {
                    no_banned_pairings = false
                },
                _ => ()
            }
        }
        previous_character = character;
    }

    vowel_count > 2 && has_double_letter && no_banned_pairings
}

fn pair_of_pairs_exists(characters: &Vec<char>) -> bool {
    for start_pair_index in 0..=(characters.len() - 4) {
        let pair = (characters[start_pair_index], characters[start_pair_index + 1]);

        for start_search_index in (start_pair_index + 2)..=(characters.len() - 2) {
            let possible_pair =
                (characters[start_search_index], characters[start_search_index + 1]);

            if pair == possible_pair {
                return true;
            }
        }
    }

    false
}

fn split_pair_exists(characters: &Vec<char>) -> bool {
    for start_pair_index in 0..=(characters.len() - 3) {
        if characters[start_pair_index] == characters[start_pair_index + 2] &&
            characters[start_pair_index] != characters[start_pair_index + 1] {
            return true;
        }
    }

    false
}

fn part_two_is_nice(input: &str) -> bool {
    let characters : Vec<char> = input.chars().collect();

    pair_of_pairs_exists(&characters) && split_pair_exists(&characters)
}

#[cfg(test)]
mod tests {
    use std::fs;
    use super::*;

    fn input() -> String {
        fs::read_to_string("input.txt").unwrap()
    }

    #[test]
    fn part_one_examples() {
        assert!(part_one_is_nice("ugknbfddgicrmopn"));
        assert!(part_one_is_nice("aaa"));
        assert!(!part_one_is_nice("jchzalrnumimnmhp"));
        assert!(!part_one_is_nice("haegwjzuvuyypxyu"));
        assert!(!part_one_is_nice("dvszwmarrgswjxmb"));
    }

    #[test]
    fn part_one() {
        assert_eq!(
            input().lines().filter(|line| !line.is_empty() && part_one_is_nice(line)).count(),
            255
        )
    }

    #[test]
    fn part_two_examples() {
        assert!(part_two_is_nice("qjhvhtzxzqqjkmpb"));
        assert!(part_two_is_nice("xxyxx"));
        assert!(!part_two_is_nice("uurcxstgmygtbstg"));
        assert!(!part_two_is_nice("ieodomkazucvgmuy"));
    }

    #[test]
    fn part_two() {
        assert_eq!(
            input().lines().filter(|line| !line.is_empty() && part_two_is_nice(line)).count(),
            52 // this is wrong
        )
    }
}
