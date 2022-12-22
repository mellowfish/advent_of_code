extern crate core;

use std::collections::HashMap;
use std::ops::RangeInclusive;

#[derive(Clone, Copy, Debug)]
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

#[derive(Clone, Debug, PartialEq, Copy)]
enum MapFace {
    Top,    // 1
    Back,   // 2
    Left,   // 3
    Front,  // 4
    Bottom, // 5
    Right,  // 6
}

impl MapFace {
    fn from_grid(row: i32, column: i32) -> Option<MapFace> {
        match (row, column) {
            (0, 2) => Some(MapFace::Top),
            (1, 0) => Some(MapFace::Back),
            (1, 1) => Some(MapFace::Left),
            (1, 2) => Some(MapFace::Front),
            (2, 2) => Some(MapFace::Bottom),
            (2, 3) => Some(MapFace::Right),
            (0, 0) | (0, 1) | (0, 3) | (1, 3) | (2, 0) | (2, 1) => None, // blank spots on map (triggers portal)
            (-1, _) | (_, -1) | (3, _) | (_, 4) => None, // just outside map (triggers portal)
            _ => panic!("Invalid grid coordinates: {row}, {column}") // should never happen

        }
    }

    fn origin(&self, grid_size: usize) -> Point {
        let grid_size = grid_size as i32;
        match self {
            MapFace::Top => Point { x: 2 * grid_size, y: 0 },
            MapFace::Back => Point { x: 0, y: grid_size },
            MapFace::Left => Point { x: grid_size, y: grid_size },
            MapFace::Front => Point { x: 2 * grid_size, y: grid_size },
            MapFace::Bottom => Point { x: 2 * grid_size, y: 2 * grid_size },
            MapFace::Right => Point { x: 3 * grid_size, y: 2 * grid_size },
        }
    }

    fn portal(from: MapFace, pointed: Direction, grid_point: Point, grid_size: usize) -> (MapFace, Direction, Point) {
        let new_face =
            match (from.clone(), pointed.clone()) {
                (MapFace::Back, Direction::North)
                | (MapFace::Left, Direction::North)
                | (MapFace::Right, Direction::East)
                    => MapFace::Top,

                (MapFace::Top, Direction::North)
                | (MapFace::Bottom, Direction::South)
                | (MapFace::Right, Direction::South)
                    => MapFace::Back,

                (MapFace::Top, Direction::West)
                | (MapFace::Bottom, Direction::West)
                    => MapFace::Left,

                (MapFace::Top, Direction::East)
                | (MapFace::Back, Direction::West)
                | (MapFace::Front, Direction::East)
                    => MapFace::Right,

                (MapFace::Back, Direction::South)
                | (MapFace::Left, Direction::South)
                    => MapFace::Bottom,

                (MapFace::Right, Direction::North)
                    => MapFace::Front,

                (MapFace::Top, Direction::South)
                | (MapFace::Front, Direction::North) | (MapFace::Front, Direction::West) | (MapFace::Front, Direction::South)
                | (MapFace::Back, Direction::East)
                | (MapFace::Left, Direction::West) | (MapFace::Left, Direction::East)
                | (MapFace::Bottom, Direction::North) | (MapFace::Bottom, Direction::East)
                | (MapFace::Right, Direction::West)
                    => panic!("Normal traverse, not a portal!"),
            };

        let (new_direction, new_grid_point) = Self::traverse(from, new_face.clone(), grid_point, grid_size);

        (new_face, new_direction, new_grid_point)
    }

