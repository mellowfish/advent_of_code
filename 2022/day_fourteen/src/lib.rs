use std::cmp::{max, min};
use std::collections::HashMap;

enum Regolith {
    Sand,
    Rock
}

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
struct Point {
    x:  i32,
    y: i32
}

impl Point {
    fn new(input: &str) -> Self {
        let parts : Vec<i32> = input.split(",").map(|number| number.parse::<i32>().unwrap()).collect();

        Self { x: parts[0], y: parts[1] }
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

    fn is_above(&self, other: &Point) -> bool {
        self.y < other.y
    }

    fn is_below(&self, other: &Point) -> bool {
        self.y > other.y
    }

    fn is_left_of(&self, other: &Point) -> bool {
        self.x < other.x
    }

    #[allow(dead_code)]
    fn is_right_of(&self, other: &Point) -> bool {
        self.x > other.x
    }
}

struct Path {
    vertices: Vec<Point>
}

impl Path {
    fn new(input: &str) -> Self {
        Self { vertices: input.split(" -> ").map(Point::new).collect() }
    }

    fn points(&self) -> Vec<Point> {
        self.vertices.windows(2).map(|segment_points| {
            Self::points_between(&segment_points[0], &segment_points[1])
        }).flatten().collect()
    }

    fn points_between(start: &Point, end: &Point) -> Vec<Point> {
        if start.is_above(end) {
            (start.y..=end.y).map(|y| Point { x: start.x, y }).collect()
        } else if start.is_below(end) {
            (end.y..=start.y).map(|y| Point { x: start.x, y }).collect()
        } else if start.is_left_of(end) {
            (start.x..=end.x).map(|x| Point { x, y: start.y }).collect()
        } else {
            (end.x..=start.x).map(|x| Point { x, y: start.y }).collect()
        }
    }
}

struct CaveSystem {
    stable_points: HashMap<Point, Regolith>
}

impl CaveSystem {
    fn new(input: &str) -> Self {
        let mut stable_points : HashMap<Point, Regolith>  = HashMap::new();

        for path in input.lines().map(Path::new) {
            for point in path.points().iter() {
                stable_points.insert(point.clone(), Regolith::Rock);
            }
        }

        Self { stable_points }
    }

    fn sand_origin() -> Point {
        Point { x: 500, y: 0 }
    }

    fn fill_with_sand(&mut self, stop_at_floor: bool) {
        let floor;
        if stop_at_floor {
            floor = Some(self.floor());
        } else {
            floor = None;
        }

        loop {
            match self.drop_one_sand(floor) {
                Some(point) => {
                    self.stable_points.insert(point, Regolith::Sand);
                    if point.eq(&Self::sand_origin()) {
                        return;
                    }
                },
                None => {
                    return;
                }
            }
        }
    }

    fn regolith_at(&self, point: &Point, optional_floor: Option<i32>) -> Option<&Regolith> {
        self.stable_points.get(point).or({
            match optional_floor {
                Some(floor_row) => {
                    if point.y == floor_row {
                        Some(&Regolith::Rock)
                    } else {
                        None
                    }
                },
                None => None
            }
        })
    }

    fn drop_one_sand(&self, optional_floor: Option<i32>) -> Option<Point> {
        let mut sand = Self::sand_origin();
        let mut next_sand : Point;

        loop {
            next_sand = sand.down();
            match self.regolith_at(&next_sand, optional_floor) {
                Some(_) => {
                    next_sand = next_sand.left();
                    match self.regolith_at(&next_sand, optional_floor) {
                        Some(_) => {
                            next_sand = next_sand.right().right();
                            match self.regolith_at(&next_sand, optional_floor) {
                                Some(_) => { return Some(sand) }, // can't move anywhere, return current position
                                None => { sand = next_sand } // move down-right
                            }
                        },
                        None => { sand = next_sand } // move down-left
                    }
                },
                None => {
                    if matches!(optional_floor, None) && next_sand.is_below(&self.dimensions().1) {
                        return None; // going to fall forever, abort!
                    }
                    sand = next_sand; // move down
                }
            }
        }
    }

    fn units_of_sand(&self) -> usize {
        self.stable_points.values().filter(|regolith| matches!(regolith, Regolith::Sand)).count()
    }

    fn dimensions(&self) -> (Point, Point) {
        let mut min_x : i32 = 600;
        let mut min_y : i32 = 0;
        let mut max_x : i32 = 0;
        let mut max_y : i32 = 0;

        for &point in self.stable_points.keys() {
            min_y = min(min_y, point.y);
            max_y = max(max_y, point.y);
            min_x = min(min_x, point.x);
            max_x = max(max_x, point.x);
        }

        ( Point { x: min_x, y: min_y }, Point { x: max_x, y: max_y } )
    }

    fn floor(&self) -> i32 {
        let mut max_x = 0;

        for (point, regolith) in self.stable_points.iter() {
            if matches!(regolith, Regolith::Rock) {
                max_x = max(max_x, point.y)
            }
        }

        max_x + 2
    }

    #[allow(dead_code)]
    fn print(&self) {
        let (top_left, bottom_right) = self.dimensions();

        for row in top_left.y..=(bottom_right.y + 1) {
            for column in (top_left.x - 1)..=(bottom_right.x + 1) {
                if row == 0 && column == 500 {
                    print!("+");
                    continue;
                }
                let point = Point { x: column, y: row };
                match self.stable_points.get(&point) {
                    Some(Regolith::Rock) => { print!("#") },
                    Some(Regolith::Sand) => { print!("o") },
                    None => { print!(".") }
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
        let mut cave_system = CaveSystem::new(fs::read_to_string("example_input.txt").unwrap().as_str());
        cave_system.fill_with_sand(false);
        cave_system.print();
        assert_eq!(cave_system.units_of_sand(), 24);
    }

    #[test]
    fn part_one() {
        let mut cave_system = CaveSystem::new(fs::read_to_string("input.txt").unwrap().as_str());
        cave_system.fill_with_sand(false);
        cave_system.print();
        assert_eq!(cave_system.units_of_sand(), 745);
    }

    #[test]
    fn part_two_example() {
        let mut cave_system = CaveSystem::new(fs::read_to_string("example_input.txt").unwrap().as_str());
        cave_system.fill_with_sand(true);
        cave_system.print();
        assert_eq!(cave_system.units_of_sand(), 93);
    }

    #[test]
    fn part_two() {
        let mut cave_system = CaveSystem::new(fs::read_to_string("input.txt").unwrap().as_str());
        cave_system.fill_with_sand(true);
        cave_system.print();
        assert_eq!(cave_system.units_of_sand(), 27551);
    }
}
