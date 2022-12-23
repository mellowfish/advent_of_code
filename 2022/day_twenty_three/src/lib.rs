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

    fn run_rounds(&mut self, rounds: usize) {
        let original_directions = vec!['N', 'S', 'W', 'E'];
        let mut directions : Cycle<Iter<char>> = original_directions.iter().cycle();
        self.print();
        for _ in 0..rounds {
            self.run_round(&mut directions);
            directions.next();
            self.print();
        }
    }

    fn has_elf_at(&self, point: &Point) -> bool {
        self.elves.contains_key(point)
    }

    fn run_round(&mut self, directions: &mut Cycle<Iter<char>>) {
        let mut proposed_moves : HashMap<Point, Vec<Elf>> = HashMap::new();

        for (point, elf) in self.elves.iter() {
            for direction in directions.take(4) {
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

        for (point, elves) in proposed_moves.iter() {
            if elves.len() > 1 {
                continue;
            }

            self.elves.remove(&elves[0].current_position);
            self.elves.insert(point.clone(), Elf { current_position: point.clone()});
        }
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
        let mut min_x : i32 = 0;
        let mut min_y : i32 = 0;
        let mut max_x : i32 = 0;
        let mut max_y : i32 = 0;

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
            println!()
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

        assert_eq!(planting_party.open_land(), 24);
    }
}
