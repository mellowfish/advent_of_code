#[derive(PartialEq, Clone, Debug)]
struct House {
    row: i32,
    column: i32
}

impl House {
    fn origin() -> Self {
        Self { row: 0, column: 0}
    }

    fn up(&self) -> Self {
        Self { row: self.row + 1, column: self.column }
    }

    fn down(&self) -> Self {
        Self { row: self.row - 1, column: self.column }
    }

    fn left(&self) -> Self {
        Self { row: self.row, column: self.column + 1 }
    }

    fn right(&self) -> Self {
        Self { row: self.row, column: self.column - 1 }
    }
}

struct SantaRoute {
    stops: Vec<House>,
}

impl SantaRoute {
    fn interpret_instruction(instruction : char, house: House) -> House {
        match instruction {
            '^' => house.up(),
            'v' | 'V' => house.down(),
            '<' => house.left(),
            '>' => house.right(),
            unexpected_character => panic!("Unsupported direction: '{unexpected_character}'")
        }
    }

    fn build_part_one(input : &str) -> Self {
        let mut current_house = House::origin();
        let mut stops : Vec<House> = vec![current_house.clone()];

        for instruction in input.chars() {
            current_house = SantaRoute::interpret_instruction(instruction, current_house);
            stops.push(current_house.clone());
        }

        Self { stops }
    }

    fn build_part_two(input : &str) -> Self {
        let mut santa_current_house = House::origin();
        let mut robosanta_current_house = House::origin();
        let mut stops : Vec<House> = vec![santa_current_house.clone()];

        for (index, instruction) in input.chars().enumerate() {
            match index % 2 {
                0 => {
                    santa_current_house = SantaRoute::interpret_instruction(instruction, santa_current_house);
                    stops.push(santa_current_house.clone());
                },
                1 => {
                    robosanta_current_house = SantaRoute::interpret_instruction(instruction, robosanta_current_house);
                    stops.push(robosanta_current_house.clone());
                },
                _ => panic!("not possible")
            }
        }

        Self { stops }
    }

    fn total_visited_houses(&self) -> u32 {
        let mut unique_stops : Vec<House> = vec![];

        for house in self.stops.iter() {
            if !unique_stops.contains(house) {
                unique_stops.push(house.clone());
            }
        }

        unique_stops.len() as u32
    }
}

#[cfg(test)]
mod tests {
    use std::fs;
    use super::*;

    #[test]
    fn part_one_examples() {
        assert_eq!(SantaRoute::build_part_one(">").total_visited_houses(), 2);
        assert_eq!(SantaRoute::build_part_one("^>v<").total_visited_houses(), 4);
        assert_eq!(SantaRoute::build_part_one("^v^v^v^v^v").total_visited_houses(), 2);
    }

    #[test]
    fn part_one() {
        assert_eq!(SantaRoute::build_part_one(fs::read_to_string("input.txt").unwrap().as_str()).total_visited_houses(), 2572);
    }

    #[test]
    fn part_two_examples() {
        assert_eq!(SantaRoute::build_part_two("^v").total_visited_houses(), 3);
        assert_eq!(SantaRoute::build_part_two("^>v<").total_visited_houses(), 3);
        assert_eq!(SantaRoute::build_part_two("^v^v^v^v^v").total_visited_houses(), 11);
    }

    #[test]
    fn part_two() {
        assert_eq!(SantaRoute::build_part_two(fs::read_to_string("input.txt").unwrap().as_str()).total_visited_houses(), 2631);
    }
}
