enum Direction {
    North,
    South,
    East,
    West
}

impl Direction {
    fn new(symbol: char) -> Option<Self> {
        match symbol {
            '^' => Some(Direction::North),
            'v' => Some(Direction::South),
            '<' => Some(Direction::West),
            '>' => Some(Direction::East),
            _ => None
        }
    }

    fn to_string(&self) -> String {
        match self {
            Direction::North => String::from('^'),
            Direction::South => String::from('v'),
            Direction::West => String::from('<'),
            Direction::East => String::from('>'),
        }
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Debug, Ord, PartialOrd)]
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
        vec![self.up(), self.down(), self.left(), self.right()]
    }

    fn is_neighbor_of(&self, other: Point) -> bool {
        other == self.up() ||
            other == self.down() ||
            other == self.left() ||
            other == self.right()
    }
}

struct Blizzard {
    start_point: Point,
    direction: Direction
}

impl Blizzard {
    fn is_forecast(&self, tick: usize, target_point: &Point, interior_dimensions: &Point) -> bool {
        let tick = tick as i32;
        match self.direction {
            Direction::North => {
                target_point.x == self.start_point.x
                    && target_point.y == (self.start_point.y - tick).rem_euclid(interior_dimensions.y)
            },
            Direction::South => {
                target_point.x == self.start_point.x
                    && target_point.y == (self.start_point.y + tick).rem_euclid(interior_dimensions.y)
            },
            Direction::West => {
                target_point.y == self.start_point.y
                    && target_point.x == (self.start_point.x - tick).rem_euclid(interior_dimensions.x)
            },
            Direction::East => {
                target_point.y == self.start_point.y
                    && target_point.x == (self.start_point.x + tick).rem_euclid(interior_dimensions.x)
            },
        }
    }

    fn to_string(&self) -> String {
        self.direction.to_string()
    }
}

struct Valley {
    blizzards: Vec<Blizzard>,
    entrance: Point,
    exit: Point,
    interior_dimensions: Point,
    open_rooms_through_time: Vec<Vec<Point>>
}

impl Valley {
    fn new(input: &str) -> Self {
        let mut lines = input.lines();
        let row_count = lines.clone().count() as i32 - 2;
        let example_line = lines.next().unwrap(); // ignore first line
        let column_count = example_line.len() as i32 - 2;

        let entrance = Point { x: 0, y: -1 };
        let exit = Point { x: column_count - 1, y: row_count };
        let mut blizzards : Vec<Blizzard> = vec![];


        for (_row, line) in lines.enumerate() {
            let row = _row as i32;
            if row >= row_count {
                break;
            }

            for (_column, character) in line.chars().enumerate() {
                let column = _column as i32 - 1;
                if column < 0 || column >= column_count {
                    continue;
                }

                if let Some(direction) = Direction::new(character) {
                    blizzards.push(Blizzard { direction, start_point:  Point { x: column, y: row } });
                }
            }
        }

        let interior_dimensions = Point { x: column_count, y: row_count };
        let loop_count = ((interior_dimensions.x + 1) * (interior_dimensions.y + 1)) as usize;

        dbg!(loop_count);
        let mut open_rooms_through_time: Vec<Vec<Point>> = vec![];
        for tick in 0..loop_count {
            let mut open_rooms: Vec<Point> = vec![entrance.clone()];

            let mut rooms_to_check : Vec<Point>;
            if tick == 0 {
                rooms_to_check = vec![entrance.down()];
            } else {
                rooms_to_check = open_rooms_through_time[tick - 1].iter().flat_map(|point| {
                    point.neighbors().into_iter().filter(|neighbor| {
                        if neighbor.eq(&entrance) || neighbor.eq(&exit) {
                            true
                        } else {
                            0 <= neighbor.x && neighbor.x < interior_dimensions.x
                                && 0 <= neighbor.y && neighbor.y < interior_dimensions.y
                        }
                    })
                }).collect();
            }
            rooms_to_check.sort();
            rooms_to_check.dedup();
            // for row in 0..interior_dimensions.y {
            //     for column in 0..interior_dimensions.x {
            //         let point = Point { x: column, y: row };
            for point in rooms_to_check.iter() {
                if !blizzards.iter().any(|blizzard| blizzard.is_forecast(tick, point, &interior_dimensions)) {
                    if tick == 0 {
                        if point.is_neighbor_of(entrance) {
                            open_rooms.push(point.clone());
                        }
                    } else if open_rooms_through_time[tick - 1].iter().any(|previous_point| previous_point.eq(point) || previous_point.is_neighbor_of(point.clone())) {
                        open_rooms.push(point.clone());
                    }
                }
            }

            open_rooms.push(exit.clone());

            if tick % 100 == 0 {
                dbg!(tick, open_rooms.len());
            }

            open_rooms_through_time.push(open_rooms)
        }

        Self { blizzards, entrance, exit, interior_dimensions, open_rooms_through_time }
    }

    fn nodes(&self) -> Vec<(usize, usize, Point)> {
        let mut nodes : Vec<(usize, Point)> = vec![(0, self.entrance.clone())];

        for (tick, open_nodes) in self.open_rooms_through_time.iter().enumerate() {
            if tick == 0 {
                continue;
            }
            if open_nodes[0] == self.entrance.down() {
                nodes.push((tick + 1, self.entrance.clone()))
            }
            nodes.extend(open_nodes.iter().map(|point| (tick, point.clone())));
            if *open_nodes.last().unwrap() == self.exit.up() {
                nodes.push((tick, self.exit.clone()))
            }
        }

        nodes.iter().enumerate().map(|(index, (tick, point))| (index, *tick, *point)).collect()
    }

