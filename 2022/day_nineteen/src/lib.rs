struct Blueprint {
    number: usize,

}

impl Blueprint {
    fn new(input: &str) -> Self {
        let mut line = input.strip_prefix("Blueprint ").unwrap();
        let (number_str, line) = line.split_once(": ").unwrap();
        let number = number_str.parse().unwrap();

    }

    fn quality_level(&self) -> usize {
        0
    }
}

struct BlueprintSet {
    blueprints: Vec<Blueprint>
}

impl BlueprintSet {
    fn new(input: &str) -> Self {
        Self { blueprints: input.lines().map(|line| Blueprint::new(line)).collect() }
    }

    fn total_quality_level(&self) -> usize {
        self.blueprints.iter().map(Blueprint::quality_level).sum()
    }
}

#[cfg(test)]
mod tests {
    use std::fs;
    use super::*;

    #[test]
    fn part_one_example() {
        assert_eq!(
            BlueprintSet::new(fs::read_to_string("example_input.txt").unwrap().as_str()).total_quality_level(),
            33
        );
    }
}
