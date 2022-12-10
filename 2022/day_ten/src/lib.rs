struct CPU {
    register_x_values: Vec<i32>
}

impl CPU {
    fn new() -> Self { Self { register_x_values: vec![1] } }

    fn run_program(&mut self, input: &str) {
        for line in input.lines() {
            let parts : Vec<&str> = line.split(" ").collect();
            match parts[0] {
                "noop" => { self.noop(); },
                "addx" => { self.add_x(parts[1].parse().unwrap()); },
                _ => panic!("Unknown instruction: {line}")
            }
        }
    }

    fn x_value_at(&self, cycle: usize) -> i32 {
        self.register_x_values[cycle - 1]
    }

    fn register_x(&self) -> i32 {
        *self.register_x_values.last().unwrap()
    }

    fn noop(&mut self) {
        self.register_x_values.push(self.register_x());
    }

    fn add_x(&mut self, value: i32) {
        self.register_x_values.push(self.register_x());
        self.register_x_values.push(self.register_x() + value);
    }

    fn signal_samples_over_time(&self) -> i32 {
        let mut cycle = 20;
        let mut total = 0;

        while cycle <= 220 {
            total += self.x_value_at(cycle) * (cycle as i32);
            cycle += 40;
        }

        total
    }

    fn print_to_crt(&self) -> String {
        let mut output = String::new();
        let mut pixel_index = 0;

        for &sprite_position in self.register_x_values.iter() {
            if sprite_position == pixel_index || sprite_position - 1 == pixel_index || sprite_position + 1 == pixel_index {
                output.push('#')
            } else {
                output.push('.')
            }

            if pixel_index == 39 {
                output.push('\n');
                pixel_index = 0;
            } else {
                pixel_index += 1;
            }
        }

        output
    }
}

#[cfg(test)]
mod tests {
    use std::fs;
    use super::*;

    #[test]
    fn part_one_example() {
        let mut cpu = CPU::new();
        cpu.run_program(fs::read_to_string("example_input.txt").unwrap().as_str());
        assert_eq!(cpu.signal_samples_over_time(), 13140);
    }

    #[test]
    fn part_one() {
        let mut cpu = CPU::new();
        cpu.run_program(fs::read_to_string("input.txt").unwrap().as_str());
        assert_eq!(cpu.signal_samples_over_time(), 11720);
    }

    #[test]
    fn part_two_example() {
        let mut cpu = CPU::new();
        cpu.run_program(fs::read_to_string("example_input.txt").unwrap().as_str());
        assert_eq!(
            cpu.print_to_crt(),
            fs::read_to_string("example_output.txt").unwrap().trim_end()
        );
    }

    #[test]
    fn part_two() {
        let mut cpu = CPU::new();
        cpu.run_program(fs::read_to_string("input.txt").unwrap().as_str());
        assert_eq!(
            cpu.print_to_crt(),
            fs::read_to_string("output.txt").unwrap().trim_end()
        );
    }
}
