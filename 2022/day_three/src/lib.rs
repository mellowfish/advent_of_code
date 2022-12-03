use std::collections::HashSet;
use std::iter::Iterator;

const ALPHABET : &str = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";

fn union<'a>(left: &'a HashSet<char>, right: &'a HashSet<char>) -> HashSet<char> {
    let mut result : HashSet<char> = HashSet::new();

    for letter in left {
        result.insert(*letter);
    }

    for letter in right {
        result.insert(*letter);
    }

    result
}

fn intersection<'a>(left: &'a HashSet<char>, right: &'a HashSet<char>) -> HashSet<char> {
    let mut collector : HashSet<char> = HashSet::new();

    for letter in left.intersection(&right) {
        collector.insert(*letter);
    }

    collector
}

fn value_for_letter(target_letter: char) -> u32 {
    ALPHABET.chars().position(|letter| letter == target_letter).unwrap() as u32 + 1
}

struct ElfGroup {
    rucksacks: Vec<Rucksack>
}

impl ElfGroup {
    fn new() -> Self { Self { rucksacks: vec![] } }

    fn form_groups(rucksacks: Vec<Rucksack>) -> Vec<Self> {
        let mut groups : Vec<Self> = vec![];
        let mut group = Self::new();

        for (index, rucksack) in rucksacks.iter().enumerate() {
            group.push_rucksack(rucksack.clone());

            if index % 3 == 2 {
                let old_group = group;
                group = Self::new();
                groups.push(old_group);
            }
        }

        groups
    }

    fn push_rucksack(&mut self, rucksack: Rucksack) {
        if self.rucksacks.len() == 3 {
            panic!("Trying to form a group of 4!")
        }

        self.rucksacks.push(rucksack);
    }

    fn badge(&self) -> char {
        *self.rucksacks
            .iter()
            .map(Rucksack::unique_letters)
            .reduce(|left, right| intersection(&left, &right))
            .unwrap()
            .iter()
            .next()
            .unwrap()
    }

    fn badge_value(&self) -> u32 {
        value_for_letter(self.badge())
    }
}

#[derive(Clone)]
struct Rucksack {
    front: HashSet<char>,
    back: HashSet<char>
}

impl Rucksack {
    fn new(input: &str) -> Self {
        let front : Vec<char> = input[0..(input.len() / 2)].chars().collect();
        let back : Vec<char> = input[(input.len() / 2)..input.len()].chars().collect();

        let new_rucksack = Self {
            front: HashSet::from_iter(front),
            back: HashSet::from_iter(back)
        };

        new_rucksack
    }

    fn mismatched_letter(&self) -> char {
        *self.front.intersection(&self.back).next().unwrap()
    }

    fn mismatched_letter_value(&self) -> u32 {
        value_for_letter(self.mismatched_letter())
    }

    fn unique_letters(&self) -> HashSet<char> {
        union(&self.front, &self.back)
    }
}

#[cfg(test)]
mod tests {
    use std::fs;
    use super::*;

    fn read_rucksacks(file_name: &str) -> Vec<Rucksack> {
        fs::read_to_string(file_name).unwrap().lines().map(Rucksack::new).collect()
    }

    #[test]
    fn part_one_examples() {
        let rucksacks : Vec<Rucksack> = read_rucksacks("example_input.txt");

        assert_eq!(
            String::from_iter(rucksacks.iter().map(Rucksack::mismatched_letter)),
            "pLPvts"
        );

        assert_eq!(rucksacks.iter().map(Rucksack::mismatched_letter_value).sum::<u32>(), 157);
    }

    #[test]
    fn part_one() {
        let rucksacks : Vec<Rucksack> = read_rucksacks("input.txt");
        assert_eq!(rucksacks.iter().map(Rucksack::mismatched_letter_value).sum::<u32>(), 7763);
    }

    #[test]
    fn part_two_examples() {
        let groups = ElfGroup::form_groups(read_rucksacks("example_input.txt"));

        assert_eq!(
            String::from_iter(groups.iter().map(ElfGroup::badge)),
            "rZ"
        );

        assert_eq!(groups.iter().map(ElfGroup::badge_value).sum::<u32>(), 70);
    }

    #[test]
    fn part_two() {
        let groups = ElfGroup::form_groups(read_rucksacks("input.txt"));

        assert_eq!(groups.iter().map(ElfGroup::badge_value).sum::<u32>(), 2569);
    }
}
