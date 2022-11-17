use std::collections::HashMap;

type Signal = u16;

#[derive(Copy, Clone)]
enum Component<'a> {
    Value { signal: Signal },
    Source { source: &'a str },
    AndGate { left: &'a str, right: &'a str },
    OrGate { left: &'a str, right: &'a str },
    ComplimentGate { source: &'a str },
    LeftShiftGate { source: &'a str, shift: Signal },
    RightShiftGate { source: &'a str, shift: Signal },
}

struct Circuit<'a> {
    components: HashMap<&'a str, Component<'a>>
}

impl<'a> Circuit<'a> {
    fn new() -> Self {
        Self { components: HashMap::new() }
    }

    fn add_component(&mut self, input: &'a str) -> &mut Self {
        let mut parts = input.split(" ").collect::<Vec<&str>>();
        let name = parts.pop().unwrap();
        parts.pop(); // ->
        let right = parts.pop().unwrap();
        match parts.len() {
            0 => {
                self.components.insert(name, Component::Source { source: right });
            },
            1 => {
                self.components.insert(name, Component::ComplimentGate { source: right });
            },
            2 => {
                let operator = parts.pop().unwrap();
                let left = parts.pop().unwrap();
                match operator {
                    "AND" => { self.components.insert(name, Component::AndGate { left, right }); },
                    "OR" => { self.components.insert(name, Component::OrGate { left, right }); },
                    "LSHIFT" => {
                        self.components.insert(
                            name,
                            Component::LeftShiftGate { source: left, shift: right.parse::<Signal>().unwrap() }
                        );
                    },
                    "RSHIFT" => {
                        self.components.insert(
                            name,
                            Component::RightShiftGate { source: left, shift: right.parse::<Signal>().unwrap() }
                        );
                    },
                    x => panic!("Unknown operator: {x}")
                }
            },
            _ => panic!("Invalid input: {input}")
        }

        self
    }

    fn signal_on(&mut self, wire: &'a str) -> Signal {
        if let Ok(signal) = wire.parse::<Signal>() {
            return signal;
        }

        let component = self.components.get(wire).unwrap_or_else(|| panic!("Unknown wire: {wire}")).clone();
        let new_value : Signal =
            match component {
                Component::Value { signal } => { return signal },
                Component::Source { source } => { self.signal_on(source) },
                Component::AndGate { left, right } => {
                    self.signal_on(left) & self.signal_on(right)
                },
                Component::OrGate { left, right } => {
                    self.signal_on(left) | self.signal_on(right)
                },
                Component::ComplimentGate { source } => {
                    !self.signal_on(source)
                },
                Component::LeftShiftGate { source, shift } => {
                    self.signal_on(source) << shift
                },
                Component::RightShiftGate { source, shift } => {
                    self.signal_on(source) >> shift
                },
            };

        self.components.insert(wire, Component::Value { signal: new_value });

        new_value
    }
}

#[cfg(test)]
mod tests {
    use std::fs;
    use super::*;

    #[test]
    fn source() {
        assert_eq!(
            Circuit::new()
                .add_component("123 -> x")
                .signal_on("x"),
            123
        );
    }

    #[test]
    fn not() {
        assert_eq!(
            Circuit::new()
                .add_component("1 -> x")
                .add_component("NOT x -> y")
                .signal_on("y"),
            65534
        );
    }

    #[test]
    fn double_not() {
        assert_eq!(
            Circuit::new()
                .add_component("123 -> x")
                .add_component("NOT x -> y")
                .add_component("NOT y -> z")
                .signal_on("z"),
            123
        );
    }

    #[test]
    fn or() {
        assert_eq!(
            Circuit::new()
                .add_component("65280 -> x")
                .add_component("255 -> y")
                .add_component("x OR y -> z")
                .signal_on("z"),
            65535
        );
    }

    #[test]
    fn and() {
        assert_eq!(
            Circuit::new()
                .add_component("65520 -> x")
                .add_component("4095 -> y")
                .add_component("x AND y -> z")
                .signal_on("z"),
            4080
        );
    }

    #[test]
    fn left_shift() {
        assert_eq!(
            Circuit::new()
                .add_component("60 -> x")
                .add_component("x LSHIFT 2 -> z")
                .signal_on("z"),
            240
        );
    }

    #[test]
    fn left_shift_full() {
        assert_eq!(
            Circuit::new()
                .add_component("65535 -> x")
                .add_component("x LSHIFT 2 -> z")
                .signal_on("z"),
            65532
        );
    }

    #[test]
    fn right_shift() {
        assert_eq!(
            Circuit::new()
                .add_component("240 -> x")
                .add_component("x RSHIFT 2 -> z")
                .signal_on("z"),
            60
        );
    }

    #[test]
    fn right_shift_full() {
        assert_eq!(
            Circuit::new()
                .add_component("65535 -> x")
                .add_component("x RSHIFT 2 -> z")
                .signal_on("z"),
            16383
        );
    }

    #[test]
    fn part_one_example() {
        let input = "123 -> x
456 -> y
x AND y -> d
x OR y -> e
x LSHIFT 2 -> f
y RSHIFT 2 -> g
NOT x -> h
NOT y -> i";

        let mut circuit = Circuit::new();
        for line in input.lines() {
            circuit.add_component(line);
        }

        assert_eq!(circuit.signal_on("d"), 72);
        assert_eq!(circuit.signal_on("e"), 507);
        assert_eq!(circuit.signal_on("f"), 492);
        assert_eq!(circuit.signal_on("g"), 114);
        assert_eq!(circuit.signal_on("h"), 65412);
        assert_eq!(circuit.signal_on("i"), 65079);
        assert_eq!(circuit.signal_on("x"), 123);
        assert_eq!(circuit.signal_on("y"), 456);
    }

    #[test]
    fn part_one() {
        let input = fs::read_to_string("input.txt").unwrap();
        let mut circuit = Circuit::new();
        for line in input.lines() {
            circuit.add_component(line);
        }
        assert_eq!(circuit.signal_on("a"), 16076);
    }

    #[test]
    fn part_two() {
        let input = fs::read_to_string("input.txt").unwrap();
        let mut circuit = Circuit::new();
        for line in input.lines() {
            circuit.add_component(line);
        }
        circuit.add_component("16076 -> b");
        assert_eq!(circuit.signal_on("a"), 2797);
    }
}
