use std::borrow::{Borrow, BorrowMut};
use std::cell::RefCell;
use std::rc::Rc;

struct LinkedList {
    head: Option<DoubleEndedNodeLink>
}

type DoubleEndedNodeLink = Rc<RefCell<DoubleEndedNode>>;

struct DoubleEndedNode {
    value: i32,
    next: Option<DoubleEndedNodeLink>,
    previous: Option<DoubleEndedNodeLink>
}

impl DoubleEndedNode {
    fn new(value: i32) -> DoubleEndedNodeLink {
        Rc::new(RefCell::new(Self { value, next: None, previous: None }))
    }

    fn new_with_next(value: i32, next: &DoubleEndedNodeLink) -> DoubleEndedNodeLink {
        Rc::new(RefCell::new( Self { value, next: Some(Rc::clone(next)), previous: None }))
    }

    fn new_with_previous(value: i32, previous: &DoubleEndedNodeLink) -> DoubleEndedNodeLink {
        let maybe_next : Option<DoubleEndedNodeLink> = todo!()

        if let Some(next) = previous.borrow().next {
            Rc::new(RefCell::new( Self { value, next: Some(Rc::clone(&next)), previous: Some(Rc::clone(previous)) }))
        } else {
            Rc::new(RefCell::new( Self { value, next: None, previous: Some(Rc::clone(previous)) }))
        }
    }
}

impl LinkedList {
    fn new() -> Self {
        Self { head: None }
    }

    fn append(&mut self, value: i32) {
        match &mut self.head {
            None => {
                self.head = Some(DoubleEndedNode::new(value))
            },
            Some(head) => {
                let new_node = DoubleEndedNode::new_with_previous(value, head);
                head.()
            }
        }
    }
}

struct CoordinateFile {
    data: Vec<i32>
}

impl CoordinateFile {
    fn new(input: &str) -> Self {
        Self { data: input.lines().map(|line| line.parse::<i32>().unwrap()).collect() }
    }

    fn mix_one_number(data: Vec<i32>, number: i32) -> Vec<i32> {
        let mut output = data.clone();

        Self { data }.mix_data(number, &mut output);

        output
    }

    fn decrypt_data(&self) -> Vec<i32> {
        let mut decrypted_data : Vec<i32> = self.data.clone();

        dbg!(&decrypted_data);

        for &number in self.data.iter() {
            self.mix_data(number, &mut decrypted_data);

            dbg!(&decrypted_data);
        }

        decrypted_data
    }

    fn mix_data(&self, number: i32, target_data: &mut Vec<i32>) {
        todo!();
    }

    fn coordinate_sum(&self) -> i32 {
        let decrypted_data = self.decrypt_data();

        todo!();
    }
}

#[cfg(test)]
mod tests {
    use std::fs;
    use super::*;

    #[test]
    fn mix_one_number() {
        // assert_eq!(CoordinateFile::mix_one_number(vec![], -1), vec![]);
        assert_eq!(CoordinateFile::mix_one_number(vec![4, 5, 6, 1, 7, 8, 9], 1), vec![4, 5, 6, 7, 1, 8, 9]);
        assert_eq!(CoordinateFile::mix_one_number(vec![4, -2, 5, 6, 7, 8, 9], -1), vec![4, 5, 6, 7, 8, -2, 9]);
    }

    #[test]
    fn part_one_example() {
        assert_eq!(
            CoordinateFile::new(fs::read_to_string("example_input.txt").unwrap().as_str()).coordinate_sum(),
            4
        );
    }
}
