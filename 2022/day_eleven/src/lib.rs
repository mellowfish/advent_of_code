use std::borrow::BorrowMut;
use num_bigint::BigUint;
use num_bigint::ToBigUint;
use num_traits::Zero;

type Worry = BigUint;

struct Toss {
    item: Worry,
    target: usize
}

struct Troop {
    monkeys: Vec<Monkey>
}

impl Troop {
    fn new(input: &str, worry_reducer: u8) -> Self {
        Self { monkeys: input.split("\n\n").map(|note| Monkey::new(note, worry_reducer)).collect() }
    }

    fn run_rounds(&mut self, rounds: usize) {
        for round in 0..rounds {
            self.run_round(round);
        }
    }

    fn run_round(&mut self, _round: usize) {
        for index in 0..self.monkeys.len() {
            let monkey = self.monkeys[index].borrow_mut();
            for toss in monkey.run_round() {
                self.monkeys[toss.target].catch(toss.item);
            }
        }
    }

    fn monkey_business(&self) -> Worry {
        let mut inspections : Vec<Worry> = self.monkeys.iter().map(|monkey| monkey.inspections.clone()).collect();

        inspections.sort();
        inspections.reverse();

        inspections[0].clone() * inspections[1].clone()
    }
}

enum Operation {
    Add(u8),
    Multiply(u8),
    Square
}

impl Operation {
    fn new(input: &str) -> Self {
        let parts : Vec<&str> = input.split("new = old ").last().unwrap().split(" ").collect();
        if parts[1] == "old" {
            match parts[0] {
                "*" => { return Operation::Square },
                _ => panic!("Unknown operation: {input}")
            }
        }
        let number = parts[1].parse::<u8>().unwrap();

        match parts[0] {
            "*" => Operation::Multiply(number),
            "+" => Operation::Add(number),
            _ => panic!("Unknown operation: {input}")
        }
    }
}

struct Monkey {
    items: Vec<Worry>,
    inspections: Worry,
    operation: Operation,
    divisor: u8,
    worry_reducer: u8,
    next_monkeys: (usize, usize)
}

impl Monkey {
    fn new(input: &str, worry_reducer: u8) -> Self {
        let mut all_lines = input.lines().map(str::trim);
        all_lines.next(); // Ignore initial "monkey N:"
        let lines : Vec<&str> = all_lines.map(|line| line.split(": ").last().unwrap()).collect();
        let items = lines[0].split(", ").map(|item| item.parse::<Worry>().unwrap()).collect();
        let operation= Operation::new(lines[1]);
        let divisor = lines[2].split("divisible by ").last().unwrap().parse::<u8>().unwrap();
        let monkey_one = lines[3].split(" monkey ").last().unwrap().parse::<usize>().unwrap();
        let monkey_two = lines[4].split(" monkey ").last().unwrap().parse::<usize>().unwrap();

        Self { inspections: Zero::zero(), worry_reducer, items, operation, divisor, next_monkeys: (monkey_one, monkey_two) }
    }

    fn run_round(&mut self) -> Vec<Toss> {
        let tosses : Vec<Toss> = self.items.iter().map(|item| self.toss_for_item(item.clone())).collect();
        self.inspections += tosses.len();

        self.items = vec![];

        tosses
    }

    fn toss_for_item(&self, original_item: Worry) -> Toss {
        let mut item = self.increase_worry(original_item);
        if self.worry_reducer > 0 {
            item /= self.worry_reducer.to_biguint().unwrap();
        }

        let target = self.target_for(item.clone());

        Toss { item, target }
    }

    fn increase_worry(&self, item: Worry) -> Worry {
        let item_copy = item.clone();
        match self.operation {
            Operation::Square => { item * item_copy },
            Operation::Add(addend) => { item + addend },
            Operation::Multiply(multiplier) => { item * multiplier }
        }
    }

    fn target_for(&self, item: Worry) -> usize {
        if (item % self.divisor) == Zero::zero() {
            self.next_monkeys.0
        } else {
            self.next_monkeys.1
        }
    }

    fn catch(&mut self, item: Worry) {
        self.items.push(item);
    }
}

#[cfg(test)]
mod tests {
    use std::fs;
    use super::*;

    #[test]
    fn part_one_example() {
        let mut troop = Troop::new(fs::read_to_string("example_input.txt").unwrap().as_str(), 3);
        troop.run_rounds(20);
        assert_eq!(
            troop.monkey_business(),
            10605.to_biguint().unwrap()
        );
    }

    #[test]
    fn part_one() {
        let mut troop = Troop::new(fs::read_to_string("input.txt").unwrap().as_str(), 3);
        troop.run_rounds(20);
        assert_eq!(
            troop.monkey_business(),
            55930.to_biguint().unwrap()
        );
    }

    #[test]
    fn part_two_example() {
        let mut troop = Troop::new(fs::read_to_string("example_input.txt").unwrap().as_str(), 0);
        troop.run_rounds(20);
        assert_eq!(troop.monkey_business(), 10197.to_biguint().unwrap());
        troop.run_rounds(980);
        assert_eq!(troop.monkey_business(), (5204 * 5192).to_biguint().unwrap());
    }
}
