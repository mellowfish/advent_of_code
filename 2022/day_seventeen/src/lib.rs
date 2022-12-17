use std::iter::Cycle;
use std::slice::Iter;

#[derive(Clone, Debug)]
struct Point {
    x: usize,
    y: usize
}

impl Point {
    fn down(&self) -> Self {
        Self { x: self.x, y: self.y - 1 }
    }

    fn left(&self) -> Self {
        Self { x: self.x - 1, y: self.y }
    }

    fn right(&self) -> Self {
        Self { x: self.x + 1, y: self.y }
    }
}

enum Shape {
    Dash(Vec<Vec<char>>),
    Plus(Vec<Vec<char>>),
    Elbow(Vec<Vec<char>>),
    Pipe(Vec<Vec<char>>),
    Square(Vec<Vec<char>>)
}

impl Shape {
    fn first() -> Self {
        Self::dash()
    }

    fn next(previous: &Self) -> Self {
        match previous {
            Self::Dash(_) => Self::plus(),
            Self::Plus(_) => Self::elbow(),
            Self::Elbow(_) => Self::pipe(),
            Self::Pipe(_) => Self::square(),
            Self::Square(_) => Self::dash(),
        }
    }

    fn dash() -> Self {
        Self::Dash(
            vec![
                "@@@@".chars().collect()
            ]
        )
    }

    fn plus() -> Self {
        Self::Plus(
            vec![
                ".@.".chars().collect(),
                "@@@".chars().collect(),
                ".@.".chars().collect()
            ]
        )
    }

    fn elbow() -> Self {
        Self::Elbow(
            vec![
                "..@".chars().collect(),
                "..@".chars().collect(),
                "@@@".chars().collect()
            ]
        )
    }

    fn pipe() -> Self {
        Self::Pipe(
            vec![
                "@".chars().collect(),
                "@".chars().collect(),
                "@".chars().collect(),
                "@".chars().collect()
            ]
        )
    }

    fn square() -> Self {
        Self::Square(
            vec![
                "@@".chars().collect(),
                "@@".chars().collect()
            ]
        )
    }

    fn unwrap(&self) -> &Vec<Vec<char>> {
        match self {
            Self::Dash(data) => data,
            Self::Plus(data) => data,
            Self::Elbow(data) => data,
            Self::Pipe(data) => data,
            Self::Square(data) => data,
        }
    }

    fn height(&self) -> usize {
        self.unwrap().len()
    }

    fn width(&self) -> usize {
        self.unwrap().first().unwrap().len()
    }

    fn top_right(&self, bottom_left: &Point) -> Point {
        Point { x: bottom_left.x + self.width() - 1, y: bottom_left.y + self.height() - 1 }
    }
}

#[derive(Clone)]
enum Jet {
    Left,
    Right
}

struct Chamber {
    jets: Vec<Jet>,
    rows: Vec<Vec<char>>
}

impl Chamber {
    fn new(input: &str) -> Self {
        let jets : Vec<Jet> = input.chars().map(|symbol| {
            match symbol {
                '<' => Jet::Left,
                '>' => Jet::Right,
                _ => panic!("Unexpected chartacer: {symbol}")
            }
        }).collect();
        let floor = "+-------+".chars().collect();

        Self { jets, rows: vec![floor] }
    }

    fn drop_rocks(&mut self, count: usize) {
        let owned_jets = self.jets.clone();
        let mut jet_cycle = owned_jets.iter().cycle();
        let mut current_rock = Shape::first();

        for _ in 0..count {
            self.drop_rock(&current_rock, &mut jet_cycle);
            current_rock = Shape::next(&current_rock);
        }
    }

    fn drop_rock(&mut self, rock: &Shape, jet_cycle: &mut Cycle<Iter<Jet>>) {
        self.ensure_space_for_new_rock(rock);

        let mut bottom_left = Point { x: 3, y: self.rock_height() + 4 };

        self.blit_rock(rock, &bottom_left);
        self.print();
        self.blank_rock(rock, &bottom_left);
        loop {
            match jet_cycle.next() {
                None => panic!("Jets ran out of steam!"),
                Some(Jet::Left) => {
                    if self.can_move_left() {
                        self.blank_rock(rock, &bottom_left);
                        bottom_left = bottom_left.left();
                        self.blit_rock(rock, &bottom_left);
                    }
                },
                Some(Jet::Right) => {
                    if self.can_move_right() {
                        self.blank_rock(rock, &bottom_left);
                        bottom_left = bottom_left.right();
                        self.blit_rock(rock, &bottom_left);
                    }
                }
            }
            self.print();
            // TODO: check for collisions!
            self.blank_rock(rock, &bottom_left);
            if self.can_move_down() {
                bottom_left = bottom_left.down();
            } else {
                todo!("Handle stopping")
            }
            self.blit_rock(rock, &bottom_left);
            self.print();

            todo!()
        }

        todo!()
    }

    fn can_move_right(&self) -> bool {
        for row in self.rows.iter().rev().take(10) {
            for pair in row.windows(2) {
                if pair['0'] == '@'
            }
        }
    }

    fn blit_rock(&mut self, rock: &Shape, bottom_left: &Point) {
        let top_right = rock.top_right(bottom_left);
        for (row_index, row) in (bottom_left.y..=top_right.y).rev().enumerate() {
            for (column_index, column) in (bottom_left.x..=top_right.x).enumerate() {
                if self.rows[row][column] != '.' {
                    panic!("About to blow a hole in the universe!")
                }
                self.rows[row][column] = rock.unwrap()[row_index][column_index];
            }
        }
    }

    fn blank_rock(&mut self, rock: &Shape, bottom_left: &Point) {
        let top_right = rock.top_right(bottom_left);
        for row in bottom_left.y..=top_right.y {
            for column in bottom_left.x..=top_right.x {
                self.rows[row][column] = '.';
            }
        }
    }

    fn print(&self) {
        for row in self.rows.iter().rev() {
            println!("{}", String::from_iter(row.iter()))
        }
        println!();
    }

    fn ensure_space_for_new_rock(&mut self, rock: &Shape) {
        let delta : i32 = (self.rock_height() + rock.height() + 3) as i32 - self.rows.len() as i32;
        if delta > 0 {
            for _ in 0..=delta {
                self.add_empty_row();
            }
        }
    }

    fn add_empty_row(&mut self) {
        self.rows.push("|.......|".chars().collect());
    }

    fn rock_height(&self) -> usize {
        for index in (0..self.rows.len()).rev() {
            if self.rock_in_row(index) {
                return index;
            }
        }

        0
    }

    fn rock_in_row(&self, index: usize) -> bool {
        self.rows[index].contains(&'#')
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_one_example() {
        let mut chamber = Chamber::new(">>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>");
        chamber.drop_rocks(2022);
        assert_eq!(chamber.rock_height(), 3068);
    }
}
