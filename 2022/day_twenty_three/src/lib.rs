use std::cmp::{max, min};
use std::collections::HashMap;
use std::iter::Cycle;
use std::slice::Iter;

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
struct Point {
    x: i32,
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

    fn neighbors(&self) -> Vec<Point> {
        let mut neighbors = vec![];
        let mut current = self.up(); // north
        let mut next = current.left(); // northwest
        neighbors.push(current);
        current = next;
        next = current.down(); // west
        neighbors.push(current);
        current = next;
        next = current.down(); // southwest
        neighbors.push(current);
        current = next;
        next = current.right(); // south
        neighbors.push(current);
        current = next;
        next = current.right(); // southeast
        neighbors.push(current);
        current = next;
        next = current.up(); // east
        neighbors.push(current);
        current = next;
        next = current.up(); // northeast
        neighbors.push(current);
        neighbors.push(next);

        neighbors
    }
}

#[derive(Clone)]
struct Elf {
    current_position: Point
}

struct PlantingParty {
    elves: HashMap<Point, Elf>
}

impl PlantingParty {
    fn new(input: &str) -> Self {
        let mut elves = HashMap::new();

        for (row, symbols) in input.lines().enumerate() {
            for (column, symbol) in symbols.chars().enumerate() {
                if symbol == '#' {
                    let point = Point { x: column as i32, y: row as i32 };
                    elves.insert(point, Elf { current_position: point });
                }
            }
        }

        Self { elves }
    }

    fn run_rounds(&mut self, rounds: usize) -> usize {
        let original_directions = vec!['N', 'S', 'W', 'E'];
        let mut directions : Cycle<Iter<char>> = original_directions.iter().cycle();

        println!("Start");
        self.print();

        for n in 0..rounds {
            let round = n + 1;
            let moves = self.run_round(&mut directions);

            if moves == 0 {
                println!("Reached steady state after {} round(s):", round);
                self.print();
                return round
            }

            if round % 100 == 0 {
                println!("Round: {}: {} move(s)", round, moves);
                self.print();
            }
            directions.next();
        }

        rounds
    }

    fn has_elf_at(&self, point: &Point) -> bool {
        self.elves.contains_key(point)
    }

    fn run_round(&mut self, directions: &mut Cycle<Iter<char>>) -> usize {
        let mut proposed_moves : HashMap<Point, Vec<Elf>> = HashMap::new();

        for (point, elf) in self.elves.iter() {
            if point.neighbors().iter().all(|neighbor| { !self.has_elf_at(neighbor)}) {
                continue;
            };

            for direction in directions.clone().take(4) {
                match direction {
                    'N' => {
                        if let Some(point) = self.consider_moving_north(elf) {
                            self.record_proposed_move(&mut proposed_moves, elf, point);
                            break;
                        }
                    },
                    'S' => {
                        if let Some(point) = self.consider_moving_south(elf) {
                            self.record_proposed_move(&mut proposed_moves, elf, point);
                            break;
                        }

                    },
                    'E' => {
                        if let Some(point) = self.consider_moving_east(elf) {
                            self.record_proposed_move(&mut proposed_moves, elf, point);
                            break;
                        }
                    },
                    'W' => {
                        if let Some(point) = self.consider_moving_west(elf) {
                            self.record_proposed_move(&mut proposed_moves, elf, point);
                            break;
                        }
                    },
                    _ => panic!("Unexpected direction {direction}")
                }
            }
        }

        let mut moves = 0;
        for (point, elves) in proposed_moves.iter() {
            if elves.len() > 1 {
                continue;
            }

            moves += 1;

            self.elves.remove(&elves[0].current_position);
            self.elves.insert(point.clone(), Elf { current_position: point.clone()});
        }

        moves
    }

