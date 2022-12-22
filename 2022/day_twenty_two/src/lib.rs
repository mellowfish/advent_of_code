use std::borrow::Borrow;
use std::collections::HashMap;
use std::ops::RangeInclusive;
use std::thread::sleep;
use std::time::Duration;

#[derive(Clone)]
enum Direction {
    North,
    South,
    East,
    West
}

impl Direction {
    fn turn_left(&self) -> Direction {
        match self {
            Direction::North => Direction::West,
            Direction::West => Direction::South,
            Direction::South => Direction::East,
            Direction::East => Direction::North
        }
    }

    fn turn_right(&self) -> Direction {
        match self {
            Direction::North => Direction::East,
            Direction::East => Direction::South,
            Direction::South => Direction::West,
            Direction::West => Direction::North
        }
    }

    fn score(&self) -> usize {
        match self {
            Direction::East => 0,
            Direction::South => 1,
            Direction::West => 2,
            Direction::North => 3,
        }
    }
}

enum MapTile {
    Space,
    Wall
}

impl MapTile {
    fn new(symbol: char) -> Option<Self> {
        match symbol {
            '.' => Some(MapTile::Space),
            '#' => Some(MapTile::Wall),
            ' ' => None,
            _ => panic!("Unexpected character: {symbol}")
        }
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
struct Point {
    x:  i32,
    y: i32
}

impl Point {
    fn up(&self) -> Self {
        Self { x: self.x, y: self.y - 1 }
    }

    fn down(&self) -> Self {
        Self { x: self.x, y: self.y + 1 }
    }

    fn left(&self) -> Self {
        Self { x: self.x - 1, y: self.y }
    }

    fn right(&self) -> Self {
        Self { x: self.x + 1, y: self.y }
    }

    fn move_one_step(&self, direction: &Direction) -> Self {
        match direction {
            Direction::North => self.up(),
            Direction::East => self.right(),
            Direction::South => self.down(),
            Direction::West => self.left()
        }
    }

    fn score(&self) -> usize {
        assert!(self.y >= 0);
        assert!(self.x >= 0);

        (((self.y + 1) * 1000) + ((self.x + 1) * 4)) as usize
    }
}

#[derive(Debug)]
enum Instruction {
    Move(usize),
    TurnLeft,
    TurnRight
}

impl Instruction {
    fn new_sequence(input: &str) -> Vec<Self> {
        let mut sequence = vec![];
        let mut characters = input.trim().chars();
        let mut character = characters.next().unwrap();

        loop {
            match character {
                'L' => {
                    sequence.push(Instruction::TurnLeft);
                },
                'R' => {
                    sequence.push(Instruction::TurnRight);
                },
                '0'..='9' => {
                    let mut number = character.to_digit(10).unwrap();
                    if let Some(temp_character) = characters.next() {
                        character = temp_character;
                        match character {
                            '0'..='9' => {
                                number = (number * 10) + character.to_digit(10).unwrap();
                                sequence.push(Instruction::Move(number as usize));
                                // TODO(?): Handle 3 digit numbers
                            },
                            _ => {
                                sequence.push(Instruction::Move(number as usize));
                                continue;
                            }
                        }
                    } else {
                        sequence.push(Instruction::Move(number as usize));
                        return sequence;
                    }
                },
                _ => panic!("Unexpected character: {character}")
            }

            if let Some(temp_character) = characters.next() {
                character = temp_character;
                continue;
            }

            break;
        }

        sequence
    }
}

type RowRange = RangeInclusive<i32>;

struct PasswordMap {
    map: HashMap<Point, MapTile>,
    instructions: Vec<Instruction>,
    row_ranges: Vec<RowRange>
}

impl PasswordMap {
    fn new(input: &str) -> Self {
        let (map_str, instructions_str) = input.split_once("\n\n").unwrap();
        let instructions = Instruction::new_sequence(instructions_str);

        let mut map: HashMap<Point, MapTile> = HashMap::new();
        let mut row_ranges : Vec<RowRange> = vec![];

        for (row, line) in map_str.lines().enumerate() {
            let mut start_col = 0;
            let end_col = (line.len() - 1) as i32;
            for (column, character) in line.chars().enumerate() {
                if let Some(tile) = MapTile::new(character) {
                    map.insert(Point { x: column as i32, y: row as i32 }, tile);
                } else {
                    start_col += 1;
                }
            }
            row_ranges.push(start_col..=end_col)
        }

        Self { map, instructions, row_ranges }
    }

    fn row_range(&self, row: usize) -> &RowRange {
        &self.row_ranges[row]
    }

    fn start_of_row(&self, row: usize) -> i32 {
        *self.row_range(row).start()
    }

    fn end_of_row(&self, row: usize) -> i32 {
        *self.row_range(row).end()
    }

    fn height(&self) -> usize {
        self.row_ranges.len()
    }

    fn width(&self) -> usize {
        self.row_ranges.iter().map(|row_range| *row_range.end()).max().unwrap() as usize
    }

    fn decode_password(&self) -> usize {
        let mut current_point = Point { x: self.start_of_row(0), y: 0 };
        let mut current_direction = Direction::East;
        let mut path: HashMap<Point, Direction> = HashMap::new();

        for instruction in self.instructions.iter() {
            // dbg!(instruction);
            // self.print(&path, &current_point, &current_direction);

            match instruction {
                Instruction::Move(distance) => {
                    current_point = self.walk_forward(&current_point, &current_direction, *distance, &mut path);
                },
                Instruction::TurnLeft => {
                    current_direction = current_direction.turn_left();
                },
                Instruction::TurnRight => {
                    current_direction = current_direction.turn_right();
                }
            }
            // sleep(Duration::from_secs(1));
        }

        current_point.score() + current_direction.score()
    }

    fn walk_forward(&self, current_point: &Point, current_direction: &Direction, distance: usize, path: &mut HashMap<Point, Direction>) -> Point {
        let mut next_point = current_point.clone();

        // println!("Walking forward");
        // self.print(&path, &next_point, &current_direction);

        for _ in 0..distance {
            let mut possible_next_point = next_point.move_one_step(current_direction);
            // dbg!(&possible_next_point);

            match self.map.get(&possible_next_point) {
                Some(MapTile::Space) => {
                    path.insert(next_point.clone(), current_direction.clone());
                    next_point = possible_next_point;
                    // self.print(&path, &next_point, &current_direction);
                },
                Some(MapTile::Wall) => {
                    // self.print(&path, &next_point, &current_direction);
                    // println!("Hit a wall");
                    return next_point;
                },
                None => {
                    // println!("Wrapping around");
                    possible_next_point =
                        match current_direction {
                            Direction::North => Point { x: current_point.x, y: self.height() as i32 },
                            Direction::East => Point { x: 0, y: current_point.y },
                            Direction::South => Point { x: current_point.x, y: 0 },
                            Direction::West => Point { x: self.width() as i32, y: current_point.y },
                        };
                    // dbg!(&possible_next_point);

                    loop {
                        match self.map.get(&possible_next_point) {
                            Some(MapTile::Space) => {
                                path.insert(next_point.clone(), current_direction.clone());
                                next_point = possible_next_point;
                                // self.print(&path, &next_point, &current_direction);
                                break;
                            },
                            Some(MapTile::Wall) => {
                                // self.print(&path, &next_point, &current_direction);
                                // println!("Hit a wall");
                                return next_point;
                            },
                            None => {
                                possible_next_point = possible_next_point.move_one_step(&current_direction);
                            }
                        }
                    }
                }
            }
        }

        next_point
    }

    // cargo test --package day_twenty_two --lib tests::part_one_example -- --exact --nocapture
    #[allow(dead_code)]
    fn print(&self, path: &HashMap<Point, Direction>, current_point: &Point, current_direction: &Direction) {
        for (row_index, row_range) in self.row_ranges.iter().enumerate() {
            for _ in 0..*row_range.start() {
                print!(" ");
            }

            for column_index in row_range.clone() {
                let point = Point { x: column_index, y: row_index as i32 };
                match path.get(&point) {
                    Some(Direction::North) => print!("^"),
                    Some(Direction::East) => print!(">"),
                    Some(Direction::South) => print!("v"),
                    Some(Direction::West) => print!("<"),
                    None => {
                        if point.eq(current_point) {
                            match current_direction {
                                Direction::North => print!("^"),
                                Direction::East => print!(">"),
                                Direction::South => print!("v"),
                                Direction::West => print!("<"),
                            }
                        } else {
                            match self.map.get(&point) {
                                Some(MapTile::Space) => print!("."),
                                Some(MapTile::Wall) => print!("#"),
                                None => { print!("?") }
                            }
                        }
                    }
                }
            }
            println!();
        }

        println!();
    }
}

#[cfg(test)]
mod tests {
    use std::fs;
    use super::*;

    #[test]
    fn part_one_example() {
        assert_eq!(PasswordMap::new(fs::read_to_string("example_input.txt").unwrap().as_str()).decode_password(), 6032);
    }

    #[test]
    fn part_one() {
        assert_eq!(PasswordMap::new(fs::read_to_string("input.txt").unwrap().as_str()).decode_password(), 30552);
    }
}
