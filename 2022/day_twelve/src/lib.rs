type Altitude = u8;

#[derive(Clone, Eq, PartialEq, Debug)]
struct Point { x: usize, y: usize }

impl Point {
    fn left(&self) -> Self {
        Point { x: self.x - 1, y: self.y }
    }

    fn right(&self) -> Self {
        Point { x: self.x + 1, y: self.y }
    }

    fn up(&self) -> Self {
        Point { x: self.x, y: self.y - 1 }
    }

    fn down(&self) -> Self {
        Point { x: self.x, y: self.y + 1 }
    }
}

struct HeightMap {
    data: Vec<Vec<Altitude>>,
    nodes: Vec<usize>,
    total_nodes: usize,
    start_position: Point,
    end_position: Point
}

impl HeightMap {
    fn new(input: &str) -> Self {
        let mut start_position = Point { x: 0, y: 0 };
        let mut end_position = Point { x: 0, y: 0 };
        let data : Vec<Vec<Altitude>> = input.lines().enumerate().map( |(row, line)| {
            line.chars().enumerate().map(|(column, letter)| {
                match letter {
                    'S' => {
                        start_position = Point { x: column, y: row };
                        0
                    },
                    'E' => {
                        end_position = Point { x: column, y: row };
                        25
                    },
                    'a'..='z' => {
                        (letter as u8) - ('a' as u8)
                    },
                    _ => panic!("unexpected character {letter}")
                }
            }).collect()
        }).collect();

        let total_nodes = data.len() * data[0].len();
        let nodes : Vec<usize> = vec![usize::MAX; total_nodes];


        Self { data, start_position, end_position, nodes, total_nodes }
    }

    fn start_node(&self) -> usize {
        self.start_position.y * self.width() + self.start_position.x
    }

    fn end_node(&self) -> usize {
        self.end_position.y * self.width() + self.end_position.x
    }

    fn part_one_path_length(&self) -> usize {
        self.path_length_to(self.start_node())
    }

    fn part_two_path_length(&self) -> usize {
        let possible_paths : Vec<(usize, usize)> = self.possible_start_nodes().iter().map(|&start_node| (start_node, self.path_length_to(start_node))).collect();

        // dbg!(&possible_paths);

        possible_paths.iter().map(|(_start_node, length)| *length).min().unwrap()
    }

    #[allow(dead_code)]
    fn possible_start_nodes(&self) -> Vec<usize> {
        let mut nodes_to_start_from : Vec<usize> = vec![];

        for (row, cells) in self.data.iter().enumerate() {
            for (column, &height) in cells.iter().enumerate() {
                if height == 0 {
                    let point = Point { x: column, y: row };
                    nodes_to_start_from.push(self.node_for_point(&point));
                }
            }
        }

        nodes_to_start_from
    }

    fn path_length_to(&self, to: usize) -> usize {
        self.nodes[to]
    }

    fn set_path_length_to(&mut self, to: usize, distance: usize) {
        self.nodes[to] = distance;
    }

    fn point_for_node(&self, node: usize) -> Point {
        Point { x: node % self.width(), y: node / self.width() }
    }

    fn node_for_point(&self, point: &Point) -> usize {
        point.y * self.width() + point.x
    }

    fn calculate_paths(&mut self) {
        self.calculate_path(self.end_node());
    }

    fn calculate_path(&mut self, start_node: usize) {
        self.nodes[start_node] = 0;

        // self.print();

        let mut to_visit : Vec<(usize, Point)> = (0..self.total_nodes).map(|node| (node, self.point_for_node(node))).collect();
        let mut previous_nodes = vec![usize::MAX; self.total_nodes];

        while !to_visit.is_empty() {
            to_visit.sort_by_key(|(node, _point)| self.path_length_to(*node));
            to_visit.reverse();

            if to_visit.iter().all(|(node, _point)| self.path_length_to(*node) == usize::MAX) {
                break;
            }

            let (node, point) = to_visit.pop().unwrap();
            if self.path_length_to(node) == usize::MAX {
                self.print();
                dbg!(node, point);
                panic!("Unexpected visit to infinite node")
            }

            // dbg!(node, point.clone());

            for neighbor in self.neighbors(point) {
                let (neighbor_node, _neighbor_point) = neighbor.clone();

                // dbg!(neighbor.clone());

                if to_visit.contains(&neighbor) {
                    let new_distance = self.path_length_to(node) + 1;
                    if new_distance < self.path_length_to(neighbor_node) {
                        self.set_path_length_to(neighbor_node, new_distance);
                        previous_nodes[neighbor_node] = node;
                    }
                }
            }
        }
    }

