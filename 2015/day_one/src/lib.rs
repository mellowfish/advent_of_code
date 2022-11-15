fn process_full_instructions(input: &str) -> i32 {
    let mut floor = 0;

    for char in input.chars() {
        match char {
            '(' => floor += 1,
            ')' => floor -= 1,
            '\n' => {},
            x => panic!("Unexpected char: {x}")
        }
    }

    floor
}

fn find_position_of_first_basement_move(input: &str) -> i32 {
    let mut floor = 0;

    for (index, char ) in input.chars().enumerate() {
        match char {
            '(' => floor += 1,
            ')' => {
                floor -= 1;
                if floor == -1 {
                    return (index + 1) as i32;
                }
            },
            '\n' => {},
            x => panic!("Unexpected char: {x}")
        }
    }

    panic!("Never entered basement!")
}

#[cfg(test)]
mod tests {
    use std::fs;
    use crate::*;

    #[test]
    fn matched_pairs_cancel_to_zero() {
        assert_eq!(process_full_instructions("(())"), 0);
        assert_eq!(process_full_instructions("()()"), 0);
    }

    #[test]
    fn threes() {
        assert_eq!(process_full_instructions("((("), 3);
        assert_eq!(process_full_instructions("(()(()("), 3);
        assert_eq!(process_full_instructions("))((((("), 3);
    }

    #[test]
    fn negatives() {
        assert_eq!(process_full_instructions("())"), -1);
        assert_eq!(process_full_instructions("))("), -1);

        assert_eq!(process_full_instructions(")))"), -3);
        assert_eq!(process_full_instructions(")())())"), -3);
    }

    #[test]
    fn part_one() {
        let input = fs::read_to_string("input.txt").unwrap();
        assert_eq!(process_full_instructions(&input), 138);
    }

    #[test]
    fn part_two_examples() {
        assert_eq!(find_position_of_first_basement_move(")"), 1);
        assert_eq!(find_position_of_first_basement_move("()())"), 5);
    }

    #[test]
    fn part_two() {
        let input = fs::read_to_string("input.txt").unwrap();
        assert_eq!(find_position_of_first_basement_move(&input), 1771);
    }
}
