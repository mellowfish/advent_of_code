use std::collections::HashMap;

type MonkeyShout = i64;

#[derive(Clone, Debug)]
enum MonkeyBusiness {
    Adder { left: String, right: String },
    Multiplier { left: String, right: String },
    Divider { left: String, right: String },
    Subtracter { left: String, right: String },
    Constant { value: MonkeyShout }
}

impl MonkeyBusiness {
    fn new(input: &str) -> Self {
        let parts : Vec<&str> = input.split(" ").collect();
        if parts.len() == 1 {
            return MonkeyBusiness::Constant { value: parts[0].parse().unwrap() };
        }

        let left = parts[0].to_string();
        let right = parts[2].to_string();
        match parts[1] {
            "+" => { MonkeyBusiness::Adder { left, right } },
            "-" => { MonkeyBusiness::Subtracter { left, right } },
            "*" => { MonkeyBusiness::Multiplier { left, right } },
            "/" => { MonkeyBusiness::Divider { left, right } },
            _ => panic!("Unexpected operation: {input}")
        }
    }

    fn unwrap_parts(&self) -> (String, String) {
        match self {
            MonkeyBusiness::Constant { value: _ } => panic!("Tried to unwrap a constant"),
            MonkeyBusiness::Adder { left, right }
                | MonkeyBusiness::Subtracter { left, right }
                | MonkeyBusiness::Multiplier { left, right }
                | MonkeyBusiness::Divider { left, right }
                    => { (left.clone(), right.clone()) }
        }
    }

    fn invert(&self, left: String, right: String) -> MonkeyBusiness {
        match self {
            MonkeyBusiness::Constant { value: _ } => panic!("Tried to invert a constant"),
            MonkeyBusiness::Adder { left: _, right: _ } => { MonkeyBusiness::Subtracter { left, right } },
            MonkeyBusiness::Subtracter { left: _, right: _ } => { MonkeyBusiness::Adder { left, right } },
            MonkeyBusiness::Multiplier { left: _, right: _ } => { MonkeyBusiness::Divider { left, right } },
            MonkeyBusiness::Divider { left: _, right: _ } => { MonkeyBusiness::Multiplier { left, right } },
        }
    }

    fn with(&self, left: String, right: String) -> MonkeyBusiness {
        match self {
            MonkeyBusiness::Constant { value: _ } => panic!("Tried to split a constant"),
            MonkeyBusiness::Adder { left: _, right: _ } => { MonkeyBusiness::Adder { left, right } },
            MonkeyBusiness::Subtracter { left: _, right: _ } => { MonkeyBusiness::Subtracter { left, right } },
            MonkeyBusiness::Multiplier { left: _, right: _ } => { MonkeyBusiness::Multiplier { left, right } },
            MonkeyBusiness::Divider { left: _, right: _ } => { MonkeyBusiness::Divider { left, right } },
        }
    }
}

#[derive(Clone)]
struct Monkey {
    name: String,
    business: MonkeyBusiness,
    value: Option<MonkeyShout>
}

impl Monkey {
    fn new(input: &str) -> Self {
        let (name, operation) = input.split_once(": ").unwrap();
        let business = MonkeyBusiness::new(operation);

        Self { name: name.to_string(), business, value: None }
    }

    fn with_value(&self, value: MonkeyShout) -> Self {
        Self { name: self.name.clone(), business: self.business.clone(), value: Some(value) }
    }
}

struct Troop {
    monkey_definitions: HashMap<String, Monkey>
}

impl Troop {
    fn new(input: &str) -> Self {
        Self {
            monkey_definitions: input.lines().map(|line| {
                let monkey = Monkey::new(line);

                (monkey.name.clone(), monkey)
            }).collect()
        }
    }

    fn part_one_value(&mut self) -> MonkeyShout {
        self.evaluate(String::from("root"))
    }

    fn part_two_value(&mut self) -> MonkeyShout {
        loop {
            let root = self.monkey_definitions.get("root").unwrap();
            let (left_name, right_name) = root.business.unwrap_parts();
            if left_name.eq("humn") {
                return self.evaluate(right_name);
            }
            if right_name.eq("humn") {
                return self.evaluate(left_name);
            }
            if self.human_is_under(left_name) {
                // println!("Shifting right");
                self.rotate_right();
            } else {
                // println!("Shifting left");
                self.rotate_left();
            }
        }
    }

    fn root(&self) -> &Monkey {
        self.monkey_named(String::from("root"))
    }

    fn monkey_named(&self, name: String) -> &Monkey {
        self.monkey_definitions.get(name.as_str()).unwrap()
    }

    fn rotate_right(&mut self) {
        let (left_name, right_name) = self.root().business.unwrap_parts();
        let left_monkey = self.monkey_named(left_name.clone());

        self.rotate(left_monkey.business.clone(), left_name.clone(), right_name.clone());
    }

    fn rotate_left(&mut self) {
        let (left_name, right_name) = self.root().business.unwrap_parts();
        let right_monkey = self.monkey_named(right_name.clone());

        self.rotate(right_monkey.business.clone(), right_name.clone(), left_name.clone());
    }

