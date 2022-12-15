use std::cmp::{max, min};
use std::ops::RangeInclusive;

type NumberRange = RangeInclusive<i32>;

fn apply_mask_to_range(initial: NumberRange, mask: NumberRange) -> Option<Vec<NumberRange>> {
    if mask.start() <= initial.start() && initial.end() <= mask.end()  {
        return None;
    }
    if mask.end() < initial.start() {
        return Some(vec![initial]);
    }
    if initial.end() < mask.start() {
        return Some(vec![initial]);
    }

    if initial.start() < mask.start() {
        let beginning_range = *initial.start()..=(mask.start() - 1);
        if mask.end() < initial.end() {
            let end_range = (mask.end() + 1)..=*initial.end();
            Some(vec![beginning_range, end_range])
        } else {
            Some(vec![beginning_range])
        }
    } else {
        if mask.end() < initial.end() {
            let end_range = (mask.end() + 1)..=*initial.end();
            Some(vec![end_range])
        } else {
            panic!("Shouldn't happen?");
        }
    }
}

fn apply_masks_to_range(initial: NumberRange, masks: Vec<NumberRange>) -> Vec<NumberRange> {
    let mut uncovered_ranges : Vec<NumberRange> = vec![initial];
    for mask in masks.iter() {
        uncovered_ranges = uncovered_ranges.iter().filter_map(|uncovered_range| {
            apply_mask_to_range(uncovered_range.clone(), mask.clone())
        }).flatten().collect()
    }
    uncovered_ranges
}

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
struct Point {
    x:  i32,
    y: i32
}

impl Point {
    fn new(input: &str) -> Self {
        let values : Vec<i32> =
            input.split(", ").map(|half| half.rsplit("=").next().unwrap().parse::<i32>().unwrap()).collect();

        Self { x: values[0], y: values[1] }
    }

    fn manhattan_distance_to(&self, other: &Point) -> i32 {
        (self.x - other.x).abs() + (self.y - other.y).abs()
    }
}

struct Sensor {
    location: Point,
    nearest_beacon: Point,
    distance_to_beacon: i32,
    top: i32,
    bottom: i32,
    left: i32,
    right: i32
}

impl Sensor {
    fn new(input: &str) -> Self {
        let halves : Vec<Point> =
            input.split(": ").map(|half| Point::new(half.rsplit("at ").next().unwrap())).collect();

        let location = halves[0];
        let nearest_beacon = halves[1];
        let distance_to_beacon = location.manhattan_distance_to(&nearest_beacon);
        let top = location.y - distance_to_beacon;
        let bottom = location.y + distance_to_beacon;
        let left = location.x - distance_to_beacon;
        let right = location.x + distance_to_beacon;

        Self { location, nearest_beacon, distance_to_beacon, top, bottom, left, right }
    }

    fn covers(&self, point: &Point) -> bool {
        self.location.manhattan_distance_to(point) <= self.distance_to_beacon
    }

    fn dimensions(&self) -> (Point, Point) {
        (Point { x: self.left, y: self.top }, Point { x: self.right, y: self.bottom })
    }

    fn coverage_at_row(&self, row: i32) -> Option<NumberRange> {
        if row < self.top || row > self.bottom {
            return None;
        }

        if row == self.location.y {
            return Some(self.left..=self.right);
        }

        let diff = (self.location.y - row).abs();

        Some((self.left + diff)..=(self.right - diff))
    }
}

enum MapReading {
    Sensor(usize),
    Beacon(usize),
    CoveredBySensor(usize),
    Open
}

struct BeaconSensorArray {
    sensors: Vec<Sensor>
}

impl BeaconSensorArray {
    fn new(input: &str) -> Self {
        Self { sensors: input.lines().map(Sensor::new).collect() }
    }

    fn distress_beacon_frequency(&self, max: i32) -> usize {
        let full_range = 0..=max;

        for row in full_range.clone() {
            if let Some(column) = self.check_row_for_distress_beacon(row, full_range.clone()) {
                return (column as usize * 4_000_000) + row as usize;
            }
        }

        panic!("No distress beacon location possible!")
    }

    fn check_row_for_distress_beacon(&self, row: i32, full_range: NumberRange) -> Option<i32> {
        self.find_solitary_uncovered_range_for_row(row, full_range).and_then(|target_range| {
            Some(*target_range.start())
        })
    }

    fn find_solitary_uncovered_range_for_row(&self, row: i32, full_range: NumberRange) -> Option<NumberRange> {
        self.find_possible_uncovered_ranges_for_row(row, full_range).and_then(|uncovered_ranges| {
            let target_range = uncovered_ranges.first().unwrap();
            if target_range.start() != target_range.end() {
                panic!("Too large of range!");
            }

            Some(target_range.to_owned())
        })
    }

