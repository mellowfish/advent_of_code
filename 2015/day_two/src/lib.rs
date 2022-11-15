struct Present {
    length: u32,
    width: u32,
    height: u32,
}

impl Present {
    fn new(input: &str) -> Self {
        let parts : Vec<u32> = input.split("x")
            .map(|part| part.parse::<u32>().unwrap())
            .collect();

        Self {
            length: parts[0],
            width: parts[1],
            height: parts[2],
        }
    }

    fn volume(&self) -> u32 {
        self.width * self.length * self.height
    }

    fn side_areas(&self) -> Vec<u32> {
        vec![
            self.length * self.width,
            self.width * self.height,
            self.length * self.height
        ]
    }

    fn side_perimeters(&self) -> Vec<u32> {
        vec![
            2 * (self.length + self.width),
            2 * (self.width + self.height),
            2 * (self.length + self.height)
        ]
    }

    fn total_wrapping_paper(&self) -> u32 {
        let side_areas = self.side_areas();

        let base_area : u32 = side_areas.iter().map(|area| area * 2).sum();
        let smallest_side = side_areas.iter().min().unwrap();

        base_area + smallest_side
    }

    fn total_ribbon(&self) -> u32 {
        let smallest_perimeter : u32 = *self.side_perimeters().iter().min().unwrap();

        smallest_perimeter + self.volume()
    }
}

#[cfg(test)]
mod tests {
    use std::fs;
    use crate::Present;

    fn presents() -> Vec<Present> {
        fs::read_to_string("input.txt")
            .unwrap()
            .lines()
            .map(|line| Present::new(line))
            .collect()
    }

    #[test]
    fn part_one_examples() {
        assert_eq!(Present::new("2x3x4").total_wrapping_paper(), 58);
        assert_eq!(Present::new("1x1x10").total_wrapping_paper(), 43);
    }

    #[test]
    fn part_one() {

        let total_paper : u32 =
            presents()
                .iter()
                .map(|present| present.total_wrapping_paper())
                .sum();

        assert_eq!(total_paper, 1586300);
    }

    #[test]
    fn part_two_examples() {
        assert_eq!(Present::new("2x3x4").total_ribbon(), 34);
        assert_eq!(Present::new("1x1x10").total_ribbon(), 14);
    }

    #[test]
    fn part_two() {
        let total_ribbon : u32 =
            presents()
                .iter()
                .map(|present| present.total_ribbon())
                .sum();

        assert_eq!(total_ribbon, 3737498);
    }
}