    fn rotate(&mut self, from_business: MonkeyBusiness, from_name: String, to_name: String) {
        match from_business.clone() {
            MonkeyBusiness::Adder { left: left_left_name, right: left_right_name }
                | MonkeyBusiness::Multiplier { left: left_left_name, right: left_right_name }
                    => {
                        if self.human_is_under(left_left_name.clone()) {
                            self.redefine_monkey(
                                Monkey {
                                    name: from_name.clone(),
                                    business: from_business.invert(to_name.clone(), left_right_name.clone()),
                                    value: None
                                }
                            );

                            self.redefine_monkey(
                                Monkey {
                                    name: String::from("root"),
                                    business: MonkeyBusiness::Adder { left: left_left_name.clone(), right: from_name.clone() },
                                    value: None
                                }
                            );
                        } else {
                            self.redefine_monkey(
                                Monkey {
                                    name: from_name.clone(),
                                    business: from_business.invert(to_name.clone(), left_left_name.clone()),
                                    value: None
                                }
                            );

                            self.redefine_monkey(
                                Monkey {
                                    name: String::from("root"),
                                    business: MonkeyBusiness::Adder { left: left_right_name.clone(), right: from_name.clone() },
                                    value: None
                                }
                            );
                        }
                    },
            MonkeyBusiness::Subtracter { left: left_left_name, right: left_right_name }
                | MonkeyBusiness::Divider { left: left_left_name, right: left_right_name }
                    => {
                        if self.human_is_under(left_left_name.clone()) {
                            self.redefine_monkey(
                                Monkey {
                                    name: from_name.clone(),
                                    business: from_business.invert(to_name.clone(), left_right_name.clone()),
                                    value: None
                                }
                            );

                            self.redefine_monkey(
                                Monkey {
                                    name: String::from("root"),
                                    business: MonkeyBusiness::Adder { left: left_left_name.clone(), right: from_name.clone() },
                                    value: None
                                }
                            );
                        } else {
                            self.redefine_monkey(
                                Monkey {
                                    name: from_name.clone(),
                                    business: from_business.with(left_left_name.clone(), to_name.clone()),
                                    value: None
                                }
                            );

                            self.redefine_monkey(
                                Monkey {
                                    name: String::from("root"),
                                    business: MonkeyBusiness::Adder { left: left_right_name.clone(), right: from_name.clone() },
                                    value: None
                                }
                            );
                        }
                    }
            _ => panic!("Unexpected constant operation {}", from_name)
        }
    }

    fn redefine_monkey(&mut self, new_monkey: Monkey) {
        self.monkey_definitions.insert(new_monkey.name.clone(), new_monkey);
    }

    fn human_is_under(&self, name: String) -> bool {
        let monkey = self.monkey_definitions.get(name.as_str()).unwrap().clone();
        match monkey.business {
            MonkeyBusiness::Constant { value: _ } => { monkey.name.eq("humn") },
            MonkeyBusiness::Adder { left, right }
                | MonkeyBusiness::Subtracter { left, right }
                | MonkeyBusiness::Multiplier { left, right }
                | MonkeyBusiness::Divider { left, right }
                    => { self.human_is_under(left) || self.human_is_under(right) }
        }
    }

    fn evaluate(&mut self, name: String) -> MonkeyShout {
        let monkey = self.monkey_definitions.get(name.as_str()).unwrap().clone();
        if let Some(value) = monkey.value {
            return value;
        }
        let new_value =
            match monkey.business.clone() {
                MonkeyBusiness::Constant { value } => { value },
                MonkeyBusiness::Adder { left, right } => { self.evaluate(left) + self.evaluate(right) },
                MonkeyBusiness::Subtracter { left, right } => { self.evaluate(left) - self.evaluate(right) },
                MonkeyBusiness::Multiplier { left, right } => { self.evaluate(left) * self.evaluate(right) },
                MonkeyBusiness::Divider { left, right } => { self.evaluate(left) / self.evaluate(right) }
            };

        self.with_updated_monkey_value(monkey, new_value)
    }

    fn with_updated_monkey_value(&mut self, monkey: Monkey, value: MonkeyShout) -> MonkeyShout {
        self.monkey_definitions.insert(monkey.name.clone(), monkey.with_value(value));

        value
    }
}

#[cfg(test)]
mod tests {
    use std::fs;
    use super::*;

    #[test]
    fn part_one_example() {
        let mut troop = Troop::new(fs::read_to_string("example_input.txt").unwrap().as_str());
        assert_eq!(troop.part_one_value(), 152);
    }

    #[test]
    fn part_one() {
        let mut troop = Troop::new(fs::read_to_string("input.txt").unwrap().as_str());
        assert_eq!(troop.part_one_value(), 159_591_692_827_554);
    }

    #[test]
    fn part_two_example() {
        let mut troop = Troop::new(fs::read_to_string("example_input.txt").unwrap().as_str());
        assert_eq!(troop.part_two_value(), 301);
    }

    #[test]
    fn part_two() {
        let mut troop = Troop::new(fs::read_to_string("input.txt").unwrap().as_str());
        assert_eq!(troop.part_two_value(), 3_509_819_803_065);
    }
}
