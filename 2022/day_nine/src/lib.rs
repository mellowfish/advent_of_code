use std::cmp::{max, min};
use std::collections::HashSet;

#[derive(Eq,PartialEq,Copy,Clone,Debug,Hash)]
struct Point {
    x: i32,
    y: i32
}

impl Point {
    fn origin() -> Self { Self { x: 0, y: 0 } }

    #[allow(dead_code)]
    fn is_origin(&self) -> bool {
        self.x == 0 && self.y == 0
    }

    fn up(&self, delta: usize) -> Self {
        Self { x: self.x, y: self.y + (delta as i32) }
    }

    fn down(&self, delta: usize) -> Self {
        Self { x: self.x, y: self.y - (delta as i32) }
    }

    fn left(&self, delta: usize) -> Self {
        Self { x: self.x - (delta as i32), y: self.y }
    }

    fn right(&self, delta: usize) -> Self {
        Self { x: self.x + (delta as i32), y: self.y }
    }

    fn delta(&self, other: &Point) -> Self {
        Self { x: self.x - other.x, y: self.y - other.y }
    }
}

struct RopeBridge {
    head_positions: Vec<Point>,
    tail_positions: Vec<Vec<Point>>
}

impl RopeBridge {
    fn new(size: usize) -> Self {
        Self { head_positions: vec![Point::origin()], tail_positions: vec![vec![Point::origin()]; size] }
    }

    fn total_tail_positions(&self) -> usize {
        let unique_tail_positions : HashSet<&Point> = HashSet::from_iter(self.tail_positions.last().unwrap().iter());

        unique_tail_positions.len()
    }

    fn head(&self) -> &Point {
        self.head_positions.last().unwrap()
    }

    #[allow(dead_code)]
    fn knot_at(&self, index: usize) -> &Point {
        self.tail_positions[index].last().unwrap()
    }

    #[allow(dead_code)]
    fn tail(&self) -> &Point {
        self.tail_positions.last().unwrap().last().unwrap()
    }

    fn run_sequence(&mut self, input: &str) {
        self.print();

        for line in input.lines() {
            let parts : Vec<&str> = line.split(" ").collect();
            let delta = parts[1].parse::<usize>().unwrap();

            // println!("{line}");
            for _ in 0..delta {
                match parts[0] {
                    "U" => { self.move_head_up(1); },
                    "D" => { self.move_head_down(1); },
                    "L" => { self.move_head_left(1); },
                    "R" => { self.move_head_right(1); },
                    _ => panic!("Malformed line: {}", line)
                }
                self.print();
            }
        }
    }

    fn move_head_up(&mut self, delta: usize) {
        self.head_positions.push(self.head().up(delta));
        self.update_tail_positions();
    }

    fn move_head_down(&mut self, delta: usize) {
        self.head_positions.push(self.head().down(delta));
        self.update_tail_positions();
    }

    fn move_head_left(&mut self, delta: usize) {
        self.head_positions.push(self.head().left(delta));
        self.update_tail_positions();
    }

    fn move_head_right(&mut self, delta: usize) {
        self.head_positions.push(self.head().right(delta));
        self.update_tail_positions();
    }