    fn find_shortest_path(&self) -> usize {
        let nodes = self.nodes();
        let node_count = nodes.len();
        dbg!(node_count, self.open_rooms_through_time.len());
        let mut shortest_paths : Vec<usize> = vec![usize::MAX; node_count];
        shortest_paths[0] = 0;

        let mut to_visit = nodes.clone();
        let mut previous_nodes = vec![usize::MAX; node_count];

        let mut latest_tick = 0;
        while !to_visit.is_empty() {
            to_visit.sort_by_key(|(node, _tick, point)| {
                let distance = shortest_paths[*node];
                if distance == usize::MAX {
                    distance
                } else if point.eq(&self.entrance) {
                    (distance * 2) + 1
                } else {
                    distance * 2
                }
            });
            to_visit.reverse();

            if to_visit.iter().all(|(node, _tick, _point)| shortest_paths[*node] == usize::MAX) {
                break;
            }

            let (node, tick, point) = to_visit.pop().unwrap();
            if tick > latest_tick {
                // if tick % 10 == 0 {
                    dbg!(tick);
                // }
                latest_tick = tick;
            }
            let distance = shortest_paths[node];
            if shortest_paths[node] == usize::MAX {
                self.print(tick, &point);
                dbg!(node, point);
                panic!("Unexpected visit to infinite node")
            }
            let neighbor_distance = distance + 1;
            let neighbor_tick = tick + 1;

            // dbg!(node, tick, distance, point.clone());

            for neighbor in nodes.iter().filter(|(_node, tick, _point)| *tick == neighbor_tick) {
                let (_neighbor_node, _neighbor_tick, _neighbor_point) = neighbor;
                let neighbor_node = *_neighbor_node;
                let neighbor_point = *_neighbor_point;

                if !(neighbor_point == point || neighbor_point.is_neighbor_of(point)) {
                    // println!("Rejecting neighbor:");
                    // dbg!(neighbor);
                    continue;
                }

                // println!("Processing neighbor:");
                // dbg!(neighbor);

                if to_visit.contains(neighbor) {
                    if neighbor_distance < shortest_paths[neighbor_node] {
                        shortest_paths[neighbor_node] = neighbor_distance;
                        previous_nodes[neighbor_node] = node;
                    }
                }
            }
        }

        // dbg!(&shortest_paths);

        let paths = shortest_paths.iter().enumerate().filter_map(|(node, _distance)| {
            let distance = *_distance;
            if distance == usize::MAX {
                None
            } else {
                let (_node, _tick, point) = nodes[node];
                if point == self.exit {
                    Some((node, distance))
                } else {
                    None
                }
            }
        });
        // dbg!(&paths.clone().collect::<Vec<(usize, usize)>>());
        let (_final_node, distance) = paths.min_by_key(|(_node, distance)| *distance).unwrap();
        // let mut path_nodes : Vec<usize> = vec![];
        // path_nodes.push(final_node);
        // let mut previous = previous_nodes[final_node];
        // path_nodes.push(previous);
        // while previous > 0 {
        //     previous = previous_nodes[previous];
        //     path_nodes.push(previous);
        // }
        // for &node in path_nodes.iter().rev() {
        //     let (_node, tick, point) = nodes[node];
        //     self.print(tick, &point);
        // }
        distance
    }

    fn print(&self, tick: usize, current_point: &Point) {
        print!("#");
        for column in 0..self.interior_dimensions.x {
            let point = Point { x: column, y: -1 };
            if point.eq(current_point) {
                print!("E");
            } else if column == 0 {
                print!(".");
            } else {
                print!("#");
            }
        }
        println!("#");

        for row in 0..self.interior_dimensions.y {
            print!("#");
            for column in 0..self.interior_dimensions.x {
                let point = Point { x: column, y: row };
                let blizzards : Vec<&Blizzard> = self.blizzards.iter().filter(|blizzard| blizzard.is_forecast(tick, &point, &self.interior_dimensions)).collect();
                match blizzards.len() {
                    0 => {
                        if point.eq(current_point) {
                            print!("E");
                        } else {
                            print!(".")
                        }
                    },
                    1 => print!("{}", blizzards[0].to_string()),
                    n => print!("{n}")
                }
            }
            println!("#");
        }

        print!("#");
        for column in 0..self.interior_dimensions.x {
            let point = Point { x: column, y: self.exit.y };
            if point.eq(current_point) {
                print!("E");
            } else if column == self.exit.x {
                print!(".");
            } else {
                print!("#");
            }
        }
        println!("#");
        println!();
    }
}

#[cfg(test)]
mod tests {
    use std::fs;
    use super::*;

    #[test]
    fn part_one_example() {
        let valley = Valley::new(fs::read_to_string("example_input.txt").unwrap().as_str());

        assert_eq!(valley.find_shortest_path(), 18);
    }

    #[test]
    fn part_one() {
        let valley = Valley::new(fs::read_to_string("input.txt").unwrap().as_str());

        assert_eq!(valley.find_shortest_path(), 18);
    }
}
