use std::cmp;
use std::ops::{Range, RangeInclusive};

type Tree = u8;
type ScenicScore = u32;

struct TreeVisibility {
    north: bool,
    south: bool,
    east: bool,
    west: bool
}

impl TreeVisibility {
    fn is_visible(&self) -> bool {
        self.north || self.south || self.east || self.west
    }

    fn north() -> Self {
        Self { north: true, south: false, east: false, west: false }
    }

    fn south() -> Self {
        Self { north: false, south: true, east: false, west: false }
    }

    fn east() -> Self {
        Self { north: false, south: false, east: true, west: false }
    }

    fn west() -> Self {
        Self { north: false, south: false, east: false, west: true }
    }
}

struct TreeScore {
    north: ScenicScore,
    south: ScenicScore,
    east: ScenicScore,
    west: ScenicScore
}

impl TreeScore {
    fn total(&self) -> ScenicScore {
        self.north * self.south * self.west * self.east
    }
}

struct ForestMap {
    rows: Vec<Vec<Tree>>
}

impl ForestMap {
    fn new(input: &str) -> Self {
        Self {
            rows: input.lines().map(
                |line| line.bytes().map(|digit| (digit - ('0' as u8))).collect::<Vec<Tree>>()
            ).collect()
        }
    }

    fn most_scenic_score(&self) -> ScenicScore {
        // self.print_score();

        let mut max_score = 0;

        for target_row in self.interior_row_indices() {
            for target_column in self.interior_column_indices() {
                max_score = cmp::max(max_score, self.scenic_score(target_row, target_column).total());
            }
        }

        max_score
    }

    fn scenic_score(&self, target_row: usize, target_column: usize) -> TreeScore {
        let target_tree = self.tree_at(target_row, target_column);
        let mut north_score = 0;
        let mut south_score = 0;
        let mut east_score = 0;
        let mut west_score = 0;

        // Look North
        for row in (0..target_row).rev() {
            north_score += 1;
            if target_tree <= self.tree_at(row, target_column) {
                break;
            }
        }
        // Look South
        for row in (target_row + 1)..=self.last_row_index() {
            south_score += 1;
            if target_tree <= self.tree_at(row, target_column) {
                break;
            }
        }
        // Look West
        for column in (0..target_column).rev() {
            west_score += 1;
            if target_tree <= self.tree_at(target_row, column) {
                break;
            }
        }
        // Look East
        for column in (target_column + 1)..=self.last_row_index() {
            east_score += 1;
            if target_tree <= self.tree_at(target_row, column) {
                break;
            }
        }

        TreeScore { north: north_score, south: south_score, east: east_score, west: west_score }
    }

    fn count_visible_trees(&self) -> usize {
        // self.print_visibility();

        let mut count = 0;

        for target_row in self.row_indices() {
            for target_column in self.column_indices() {
                if self.tree_is_visible(target_row, target_column) {
                    count += 1;
                }
            }
        }

        count
    }

    fn tree_at(&self, target_row: usize, target_column: usize) -> Tree {
        *self.rows.iter().nth(target_row).unwrap().iter().nth(target_column).unwrap()
    }

    fn tree_visibility(&self, target_row: usize, target_column: usize) -> TreeVisibility {
        if let Some(edge_visibility) = self.tree_visibility_on_edge(target_row, target_column) {
            return edge_visibility;
        }

        let target_tree = self.tree_at(target_row, target_column);

        let mut visible_from_north = true;
        let mut visible_from_south = true;
        let mut visible_from_east = true;
        let mut visible_from_west = true;

        for row in self.row_indices() {
            if row == target_row {
                continue;
            }

            let tree = self.tree_at(row, target_column);

            if tree >= target_tree {
                if row < target_row {
                    visible_from_north = false;
                } else { // row > target_row
                    visible_from_south = false;
                }
            }
        }

        for column in self.column_indices() {
            if column == target_column {
                continue;
            }

            let tree = self.tree_at(target_row, column);

            if tree >= target_tree {
                if column < target_column {
                    visible_from_west = false;
                } else { // column > target_column
                    visible_from_east = false;
                }
            }
        }

        TreeVisibility {
            north: visible_from_north,
            south: visible_from_south,
            east: visible_from_east,
            west: visible_from_west
        }
    }

