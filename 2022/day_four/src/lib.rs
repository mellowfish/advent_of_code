use std::ops::Range;

fn parse_number_range(input: &str) -> Range<u32> {
    let mut numbers = input.split("-").map(|number| number.parse::<u32>().unwrap());

    Range { start: numbers.next().unwrap(), end: numbers.next().unwrap() }
}

struct PairAssignment {
    left: Range<u32>,
    right: Range<u32>
}

impl PairAssignment {
    fn new(input: &str) -> Self {
        let mut ranges = input.split(",").map(parse_number_range);

        Self { left: ranges.next().unwrap(), right: ranges.next().unwrap() }
    }

    fn is_at_least_partial_overlap(&self) -> bool {
        self.left_crosses_into_right() || self.right_crosses_into_left()
    }

    fn is_total_overlap(&self) -> bool {
        self.left_contains_right() || self.right_contains_left()
    }

    fn left_contains_right(&self) -> bool {
        self.left.start <= self.right.start && self.right.end <= self.left.end
    }

    fn right_contains_left(&self) -> bool {
        self.right.start <= self.left.start && self.left.end <= self.right.end
    }

    fn left_crosses_into_right(&self) -> bool {
        self.right.start <= self.left.end && self.left.end <= self.right.end
    }

    fn right_crosses_into_left(&self) -> bool {
        self.left.start <= self.right.end && self.right.end <= self.left.end
    }
}

struct Schedule {
    assigned_pairs: Vec<PairAssignment>
}

impl Schedule {
    fn new(input: &str) -> Self {
        Self { assigned_pairs: input.lines().map(PairAssignment::new).collect() }
    }

    fn count_total_overlaps(&self) -> usize {
        self.assigned_pairs.iter().filter(|pair| pair.is_total_overlap()).count()
    }

    fn count_partial_overlaps(&self) -> usize {
        self.assigned_pairs.iter().filter(|pair| pair.is_at_least_partial_overlap()).count()
    }
}

#[cfg(test)]
mod tests {
    use std::fs;
    use super::*;

    fn read_input(file_name: &str) -> String {
        fs::read_to_string(file_name).expect("File not found.")
    }

    #[test]
    fn part_one_example() {
        assert_eq!(
            Schedule::new(read_input("example_input.txt").as_str()).count_total_overlaps(),
            2
        );
    }

    #[test]
    fn part_one() {
        assert_eq!(
            Schedule::new(read_input("input.txt").as_str()).count_total_overlaps(),
            599
        );
    }

    #[test]
    fn part_two_example() {
        assert_eq!(
            Schedule::new(read_input("example_input.txt").as_str()).count_partial_overlaps(),
            4
        );
    }

    #[test]
    fn part_two() {
        assert_eq!(
            Schedule::new(read_input("input.txt").as_str()).count_partial_overlaps(),
            928
        );
    }
}