    fn traverse(from: MapFace, to: MapFace, from_grid_point: Point, grid_size: usize) -> (Direction, Point) {
        match (from.clone(), to.clone()) {
            // connected faces (opposite edges)
            (MapFace::Top, MapFace::Front)
            | (MapFace::Front, MapFace::Bottom) => (Direction::South, from_grid_point.wrap_to_grid_top()),

            (MapFace::Bottom, MapFace::Front)
            | (MapFace::Front, MapFace::Top) => (Direction::North, from_grid_point.wrap_to_grid_bottom(grid_size)),

            (MapFace::Back, MapFace::Left)
            | (MapFace::Left, MapFace::Front)
            | (MapFace::Bottom, MapFace::Right) => (Direction::East, from_grid_point.wrap_to_grid_left()),

            (MapFace::Front, MapFace::Left)
            | (MapFace::Left, MapFace::Back)
            | (MapFace::Right, MapFace::Bottom) => (Direction::West, from_grid_point.wrap_to_grid_right(grid_size)),

            // transposed faces (edges rotated 90 degrees)

            (MapFace::Top, MapFace::Left) => (Direction::South, from_grid_point.transpose_grid_top_right_edge()),
            (MapFace::Left, MapFace::Top) => (Direction::East, from_grid_point.transpose_grid_top_right_edge()),


            (MapFace::Front, MapFace::Right) => (Direction::South, from_grid_point.transpose_grid_top_left_edge(grid_size)),
            (MapFace::Right, MapFace::Front) => (Direction::West, from_grid_point.transpose_grid_top_left_edge(grid_size)),

            (MapFace::Left, MapFace::Bottom)
            | (MapFace::Right, MapFace::Back) => (Direction::East, from_grid_point.transpose_grid_top_left_edge(grid_size)),

            (MapFace::Bottom, MapFace::Left)
            | (MapFace::Back, MapFace::Right) => (Direction::North, from_grid_point.transpose_grid_top_left_edge(grid_size)),

            // reversed faces (same edge joined backward)

            (MapFace::Top, MapFace::Back)
            | (MapFace::Back, MapFace::Top) => (Direction::South, from_grid_point.invert_x(grid_size)),

            (MapFace::Back, MapFace::Bottom)
            | (MapFace::Bottom, MapFace::Back) => (Direction::North, from_grid_point.invert_x(grid_size)),

            (MapFace::Top, MapFace::Right)
            | (MapFace::Right, MapFace::Top) => (Direction::West, from_grid_point.invert_y(grid_size)),


            (MapFace::Top, MapFace::Bottom) | (MapFace::Bottom, MapFace::Top)
            | (MapFace::Left, MapFace::Right) | (MapFace::Right, MapFace::Left)
            | (MapFace::Back, MapFace::Front) | (MapFace::Front, MapFace::Back)
                => panic!("Bending spacetime! Trying to go from {:?} to {:?}", from, to),

            (MapFace::Top, MapFace::Top) | (MapFace::Bottom, MapFace::Bottom)
            | (MapFace::Left, MapFace::Left) | (MapFace::Right, MapFace::Right)
            | (MapFace::Back, MapFace::Back) | (MapFace::Front, MapFace::Front)
                => panic!("From and to must be different")
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

    fn plus(&self, delta: &Self) -> Self {
        Self { x: self.x + delta.x, y: self.y + delta.y }
    }

    fn wrap_to_grid_top(&self) -> Self {
        Self { x: self.x, y: 0 }
    }

    fn wrap_to_grid_bottom(&self, grid_size: usize) -> Self {
        Self { x: self.x, y: (grid_size - 1) as i32 }
    }

    fn wrap_to_grid_left(&self) -> Self {
        Self { x: 0, y: self.y }
    }

    fn wrap_to_grid_right(&self, grid_size: usize) -> Self {
        Self { x: (grid_size - 1) as i32, y: self.y }
    }

    fn transpose_grid_top_right_edge(&self) -> Self {
        Self { x: self.y, y: self.x }
    }

    fn transpose_grid_top_left_edge(&self, grid_size: usize) -> Self {
        Self { x: grid_size as i32 - (self.y + 1), y: grid_size as i32 - (self.x + 1) }
    }

    fn invert_y(&self, grid_size: usize) -> Self {
        Self { x: self.x, y: grid_size as i32 - (self.y + 1) }
    }

    fn invert_x(&self, grid_size: usize) -> Self {
        Self { x: grid_size as i32 - (self.x + 1), y: self.y }
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

    fn decode_password(&self, folded: bool) -> usize {
        let mut current_point = Point { x: self.start_of_row(0), y: 0 };
        let mut current_direction = Direction::East;
        let mut path: HashMap<Point, Direction> = HashMap::new();

        for instruction in self.instructions.iter() {
            // dbg!(instruction);
            // self.print(&path, &current_point, &current_direction);

            match instruction {
                Instruction::Move(distance) => {
                    if folded {
                        let (new_current_point, new_current_direction) = self.walk_forward_folded(current_point, current_direction.clone(), *distance, &mut path);
                        current_point = new_current_point;
                        current_direction = new_current_direction;
                    } else {
                        let (new_current_point, new_current_direction) = self.walk_forward(current_point, current_direction.clone(), *distance, &mut path);
                        current_point = new_current_point;
                        current_direction = new_current_direction;
                    }
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

    fn walk_forward(&self, current_point: Point, current_direction: Direction, distance: usize, path: &mut HashMap<Point, Direction>) -> (Point, Direction) {
        let mut next_point = current_point.clone();

        // println!("Walking forward");
        // self.print(&path, &next_point, &current_direction);

        for _ in 0..distance {
            let mut possible_next_point = next_point.move_one_step(&current_direction);
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
                    return (next_point, current_direction.clone());
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
                                return (next_point, current_direction.clone());
                            },
                            None => {
                                possible_next_point = possible_next_point.move_one_step(&current_direction);
                            }
                        }
                    }
                }
            }
        }

        (next_point, current_direction)
    }

    fn walk_forward_folded(&self, current_point: Point, current_direction: Direction, distance: usize, path: &mut HashMap<Point, Direction>) -> (Point, Direction) {
        let mut next_point = current_point.clone();
        let (mut next_face, mut next_grid_point)  = self.face_for(&next_point).unwrap();
        let mut next_direction = current_direction.clone();
        // dbg!(next_point, next_direction, next_face);

        for _ in 0..distance {
            let mut possible_next_point = next_point.move_one_step(&next_direction);
            let mut possible_next_direction = next_direction.clone();
            let mut possible_next_face = next_face.clone();
            let mut possible_next_grid_point = next_grid_point.clone();

            // dbg!(possible_next_point, possible_next_direction, possible_next_face);

            match self.face_for(&possible_next_point) {
                Some((_possible_next_face, _possible_next_grid_point)) => {
                    possible_next_face = _possible_next_face;
                    possible_next_grid_point = _possible_next_grid_point;

                    // dbg!(next_face, possible_next_face);
                    if next_face != possible_next_face {
                        let (_possible_next_direction, _possible_next_grid_point) =
                            MapFace::traverse(next_face, possible_next_face, possible_next_grid_point, self.grid_size());

                        possible_next_direction = _possible_next_direction;
                        possible_next_grid_point = _possible_next_grid_point;
                    }
                },
                None => {
                    // println!("Time to portal!");
                    // dbg!(next_face, next_direction, next_grid_point);
                    let (_possible_next_face, _possible_next_direction, _possible_next_grid_point) =
                        MapFace::portal(next_face, next_direction, next_grid_point, self.grid_size());
                    possible_next_face = _possible_next_face;
                    possible_next_direction = _possible_next_direction;
                    possible_next_grid_point = _possible_next_grid_point;
                    // dbg!(possible_next_grid_point, possible_next_direction, possible_next_face);
                    possible_next_point = self.point_from_grid_point(&possible_next_grid_point, &possible_next_face);
                    // dbg!(possible_next_point);
                }
            }

            // having determined my possible next step, actually try to move in that direction
            match self.map.get(&possible_next_point) {
                Some(MapTile::Space) => {
                    path.insert(next_point.clone(), next_direction.clone());
                    next_point = possible_next_point;
                    next_direction = possible_next_direction;
                    next_face = possible_next_face;
                    next_grid_point = possible_next_grid_point;
                    // self.print(path, &next_point, &next_direction);
                },
                Some(MapTile::Wall) => {
                    // self.print(path, &next_point, &next_direction);
                    return (next_point, next_direction)
                },
                None => {
                    dbg!(possible_next_point, possible_next_direction, possible_next_face);
                    panic!("Somehow I fell off the cube...");
                }
            }
        }

        (next_point, next_direction)
    }

    fn grid_size(&self) -> usize {
        self.height() / 3
    }

    fn face_for(&self, point: &Point) -> Option<(MapFace, Point)> {
        let grid_size = self.grid_size() as i32;
        let grid_row = point.y / grid_size;
        let grid_column = point.x / grid_size;
        dbg!(grid_size, grid_row, grid_column);

        match MapFace::from_grid(grid_row, grid_column) {
            Some(face) => {
                let interior_grid_row = point.y % grid_size;
                let interior_grid_column = point.x % grid_size;

                Some((face, Point { x: interior_grid_column, y: interior_grid_row }))
            },
            None => None
        }
    }

    fn point_from_grid_point(&self, grid_point: &Point, face: &MapFace) -> Point {
        grid_point.plus(&face.origin(self.grid_size()))
    }

    // cargo test --package day_twenty_two --lib tests::part_two_example -- --exact --nocapture
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
        assert_eq!(PasswordMap::new(fs::read_to_string("example_input.txt").unwrap().as_str()).decode_password(false), 6032);
    }

    #[test]
    fn part_one() {
        assert_eq!(PasswordMap::new(fs::read_to_string("input.txt").unwrap().as_str()).decode_password(false), 30552);
    }

    #[test]
    fn part_two_example() {
        assert_eq!(PasswordMap::new(fs::read_to_string("example_input.txt").unwrap().as_str()).decode_password(true), 5031);
    }

    // TODO: the shape of the input cube/map is different than the example... can I somehow code a dynamic shape (with fixed grid size) that I pass in?
    #[test]
    fn part_two() {
        assert_eq!(PasswordMap::new(fs::read_to_string("input.txt").unwrap().as_str()).decode_password(true), 5031);
    }
}