    fn tree_is_visible(&self, target_row: usize, target_column: usize) -> bool {
        self.tree_visibility(target_row, target_column).is_visible()
    }

    fn tree_visibility_on_edge(&self, target_row: usize, target_column: usize) -> Option<TreeVisibility> {
        if target_row == 0 {
            return Some(TreeVisibility::north())
        }
        if target_row == self.last_row_index() {
            return Some(TreeVisibility::south())
        }
        if target_column == 0 {
            return Some(TreeVisibility::west())
        }
        if target_column == self.last_column_index() {
            return Some(TreeVisibility::east())
        }

        None
    }

    fn last_row_index(&self) -> usize {
        self.rows.len() - 1
    }

    fn row_indices(&self) -> RangeInclusive<usize> {
        0..=self.last_row_index()
    }

    fn interior_row_indices(&self) -> Range<usize> {
        1..self.last_row_index()
    }

    fn last_column_index(&self) -> usize {
        self.rows.first().unwrap().len() - 1
    }

    fn column_indices(&self) -> RangeInclusive<usize> {
        0..=self.last_column_index()
    }

    fn interior_column_indices(&self) -> Range<usize> {
        1..self.last_column_index()
    }

    #[allow(dead_code)]
    fn print_visibility(&self) {
        for target_row in self.row_indices() {
            let visibilities : Vec<(Tree, TreeVisibility)>
                = self.column_indices().map(
                    |target_column|
                        (
                            self.tree_at(target_row, target_column),
                            self.tree_visibility(target_row, target_column)
                        )
                ).collect();

            // top row
            for (_, visibility) in visibilities.iter() {
                if visibility.north {
                    print!(" ^  ");
                } else {
                    print!("    ");
                }
            }

            // middle row
            println!();
            for (tree, visibility) in visibilities.iter() {
                if visibility.west {
                    if visibility.east {
                        print!("<{}> ", tree);
                    } else {
                        print!("<{}  ", tree);
                    }

                } else {
                    if visibility.east {
                        print!(" {}> ", tree);
                    } else {
                        print!(" {}  ", tree);
                    }
                }
            }
            println!();

            // bottom row
            for (_, visibility) in visibilities.iter() {
                if visibility.south {
                    print!(" v  ");
                } else {
                    print!("    ");
                }
            }
            println!();

            // spacer row
            println!();
        }
    }

    #[allow(dead_code)]
    fn print_score(&self) {
        for target_row in self.row_indices() {
            let scores : Vec<(Tree, TreeScore)>
                = self.column_indices().map(
                |target_column|
                    (
                        self.tree_at(target_row, target_column),
                        self.scenic_score(target_row, target_column)
                    )
            ).collect();

            for (tree, score) in scores.iter() {
                print!("  {}[{}] ", score.north, tree);
            }
            println!();
            for (_tree, score) in scores.iter() {
                print!("{} {} {}  ", score.west, score.total(), score.east);
            }
            println!();
            for (_tree, score) in scores.iter() {
                print!("  {}    ", score.south);
            }
            println!();
            println!();
        }
    }
}

#[cfg(test)]
mod tests {
    use std::fs;
    use super::*;

    #[test]
    fn part_one_example() {
        assert_eq!(
            ForestMap::new(fs::read_to_string("example_input.txt").unwrap().as_str()).count_visible_trees(),
            21
        );
    }

    #[test]
    fn part_one() {
        assert_eq!(
            ForestMap::new(fs::read_to_string("input.txt").unwrap().as_str()).count_visible_trees(),
            1715
        );
    }

    #[test]
    fn part_two_example() {
        assert_eq!(
            ForestMap::new(fs::read_to_string("example_input.txt").unwrap().as_str()).most_scenic_score(),
            8
        );
    }

    #[test]
    fn part_two() {
        assert_eq!(
            ForestMap::new(fs::read_to_string("input.txt").unwrap().as_str()).most_scenic_score(),
            374400
        );
    }
}
