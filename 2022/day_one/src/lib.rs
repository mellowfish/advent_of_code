use std::cmp::Ordering;

type Snack = u32;

struct Expedition {
    elves: Vec<Elf>
}

impl Expedition {
    fn new(input: &str) -> Self {
        let mut elves = vec![];
        let mut elf = Elf::new();

        for line in input.lines() {
            if line.is_empty() {
                let previous_elf = elf;
                elf = Elf::new();
                elves.push(previous_elf)
            }
            else {
                elf.add_snack(line.parse::<Snack>().unwrap())
            }
        }

        if !elf.is_empty() {
            elves.push(elf)
        }

        elves.sort();

        Self { elves }
    }

    fn top_elf(&self) -> &Elf {
        self.elves.first().unwrap()
    }

    fn top_elves(&self, count: usize) -> Self {
        Self { elves: self.elves.as_slice()[0..count].to_vec() }
    }

    fn total_calories(&self) -> Snack {
        self.elves.iter().map(Elf::total_calories).sum()
    }
}

#[derive(Clone)]
struct Elf {
    snacks: Vec<Snack>
}

impl Elf {
    fn new() -> Self {
        Self { snacks: vec![] }
    }

    fn add_snack(&mut self, snack: Snack) {
        self.snacks.push(snack)
    }

    fn is_empty(&self) -> bool {
        self.snacks.is_empty()
    }

    fn total_calories(&self) -> Snack {
        self.snacks.iter().sum()
    }
}

impl Eq for Elf {}

impl PartialEq<Self> for Elf {
    fn eq(&self, other: &Self) -> bool {
        self.total_calories() == other.total_calories()
    }
}

impl PartialOrd<Self> for Elf {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(other.cmp(self))
    }
}

impl Ord for Elf {
    fn cmp(&self, other: &Self) -> Ordering {
        self.total_calories().cmp(&other.total_calories())
    }
}

#[cfg(test)]
mod tests {
    use std::fs;
    use crate::*;

    fn example_input() -> &'static str {
        "1000
2000
3000

4000

5000
6000

7000
8000
9000

10000"
    }

    fn full_input() -> String {
        fs::read_to_string("input.txt").unwrap()
    }

    #[test]
    fn part_one_example() {
        assert_eq!(Expedition::new(example_input()).top_elf().total_calories(), 24_000);
    }

    #[test]
    fn part_one() {
        assert_eq!(Expedition::new(full_input().as_str()).top_elf().total_calories(), 68_775);
    }

    #[test]
    fn part_two_example() {
        assert_eq!(Expedition::new(example_input()).top_elves(3).total_calories(), 45_000);
    }

    #[test]
    fn part_two() {
        assert_eq!(Expedition::new(full_input().as_str()).top_elves(3).total_calories(), 202_585);
    }
}
