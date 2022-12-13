use std::borrow::BorrowMut;

type Worry = u64;

struct Toss {
    item: Worry,
    target: usize
}

struct Troop {
    monkeys: Vec<Monkey>,
    common_multiple: Worry
}

impl Troop {
    fn new(input: &str, worry_reducer: Worry) -> Self {
        let monkeys : Vec<Monkey> = input.split("\n\n").map(|note| Monkey::new(note, worry_reducer)).collect();
        let common_multiple = monkeys.iter().map(|monkey| monkey.divisor ).reduce(|multiple, divisor| multiple * divisor ).unwrap();

        Self { monkeys, common_multiple }
    }

    fn run_rounds(&mut self, rounds: usize) {
        for round in 0..rounds {
            self.run_round(round);
        }
    }

    fn run_round(&mut self, _round: usize) {
        for index in 0..self.monkeys.len() {
            let monkey = self.monkeys[index].borrow_mut();
            for toss in monkey.run_round(self.common_multiple) {
                self.monkeys[toss.target].catch(toss.item);
            }
        }
    }

    fn monkey_business(&self) -> usize {
        let mut inspections : Vec<usize> = self.monkeys.iter().map(|monkey| monkey.inspections).collect();

        inspections.sort();
        inspections.reverse();

        inspections[0] * inspections[1]
    }
}

enum Operation {
    Add(Worry),
    Multiply(Worry),
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
        let number = parts[1].parse::<Worry>().unwrap();

        match parts[0] {
            "*" => Operation::Multiply(number),
            "+" => Operation::Add(number),
            _ => panic!("Unknown operation: {input}")
        }
    }
}

struct Monkey {
    items: Vec<Worry>,
    inspections: usize,
    operation: Operation,
    divisor: Worry,
    worry_reducer: Worry,
    next_monkeys: (usize, usize)
}

impl Monkey {
    fn new(input: &str, worry_reducer: Worry) -> Self {
        let mut all_lines = input.lines().map(str::trim);
        all_lines.next(); // Ignore initial "monkey N:"
        let lines : Vec<&str> = all_lines.map(|line| line.split(": ").last().unwrap()).collect();
        let items = lines[0].split(", ").map(|item| item.parse::<Worry>().unwrap()).collect();
        let operation= Operation::new(lines[1]);
        let divisor = lines[2].split("divisible by ").last().unwrap().parse::<Worry>().unwrap();
        let monkey_one = lines[3].split(" monkey ").last().unwrap().parse::<usize>().unwrap();
        let monkey_two = lines[4].split(" monkey ").last().unwrap().parse::<usize>().unwrap();

        Self { inspections: 0, worry_reducer, items, operation, divisor, next_monkeys: (monkey_one, monkey_two) }
    }

    fn run_round(&mut self, common_multiple: Worry) -> Vec<Toss> {
        let tosses : Vec<Toss> = self.items.iter().map(|item| self.toss_for_item(*item, common_multiple)).collect();
        self.inspections += tosses.len();

        self.items = vec![];

        tosses
    }

    fn toss_for_item(&self, original_item: Worry, common_multiple: Worry) -> Toss {
        let mut item = self.increase_worry(original_item, common_multiple);
        if self.worry_reducer > 0 {
            item /= self.worry_reducer;
        }

        let target = self.target_for(item);

        Toss { item, target }
    }

    fn increase_worry(&self, item: Worry, common_multiple: Worry) -> Worry {
        let mut new_worry = match self.operation {
            Operation::Square => { item * item },
            Operation::Add(addend) => { item + addend },
            Operation::Multiply(multiplier) => { item * multiplier }
        };

        if self.worry_reducer == 0 {
            new_worry %= common_multiple;
        }

        new_worry
    }

    fn target_for(&self, item: Worry) -> usize {
        if (item % self.divisor) == 0 {
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
            10605
        );
    }

    #[test]
    fn part_one() {
        let mut troop = Troop::new(fs::read_to_string("input.txt").unwrap().as_str(), 3);
        troop.run_rounds(20);
        assert_eq!(
            troop.monkey_business(),
            55930
        );
    }

    #[test]
    fn part_two_example() {
        let mut troop = Troop::new(fs::read_to_string("example_input.txt").unwrap().as_str(), 0);
        troop.run_rounds(20);
        assert_eq!(troop.monkey_business(), 10197);
        troop.run_rounds(980);
        assert_eq!(troop.monkey_business(), (5204 * 5192));
        troop.run_rounds(9_000);
        assert_eq!(troop.monkey_business(), 2_713_310_158);
    }

    #[test]
    fn part_two() {
        let mut troop = Troop::new(fs::read_to_string("input.txt").unwrap().as_str(), 0);
        troop.run_rounds(10_000);
        assert_eq!(troop.monkey_business(), 14_636_993_466);
    }
}