    fn update_tail_positions(&mut self) {
        let mut new_head_position = *self.head();
        let mut old_knot_position : Point;
        let mut new_knot_position : Point;
        for (index, knot_positions) in self.tail_positions.iter_mut().enumerate() {
            old_knot_position = *knot_positions.last().unwrap();

            // dbg!(old_knot_position);
            // dbg!(new_head_position.delta(&old_knot_position));
            new_knot_position =
                match new_head_position.delta(&old_knot_position) {
                    Point { x: 0, y: 0 }
                    | Point { x: 1, y: 0 } | Point { x: -1, y: 0 } | Point { x: 0, y: 1 } | Point { x: 0, y: -1 }
                    | Point { x: 1, y: 1 } | Point { x: 1, y: -1 } | Point { x: -1, y: 1 } | Point { x: -1, y: -1 } => { old_knot_position.clone() },
                    Point { x: 0, y: 2 } => { old_knot_position.up(1) },
                    Point { x: 0, y: -2 } => { old_knot_position.down(1) },
                    Point { x: 2, y: 0 } => { old_knot_position.right(1) },
                    Point { x: -2, y: 0 } => { old_knot_position.left(1) },
                    Point { x: 1, y: 2 } | Point { x: 2, y: 1 } | Point { x: 2, y: 2 } => { old_knot_position.up(1).right(1) },
                    Point { x: 1, y: -2 } | Point { x: 2, y: -1 } | Point { x: 2, y: -2 } => { old_knot_position.down(1).right(1) },
                    Point { x: -1, y: 2 } | Point { x: -2, y: 1 } | Point { x: -2, y: 2 } => { old_knot_position.up(1).left(1) },
                    Point { x: -1, y: -2 } | Point { x: -2, y: -1 } | Point { x: -2, y: -2 } => { old_knot_position.down(1).left(1) },
                    delta => {
                        self.print();
                        dbg!(index + 1);
                        dbg!(new_head_position);
                        dbg!(old_knot_position);
                        panic!("Unexpected delta: {:?}", delta)
                    }
                };
            knot_positions.push(new_knot_position);

            new_head_position = new_knot_position;
        }
    }

    #[allow(dead_code)]
    fn dimensions(&self) -> (Point, Point) {
        let mut min_x = 0;
        let mut min_y = 0;
        let mut max_x = 0;
        let mut max_y = 0;

        for point in self.head_positions.iter() {
            min_y = min(min_y, point.y);
            max_y = max(max_y, point.y);
            min_x = min(min_x, point.x);
            max_x = max(max_x, point.x);
        }

        for knot_positions in self.tail_positions.iter() {
            for point in knot_positions.iter() {
                min_y = min(min_y, point.y);
                max_y = max(max_y, point.y);
                min_x = min(min_x, point.x);
                max_x = max(max_x, point.x);
            }
        }

        ( Point { x: min_x - 1, y: min_y - 1 }, Point { x: max_x + 1, y: max_y + 1 } )
    }

    #[allow(dead_code)]
    fn print(&self) {
        // let (bottom_left, top_right) = self.dimensions();
        //
        // for row in (bottom_left.y..=top_right.y).rev() {
        //     for column in bottom_left.x..=top_right.x {
        //         // dbg!(row, column);
        //
        //         let point = Point { x: column, y: row };
        //         if self.head().eq(&point) {
        //             print!("H")
        //         } else {
        //             let mut has_printed = false;
        //             for (index, knot_position) in self.tail_positions.iter().map(|knot_positions| knot_positions.last().unwrap()).enumerate() {
        //                 // dbg!(knot_position);
        //
        //                 if point.eq(knot_position) {
        //                     if self.tail_positions.len() == 1 {
        //                         print!("T");
        //                     } else {
        //                         print!("{}", index + 1);
        //                     }
        //                     has_printed = true;
        //
        //                     break;
        //                 }
        //             }
        //             if !has_printed {
        //                 if point.is_origin() {
        //                     print!("s")
        //                 } else {
        //                     print!(".")
        //                 }
        //             }
        //         }
        //     }
        //     println!();
        // }
        // println!();
    }
}

#[cfg(test)]
mod tests {
    use std::fs::read_to_string;
    use super::*;

    #[test]
    fn part_one_example() {
        let mut bridge = RopeBridge::new(1);
        bridge.run_sequence(read_to_string("example_input.txt").unwrap().as_str());

        assert_eq!(
            bridge.total_tail_positions(),
            13
        );
    }

    #[test]
    fn part_one() {
        let mut bridge = RopeBridge::new(1);
        bridge.run_sequence(read_to_string("input.txt").unwrap().as_str());

        assert_eq!(
            bridge.total_tail_positions(),
            5960
        );
    }

    #[test]
    fn part_two_example() {
        let mut bridge = RopeBridge::new(9);
        bridge.run_sequence(read_to_string("example_two_input.txt").unwrap().as_str());

        assert_eq!(
            bridge.total_tail_positions(),
            36
        );
    }

    #[test]
    fn part_two() {
        let mut bridge = RopeBridge::new(9);
        bridge.run_sequence(read_to_string("input.txt").unwrap().as_str());

        assert_eq!(
            bridge.total_tail_positions(),
            2327
        );
    }
}
