use std::iter;

#[derive(Clone,Debug)]
struct MoveStep {
    count: usize,
    from: usize,
    to: usize
}

impl MoveStep {
    fn new(line: &str) -> Self {
        let parts: Vec<&str> = line.trim().split(" ").collect();

        MoveStep {
            count: parts[1].parse::<usize>().unwrap(),
            from: parts[3].parse::<usize>().unwrap(),
            to: parts[5].parse::<usize>().unwrap()
        }
    }

    fn iter(&self) -> iter::Take<iter::Repeat<()>> {
        std::iter::repeat(()).take(self.count)
    }
}

#[derive(Clone)]
struct Yard {
    stacks: Vec<Vec<char>>,
    move_plan: Vec<MoveStep>
}

impl Yard {
    fn new(input: &str) -> Self {
        let mut yard = Self { stacks: vec![], move_plan: vec![] };

        let mut lines = input.lines();
        let mut line : &str;

        // read in stacks
        loop {
            line = lines.next().unwrap();

            if line[0..=1].eq(" 1") {
                break;
            }
            // line = line.trim();
            let mut stack_index = 0;

            while !line.is_empty() {
                if yard.stacks.len() <= stack_index {
                    yard.stacks.push(vec![]);
                }

                let stack_name : &str = &line[0..=2];
                if !stack_name.eq("   ") {
                    yard.stacks[stack_index].push(stack_name.chars().nth(1).unwrap())
                }
                stack_index += 1;

                if line.len() == 3 {
                    break;
                }
                let old_line = line;
                line = &old_line[4..];
            }
        }

        yard.reverse_stacks();
        lines.next();

        // read in moves
        loop {
            let possible_line = lines.next();
            if possible_line.is_none() {
                break;
            }

            line = possible_line.unwrap();
            if line.is_empty() {
                break;
            }

            yard.move_plan.push(MoveStep::new(line))
        }

        yard
    }

    fn reverse_stacks(&mut self) {
        let mut new_stacks : Vec<Vec<char>> = vec![];
        for stack in self.stacks.iter() {
            let mut new_stack: Vec<char> = vec![];
            for &shipping_crate in stack.iter().rev() {
                new_stack.push(shipping_crate);
            }
            new_stacks.push(new_stack)
        }
        self.stacks = new_stacks;
    }

    fn after_following_plan(&self, executor: fn(&mut Self) -> ()) -> Self {
        let mut new_yard = self.clone();

        executor(&mut new_yard);

        new_yard
    }

    fn follow_part_one_plan(&mut self) {
        for move_step in self.move_plan.iter() {
            for _iteration in move_step.iter() {
                let shipping_crate = self.stacks[move_step.from - 1].pop().unwrap();
                self.stacks[move_step.to - 1].push(shipping_crate);
            }
        }
    }

    fn follow_part_two_plan(&mut self) {
        for move_step in self.move_plan.iter() {
            let mut crates_to_move : Vec<char> = vec![];

            for _iteration in move_step.iter() {
                let shipping_crate = self.stacks[move_step.from - 1].pop().unwrap();
                crates_to_move.push(shipping_crate);
            }

            crates_to_move.reverse();

            for shipping_crate in crates_to_move {
                self.stacks[move_step.to - 1].push(shipping_crate);
            }
        }
    }

    fn top_crates(&self) -> String {
        String::from_iter(self.stacks.iter().map(|stack| stack.iter().rev().next().unwrap()))
    }

    // fn print(&self) {
    //     let max_height = self.largest_stack_size() as i32;
    //     for height in (0..max_height).rev() {
    //         for stack in self.stacks.iter() {
    //             // dbg!(stack);
    //             let len = stack.len() as i32;
    //             // let offset = max_height - len;
    //             let index =  height;
    //             // println!("{} {} {}", height, len, index);
    //             if -1 < index && index < len  {
    //                 print!("[{}] ", stack[index as usize]);
    //             } else {
    //                 print!("{}", "    ");
    //             }
    //         }
    //         println!();
    //     }
    //
    //     for (index, _stack) in self.stacks.iter().enumerate() {
    //         print!(" {}  ", index + 1);
    //     }
    //     println!();
    // }
    //
    // fn largest_stack_size(&self) -> usize {
    //     self.stacks.iter().map(Vec::len).max().unwrap()
    // }
}

#[cfg(test)]
mod tests {
    use std::fs::read_to_string;
    use super::*;

    #[test]
    fn part_one_example() {
        assert_eq!(
            Yard::new(read_to_string("example_input.txt").unwrap().as_str())
                .after_following_plan(Yard::follow_part_one_plan)
                .top_crates(),
            "CMZ"
        );
    }

    #[test]
    fn part_one() {
        assert_eq!(
            Yard::new(read_to_string("input.txt").unwrap().as_str())
                .after_following_plan(Yard::follow_part_one_plan)
                .top_crates(),
            "TPGVQPFDH"
        );
    }

    #[test]
    fn part_two_example() {
        assert_eq!(
            Yard::new(read_to_string("example_input.txt").unwrap().as_str())
                .after_following_plan(Yard::follow_part_two_plan)
                .top_crates(),
            "MCD"
        );
    }

    #[test]
    fn part_two() {
        assert_eq!(
            Yard::new(read_to_string("input.txt").unwrap().as_str())
                .after_following_plan(Yard::follow_part_two_plan)
                .top_crates(),
            "DMRDFRHHH"
        );
    }
}
