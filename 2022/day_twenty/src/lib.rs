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

    fn rotate_index(&self, index: usize, distance: i32) -> usize {
        let length = self.data.len() as i32;
        let mut new_index = index as i32 + distance;
        if new_index < 0 {
            while new_index < 0 {
                new_index += length;
            }
        } else if (length - 1) < new_index {
            while (length - 1) < new_index {
                new_index -= length;
            }
        }

        new_index as usize

        // if new_index < 0 {
        //     (length + (new_index % length)) as usize
        // } else if new_index < length {
        //     new_index as usize
        // } else {
        //     ((new_index % length) + 1) as usize
        // }
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
        let number_index = target_data.iter().position(|value| *value == number).unwrap();
        let new_number_index = self.rotate_index(number_index, number);
        dbg!(number, number_index, new_number_index);

        if number_index == new_number_index {
            return;
        }

        if number < 0 {
            if number_index < new_number_index {
                for index in (number_index + 1)..new_number_index {
                    target_data[index - 1] = target_data[index];
                }
                target_data[new_number_index - 1] = number;
            } else {
                if new_number_index == 0 {
                    let new_number_index = self.data.len() - 1;
                    for index in (number_index + 1)..=new_number_index {
                        target_data[index - 1] = target_data[index];
                    }
                    target_data[new_number_index] = number;
                } else {
                    todo!();
                }
            }
        } else {
            if number_index < new_number_index {
                for index in (number_index + 1)..=new_number_index {
                    target_data[index - 1] = target_data[index];
                }
                target_data[new_number_index] = number;
            } else {
                for index in ((new_number_index + 1)..=number_index).rev() {
                    target_data[index] = target_data[index - 1];
                }
                target_data[new_number_index] = number;
            }
        }
    }

    fn coordinate_sum(&self) -> i32 {
        let decrypted_data = self.decrypt_data();
        let position_of_zero = decrypted_data.iter().position(|value| *value == 0).unwrap();

        dbg!(self.rotate_index(position_of_zero, 1000));
        dbg!(self.rotate_index(position_of_zero, 2000));
        dbg!(self.rotate_index(position_of_zero, 3000));
        dbg!(decrypted_data[self.rotate_index(position_of_zero, 1000)]);
        dbg!(decrypted_data[self.rotate_index(position_of_zero, 2000)]);
        dbg!(decrypted_data[self.rotate_index(position_of_zero, 3000)]);

        decrypted_data[self.rotate_index(position_of_zero, 1000)]
            + decrypted_data[self.rotate_index(position_of_zero, 2000)]
            + decrypted_data[self.rotate_index(position_of_zero, 3000)]
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
