use crate::Action::{Toggle, TurnOff, TurnOn};

type BitTransformer = fn(u8, usize) -> u8;
type ByteTransformer = fn(u8) -> u8;

enum Action {
    TurnOn,
    TurnOff,
    Toggle
}

impl Action {
    fn new(phrase : &str) -> Self {
        match phrase {
            "turn on" => TurnOn,
            "turn off" => TurnOff,
            "toggle" => Toggle,
            _ => panic!("Invalid action: {phrase}")
        }
    }

    fn to_bit_transformer(&self) -> BitTransformer {
        match self {
            TurnOn => |byte, bit_index| byte | util::bitmask(bit_index),
            TurnOff => |byte, bit_index| byte & util::inverse_bitmask(bit_index),
            Toggle => |byte, bit_index| byte ^ util::bitmask(bit_index)
        }
    }

    fn to_byte_transformer(&self) -> ByteTransformer {
        match self {
            TurnOn => |byte| byte + 1,
            TurnOff => |byte| match byte { 0 => 0, _ => byte - 1 },
            Toggle => |byte| byte + 2
        }
    }
}

struct Point {
    x: usize,
    y: usize
}

impl Point {
    fn new(comma_pair : &str) -> Self {
        let parts : Vec<usize> =
            comma_pair.split(",").map(|part| part.parse::<usize>().unwrap()).collect();

        Self {
            x: parts[0],
            y: parts[1]
        }
    }
}

struct Instruction {
    action: Action,
    from: Point,
    to: Point
}

impl Instruction {
    fn new(input : &str) -> Self {
        let mut parts = input.split(" ").collect::<Vec<&str>>();
        let to = Point::new(parts.pop().unwrap());
        parts.pop(); // through
        let from = Point::new(parts.pop().unwrap());
        let action = Action::new(parts.join(" ").as_str());

        Self { action, to, from }
    }
}

mod util {
    pub fn bitmask(index : usize) -> u8 {
        1 << index
    }

    pub fn inverse_bitmask(index: usize) -> u8 {
        bitmask(index) ^ 0b1111_1111
    }

    pub fn bits_on(byte : u8) -> u32 {
        byte.count_ones()
    }
}

struct BinaryLightingArray {
    lights: [[u8; 125]; 1000]
}

impl BinaryLightingArray {
    fn new() -> Self {
        Self { lights: [[0; 125]; 1000] }
    }

    fn map_bits(&mut self, from: Point, to: Point, transform: BitTransformer) {
        for row in from.y..=to.y {
            for column in from.x..=to.x {
                let byte_index = column / 8;
                let bit_index = column % 8;

                self.lights[row][byte_index] = transform(self.lights[row][byte_index], bit_index)
            }
        }
    }

    fn execute(&mut self, instruction : Instruction) {
        self.map_bits(instruction.from, instruction.to, instruction.action.to_bit_transformer())
    }

    fn count_lights_on(&self) -> u32 {
        let mut count = 0;

        for row in 0..1000 {
            for byte_index in 0..125 {
                count += util::bits_on(self.lights[row][byte_index]);
            }
        }

        count
    }
}

struct DimmableLightingArray {
    lights: [[u8; 1000]; 1000]
}

impl DimmableLightingArray {
    fn new() -> Self {
        Self { lights: [[0; 1000]; 1000] }
    }

    fn map_lights(&mut self, from: Point, to: Point, transform: ByteTransformer) {
        for row in from.y..=to.y {
            for column in from.x..=to.x {
                self.lights[row][column] = transform(self.lights[row][column])
            }
        }
    }

    fn execute(&mut self, instruction : Instruction) {
        self.map_lights(instruction.from, instruction.to, instruction.action.to_byte_transformer())
    }

    fn total_brightness(&self) -> u32 {
        let mut sum : u32 = 0;

        for row in 0..1000 {
            for byte_index in 0..1000 {
                sum += self.lights[row][byte_index] as u32;
            }
        }

        sum
    }
}

#[cfg(test)]
mod tests {
    use std::fs;
    use super::*;

    #[test]
    fn part_one_examples() {
        let mut array = BinaryLightingArray::new();
        array.execute(Instruction::new("turn on 0,0 through 999,999"));
        assert_eq!(array.count_lights_on(), 1_000_000);
        array.execute(Instruction::new("toggle 0,0 through 999,0"));
        assert_eq!(array.count_lights_on(), 999_000);
        array.execute(Instruction::new("turn off 499,499 through 500,500"));
        assert_eq!(array.count_lights_on(), 998_996);
    }

    #[test]
    fn part_one() {
        let mut array = BinaryLightingArray::new();
        for line in fs::read_to_string("input.txt").unwrap().lines().filter(|line| !line.is_empty()) {
            array.execute(Instruction::new(line));
        }
        assert_eq!(array.count_lights_on(), 543903);
    }

    #[test]
    fn part_two_examples() {
        let mut array = DimmableLightingArray::new();
        array.execute(Instruction::new("turn on 0,0 through 0,0"));
        assert_eq!(array.total_brightness(), 1);
        array.execute(Instruction::new("toggle 0,0 through 999,999"));
        assert_eq!(array.total_brightness(), 2_000_001);
    }

    #[test]
    fn part_two() {
        let mut array = DimmableLightingArray::new();
        for line in fs::read_to_string("input.txt").unwrap().lines().filter(|line| !line.is_empty()) {
            array.execute(Instruction::new(line));
        }
        assert_eq!(array.total_brightness(), 14_687_245);
    }
}