    fn record_proposed_move(&self, proposed_moves: &mut HashMap<Point, Vec<Elf>>, elf: &Elf, point: Point) {
        match proposed_moves.get_mut(&point) {
            Some(other_elves) => {
                other_elves.push(elf.clone());
            },
            None => {
                proposed_moves.insert(point, vec![elf.clone()]);
            }
        }
    }

    fn consider_moving_north(&self, elf: &Elf) -> Option<Point> {
        let north = elf.current_position.up();
        if !(self.has_elf_at(&north) || self.has_elf_at(&north.left()) || self.has_elf_at(&north.right())) {
            Some(north)
        } else {
            None
        }
    }

    fn consider_moving_south(&self, elf: &Elf) -> Option<Point> {
        let south = elf.current_position.down();
        if !(self.has_elf_at(&south) || self.has_elf_at(&south.left()) || self.has_elf_at(&south.right())) {
            Some(south)
        } else {
            None
        }
    }

    fn consider_moving_west(&self, elf: &Elf) -> Option<Point> {
        let west = elf.current_position.left();
        if !(self.has_elf_at(&west) || self.has_elf_at(&west.up()) || self.has_elf_at(&west.down())) {
            Some(west)
        } else {
            None
        }
    }

    fn consider_moving_east(&self, elf: &Elf) -> Option<Point> {
        let east = elf.current_position.right();
        if !(self.has_elf_at(&east) || self.has_elf_at(&east.up()) || self.has_elf_at(&east.down())) {
            Some(east)
        } else {
            None
        }
    }

    fn print(&self) {
        let (top_left, bottom_right) = self.dimensions();

        for row in top_left.y..=bottom_right.y {
            for column in top_left.x..=bottom_right.x {
                if self.has_elf_at(&Point { x: column, y: row }) {
                    print!("#")
                } else {
                    print!(".")
                }
            }
            println!()
        }
        println!()
    }

    fn dimensions(&self) -> (Point, Point) {
        let mut min_x : i32 = i32::MAX;
        let mut min_y : i32 = i32::MAX;
        let mut max_x : i32 = i32::MIN;
        let mut max_y : i32 = i32::MIN;

        for (point, _) in self.elves.iter() {
            min_y = min(min_y, point.y);
            max_y = max(max_y, point.y);
            min_x = min(min_x, point.x);
            max_x = max(max_x, point.x);
        }

        ( Point { x: min_x, y: min_y }, Point { x: max_x, y: max_y } )
    }

    fn open_land(&self) -> usize {
        let (top_left, bottom_right) = self.dimensions();
        let mut land = 0;

        for row in top_left.y..=bottom_right.y {
            for column in top_left.x..=bottom_right.x {
                if !self.has_elf_at(&Point { x: column, y: row }) {
                    land += 1
                }
            }
        }

        land
    }
}

#[cfg(test)]
mod tests {
    use std::fs;
    use super::*;

    #[test]
    fn part_one_example_small() {
        let mut planting_party = PlantingParty::new(fs::read_to_string("example_small_input.txt").unwrap().as_str());

        planting_party.run_rounds(10);

        assert_eq!(planting_party.open_land(), 25);
    }

    #[test]
    fn part_one_example() {
        let mut planting_party = PlantingParty::new(fs::read_to_string("example_input.txt").unwrap().as_str());

        planting_party.run_rounds(10);

        assert_eq!(planting_party.open_land(), 110);
    }

    #[test]
    fn part_one() {
        let mut planting_party = PlantingParty::new(fs::read_to_string("input.txt").unwrap().as_str());

        planting_party.run_rounds(10);

        assert_eq!(planting_party.open_land(), 4249);
    }

    #[test]
    fn part_two_example() {
        assert_eq!(
            PlantingParty::new(fs::read_to_string("example_input.txt").unwrap().as_str()).run_rounds(100),
            20
        );
    }

    #[test]
    fn part_two() {
        assert_eq!(
            PlantingParty::new(fs::read_to_string("input.txt").unwrap().as_str()).run_rounds(1_000),
            980
        );
    }
}