    #[allow(dead_code)]
    fn print(&self) {
        for (row, cells) in self.data.iter().enumerate() {
            for (column, _) in cells.iter().enumerate() {
                let point = Point { x: column, y: row };
                let distance = self.path_length_to(self.node_for_point(&point));
                if distance == usize::MAX {
                    print!("   ");
                } else {
                    print!("{:3}", distance);
                }
            }
            println!();
            for (column, &height) in cells.iter().enumerate() {
                let point = Point { x: column, y: row };
                let _node = self.node_for_point(&point);
                if self.start_position == point {
                    print!(" S ");
                } else if self.end_position == point {
                    print!(" E ");
                } else {
                    print!(" {} ", (height + ('a' as Altitude)) as char)
                }
            }
            println!();
        }
        println!();
    }

    fn width(&self) -> usize {
        self.data[0].len()
    }

    fn height(&self) -> usize {
        self.data.len()
    }

    fn height_for_point(&self, point: &Point) -> Altitude {
        self.data[point.y][point.x]
    }

    fn can_climb(&self, from: &Point, to: &Point) -> bool {
        (self.height_for_point(from) as i32) - (self.height_for_point(to) as i32) < 2
    }

    fn neighbors(&self, point: Point) -> Vec<(usize, Point)> {
        let mut neighbors = vec![];

        if point.x > 0 {
            let possible_neighbor = point.left();
            if self.can_climb(&point, &possible_neighbor) {
                neighbors.push((self.node_for_point(&possible_neighbor), possible_neighbor));
            }
        }
        if point.x < self.width() - 1 {
            let possible_neighbor = point.right();
            if self.can_climb(&point, &possible_neighbor) {
                neighbors.push((self.node_for_point(&possible_neighbor), possible_neighbor));
            }
        }
        if point.y > 0 {
            let possible_neighbor = point.up();
            if self.can_climb(&point, &possible_neighbor) {
                neighbors.push((self.node_for_point(&possible_neighbor), possible_neighbor));
            }
        }
        if point.y < self.height() - 1 {
            let possible_neighbor = point.down();
            if self.can_climb(&point, &possible_neighbor) {
                neighbors.push((self.node_for_point(&possible_neighbor), possible_neighbor));
            }
        }

        neighbors
    }
}

#[cfg(test)]
mod tests {
    use std::fs;
    use super::*;

    #[test]
    fn part_one_example() {
        let mut height_map = HeightMap::new(fs::read_to_string("example_input.txt").unwrap().as_str());
        height_map.calculate_paths();
        assert_eq!(
            height_map.part_one_path_length(),
            31
        );
    }

    #[test]
    fn part_one() {
        let mut height_map = HeightMap::new(fs::read_to_string("input.txt").unwrap().as_str());
        height_map.calculate_paths();
        assert_eq!(
            height_map.part_one_path_length(),
            456
        );
    }

    #[test]
    fn part_two_example() {
        let mut height_map = HeightMap::new(fs::read_to_string("example_input.txt").unwrap().as_str());
        height_map.calculate_paths();
        assert_eq!(
            height_map.part_two_path_length(),
            29
        );
    }

    #[test]
    fn part_two() {
        let mut height_map = HeightMap::new(fs::read_to_string("input.txt").unwrap().as_str());
        height_map.calculate_paths();
        assert_eq!(
            height_map.part_two_path_length(),
            454
        );
    }
}
