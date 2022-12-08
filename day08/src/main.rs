use std::collections::HashSet;
use std::env;
use std::fs::File;
use std::io::{self, BufRead};

fn main() {
    let grid = Grid {
        trees: read_lines()
            .expect("Unable to read file")
            .map(|line| {
                line.expect("Unable to read line")
                    .chars()
                    .map(|char| char.to_digit(10).expect("Invalid character") as usize)
                    .collect()
            })
            .collect(),
    };

    let part_1 = grid.visible_count();
    println!("Part 1: {}", part_1);

    let part_2 = grid.highest_scenic_score();
    println!("Part 2: {}", part_2);
}

struct Grid {
    trees: Vec<Vec<usize>>,
}

impl Grid {
    fn visible_count(&self) -> usize {
        let mut visible = HashSet::<(usize, usize)>::new();

        for tree_line in self.tree_lines() {
            let mut max = None;

            visible.extend(tree_line.iter().filter(move |&&(x, y)| match max {
                Some(number) if number < self.trees[y][x] => {
                    max = Some(self.trees[y][x]);
                    true
                }
                None => {
                    max = Some(self.trees[y][x]);
                    true
                }
                _ => false,
            }));
        }

        visible.len()
    }

    fn tree_lines(&self) -> Vec<Vec<(usize, usize)>> {
        let height = self.trees.len();
        let width = self.trees[0].len();

        let mut lines: Vec<Vec<(usize, usize)>> = vec![];
        lines.extend((0..height).map(|y| (0..width).map(move |x| (x, y)).collect()));
        lines.extend((0..height).map(|y| (0..width).rev().map(move |x| (x, y)).collect()));
        lines.extend((0..width).map(|x| (0..height).map(move |y| (x, y)).collect()));
        lines.extend((0..width).map(|x| (0..height).rev().map(move |y| (x, y)).collect()));

        lines
    }

    fn highest_scenic_score(&self) -> usize {
        let height = self.trees.len();
        let width = self.trees[0].len();

        let mut highest_score = 0;

        for y in 1..(height - 1) {
            for x in 1..(width - 1) {
                let mut score = 1;

                for (x_direction, y_direction) in [(0, 1), (0, -1), (1, 0), (-1, 0)] {
                    let mut visible = 0;
                    let mut other_x = x as isize;
                    let mut other_y = y as isize;

                    loop {
                        other_x += x_direction;
                        other_y += y_direction;

                        if !(0..width).contains(&(other_x as usize))
                            || !(0..height).contains(&(other_y as usize))
                        {
                            break;
                        }

                        visible += 1;

                        if self.trees[other_y as usize][other_x as usize] >= self.trees[y][x] {
                            break;
                        }
                    }

                    score *= visible;
                }

                if score > highest_score {
                    highest_score = score;
                }
            }
        }

        highest_score
    }
}

fn read_lines() -> io::Result<io::Lines<io::BufReader<File>>> {
    let filename: String = env::args().skip(1).next().expect("Missing file path");
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
