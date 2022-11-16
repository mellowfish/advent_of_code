use md5;

fn find_suffix_for(input: &str, prefix: &str) -> u32 {
    let mut counter : u32 = 1;
    let mut coin_cleartext: String;
    let mut coin_hex : String;

    loop {
        coin_cleartext = format!("{input}{counter}");
        coin_hex = format!("{:x?}", md5::compute(coin_cleartext.as_bytes()));
        if coin_hex.starts_with(prefix) {
            return counter;
        }
        counter += 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_one_examples() {
        assert_eq!(find_suffix_for("abcdef", "00000"), 609043);
        assert_eq!(find_suffix_for("pqrstuv", "00000"), 1048970);
    }

    #[test]
    fn part_one() {
        assert_eq!(find_suffix_for("ckczppom", "00000"), 117946);
    }

    #[test]
    fn part_two() {
        assert_eq!(find_suffix_for("ckczppom", "000000"), 3938038);
    }
}
