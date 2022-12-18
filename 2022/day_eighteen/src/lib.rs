use std::cmp::{max, min};
use std::collections::{HashMap, HashSet};

#[derive(Clone, Eq, PartialEq, Hash)]
enum Material {
    Rock,
    Air,
    Steam
}

#[derive(Eq, PartialEq, Clone, Debug, Hash)]
struct Point {
    x: i32,
    y: i32,
    z: i32
}

impl Point {
    fn new(input: &str) -> Self {
        let parts : Vec<i32> = input.split(",").map(|number| number.parse::<i32>().unwrap()).collect();

        Self { x: parts[0], y: parts[1], z: parts[2] }
    }

    fn neighbors(&self) -> Vec<Point> {
        vec![self.up(), self.down(), self.left(), self.right(), self.forward(), self.back()]
    }

    fn up(&self) -> Point {
        Self { x: self.x, y: self.y + 1, z: self.z }
    }

    fn down(&self) -> Point {
        Self { x: self.x, y: self.y - 1, z: self.z }
    }

    fn left(&self) -> Point {
        Self { x: self.x - 1, y: self.y, z: self.z }
    }

    fn right(&self) -> Point {
        Self { x: self.x + 1, y: self.y, z: self.z }
    }

    fn forward(&self) -> Point {
        Self { x: self.x, y: self.y, z: self.z + 1 }
    }

    fn back(&self) -> Point {
        Self { x: self.x, y: self.y, z: self.z - 1 }
    }
}

struct DropletScan {
    points: HashMap<Point, Material>,
    top_right_forward: Point,
    bottom_left_back: Point
}

impl DropletScan {
    fn new(input: &str) -> Self {
        let points : HashMap<Point, Material> = input.lines().map(|line| (Point::new(line), Material::Rock)).collect();
        let (bottom_left_back, top_right_forward) = Self::dimensions(&points);

        let mut droplet = Self { points, top_right_forward, bottom_left_back };

        droplet.populate_air_and_steam();

        droplet
    }

    fn total_surface_area(&self) -> usize {
        let mut area = 0;

        for (point, material) in self.points.iter() {
            if !matches!(material, Material::Rock) {
                continue;
            }

            for neighbor in point.neighbors().iter() {
                if !matches!(self.points.get(neighbor).unwrap(), Material::Rock) {
                    area += 1
                }
            }
        }

        area
    }

    fn total_exterior_surface_area(&self) -> usize {
        let mut area = 0;

        for (point, material) in self.points.iter() {
            if !matches!(material, Material::Rock) {
                continue;
            }

            for neighbor in point.neighbors().iter() {
                if matches!(self.points.get(neighbor).unwrap(), Material::Steam) {
                    area += 1
                }
            }
        }

        area
    }

    fn populate_air_and_steam(&mut self) {
        let mut steamy_neighbors : HashSet<Point> = HashSet::new();

        for x in (self.bottom_left_back.x - 1)..=(self.top_right_forward.x + 1) {
            for y in (self.bottom_left_back.y - 1)..=(self.top_right_forward.y + 1) {
                for z in (self.bottom_left_back.z - 1)..=(self.top_right_forward.z + 1) {
                    let point = Point { x, y, z };
                    if !self.points.contains_key(&point) {
                        if self.is_in_steam_envelope(&point) {
                            self.points.insert(point.to_owned(), Material::Steam);
                            for neighbor in point.neighbors() {
                                if !self.is_in_steam_envelope(&neighbor) {
                                    steamy_neighbors.insert(neighbor);
                                }
                            }
                        } else {
                            self.points.insert(point.to_owned(), Material::Air);
                        }

                    }
                }
            }
        }

        while !steamy_neighbors.is_empty() {
            let steamy_neighbor = steamy_neighbors.iter().next().unwrap().clone();
            steamy_neighbors.remove(&steamy_neighbor);

            if matches!(self.points.get(&steamy_neighbor), Some(Material::Air)) {
                self.points.insert(steamy_neighbor.clone(), Material::Steam);
                for new_steamy_neighbor in steamy_neighbor.neighbors().iter() {
                    if matches!(self.points.get(new_steamy_neighbor), Some(Material::Air)) {
                        steamy_neighbors.insert(new_steamy_neighbor.clone());
                    }
                }
            }
        }
    }

    fn is_in_steam_envelope(&self, point: &Point) -> bool {
        point.x == self.bottom_left_back.x - 1
            || point.x == self.top_right_forward.x + 1
            || point.y == self.bottom_left_back.y - 1
            || point.y == self.top_right_forward.y + 1
            || point.z == self.bottom_left_back.z - 1
            || point.z == self.top_right_forward.z + 1
    }

    fn dimensions(points: &HashMap<Point, Material>) -> (Point, Point) {
        let mut min_x : i32 = 0;
        let mut min_y : i32 = 0;
        let mut min_z : i32 = 0;
        let mut max_x : i32 = 0;
        let mut max_y : i32 = 0;
        let mut max_z : i32 = 0;

        for point in points.keys() {
            min_y = min(min_y, point.y);
            max_y = max(max_y, point.y);
            min_x = min(min_x, point.x);
            max_x = max(max_x, point.x);
            min_z = min(min_z, point.z);
            max_z = max(max_z, point.z);
        }

        ( Point { x: min_x, y: min_y, z: min_z }, Point { x: max_x, y: max_y, z: max_z } )
    }
}

#[cfg(test)]
mod tests {
    use std::fs;
    use super::*;

    #[test]
    fn part_one_example() {
        assert_eq!(
            DropletScan::new(fs::read_to_string("example_input.txt").unwrap().as_str()).total_surface_area(),
            64
        );
    }

    #[test]
    fn part_one() {
        assert_eq!(
            DropletScan::new(fs::read_to_string("input.txt").unwrap().as_str()).total_surface_area(),
            4282
        );
    }

    #[test]
    fn part_two_example() {
        assert_eq!(
            DropletScan::new(fs::read_to_string("example_input.txt").unwrap().as_str()).total_exterior_surface_area(),
            58
        );
    }

    #[test]
    fn part_two() {
        assert_eq!(
            DropletScan::new(fs::read_to_string("input.txt").unwrap().as_str()).total_exterior_surface_area(),
            2452
        );
    }
}
