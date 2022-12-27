fn parse_snafu(input: &str) -> i32 {
    let mut total : i32 = 0;
    for (power_of_five, character) in input.chars().rev().enumerate() {
        let multiplier = match character {
            '2' => 2,
            '1' => 1,
            '0' => 0,
            '-' => -1,
            '=' => -2,
            _ => panic!("Unexpected character: {character}")
        };
        total += (5 as i32).pow(power_of_five as u32) as i32 * multiplier;
    }

    total
}

fn decimal_to_snafu(number: i32) -> String {
    if number < 0 {
        panic!("Negative numbers not supported")
    }
    let mut digits : Vec<char> = vec![];
    let mut power_of_five = 0;
    let mut multiplier = 0;
    while number > 0 {
        if number < 3 {
            digits.push(('0' as u8 + number as u8) as char);
            break;
        }

        todo!();
    }
    String::from("lol")
}

#[cfg(test)]
mod tests {
    use std::fs;
    use super::*;

    #[test]
    fn part_one_example() {
        let values : Vec<(String, i32)> = fs::read_to_string("example_input.txt").unwrap().lines().map(|snafu| (snafu.to_string(), parse_snafu(snafu))).collect();
        for (snafu, value) in values.iter() {
            println!("{:>6}, {:4}", snafu, *value);
        }
        let total : i32 = values.iter().map(|(_snafu, value)| *value).sum();
        assert_eq!(4890, total);
        let snafu_total = decimal_to_snafu(total);
        assert_eq!("2=-1=0", snafu_total);
    }
}