    fn find_possible_uncovered_ranges_for_row(&self, row: i32, full_range: NumberRange) -> Option<Vec<NumberRange>> {
        let uncovered_ranges = self.find_all_uncovered_ranges_for_row(row, full_range);
        if uncovered_ranges.is_empty() {
            return None;
        }

        if uncovered_ranges.len() > 1 {
            dbg!(&uncovered_ranges);
            panic!("Too many ranges!");
        }

        Some(uncovered_ranges)
    }

    fn find_all_uncovered_ranges_for_row(&self, row: i32, full_range: NumberRange) -> Vec<NumberRange> {
        let coverage_ranges = self.sensor_coverage_ranges_for_row(row);
        if coverage_ranges.is_empty() {
            panic!("No sensor coverage detected on row {row}...");
        }
        apply_masks_to_range(full_range, coverage_ranges)
    }

    fn sensor_coverage_ranges_for_row(&self, row: i32) -> Vec<NumberRange> {
        self.sensors.iter().filter_map(|sensor| sensor.coverage_at_row(row as i32)).collect()
    }

    fn beacon_exclusions_at_row(&self, row: i32) -> usize {
        let dimensions = self.dimensions();
        let mut count = 0;

        for column in dimensions.0.x..=dimensions.1.x {
            if matches!(self.sensor_coverage_at(&Point { x: column, y: row }), MapReading::CoveredBySensor(_)) {
                count += 1;
            }
        }

        count
    }

    fn dimensions(&self) -> (Point, Point) {
        let mut min_x : i32 = 0;
        let mut min_y : i32 = 0;
        let mut max_x : i32 = 0;
        let mut max_y : i32 = 0;

        for sensor in self.sensors.iter() {
            let (top_left, bottom_right) = sensor.dimensions();

            min_y = min(min_y, top_left.y);
            max_y = max(max_y, bottom_right.y);
            min_x = min(min_x, top_left.x);
            max_x = max(max_x, bottom_right.x);
        }

        ( Point { x: min_x, y: min_y }, Point { x: max_x, y: max_y } )
    }

    fn sensor_coverage_at(&self, point: &Point) -> MapReading {
        for (index, sensor) in self.sensors.iter().enumerate() {
            if sensor.location == *point {
                return MapReading::Sensor(index);
            }

            if sensor.nearest_beacon == *point {
                return MapReading::Beacon(index);
            }

            if sensor.covers(point) {
                return MapReading::CoveredBySensor(index);
            }
        }

        MapReading::Open
    }

    fn print(&self) {
        let (top_left, bottom_right) = self.dimensions();

        print!("    ");
        for column in top_left.x..=bottom_right.x {
            print!("{:^3} ", column);
        }
        println!();

        for row in top_left.y..=bottom_right.y {
            print!("{:>3} ", row);
            for column in top_left.x..=bottom_right.x {
                let point = Point { x: column, y: row };
                let mut has_printed = false;
                for (index, sensor) in self.sensors.iter().enumerate() {
                    if sensor.nearest_beacon == point {
                        print!("B{:<2} ", index + 1);
                        has_printed = true;
                        break;
                    }

                    if sensor.location == point {
                        print!("S{:<2} ", index + 1);
                        has_printed = true;
                        break;
                    }

                    if sensor.covers(&point) {
                        print!("#{:<2} ", index + 1);
                        has_printed = true;
                        break;
                    }
                }

                if !has_printed {
                    print!(" .. ");
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
        let array = BeaconSensorArray::new(fs::read_to_string("example_input.txt").unwrap().as_str());
        array.print();
        assert_eq!(array.beacon_exclusions_at_row(10), 26);
    }

    #[test]
    fn part_one() {
        let array = BeaconSensorArray::new(fs::read_to_string("input.txt").unwrap().as_str());
        assert_eq!(array.beacon_exclusions_at_row(2_000_000), 5_256_611);
    }

    #[test]
    fn part_two_example() {
        let array = BeaconSensorArray::new(fs::read_to_string("example_input.txt").unwrap().as_str());
        array.print();
        assert_eq!(array.distress_beacon_frequency(20), 56_000_011);
    }

    #[test]
    fn part_two() {
        let array = BeaconSensorArray::new(fs::read_to_string("input.txt").unwrap().as_str());
        assert_eq!(array.distress_beacon_frequency(4_000_000), 13_337_919_186_981);
    }
}
