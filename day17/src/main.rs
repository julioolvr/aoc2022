use std::collections::HashSet;
use std::env;
use std::fs::File;
use std::io::{self, BufRead};

use anyhow::bail;

fn main() {
    let directions = read_lines()
        .expect("Unable to read file")
        .next()
        .expect("Didn't find any lines")
        .expect("Unable to read line")
        .chars()
        .map(|c| TryInto::<Direction>::try_into(c).expect("Unable to parse direction"))
        .collect();

    let shapes = vec![Shape::Dash, Shape::Plus, Shape::J, Shape::I, Shape::O];

    let mut board = Board::new(shapes, directions);
    board.simulate(2022);
    let part_1 = board.tower_height();
    println!("Part 1: {}", part_1);
}

struct Board {
    occupied: HashSet<(usize, usize)>,
    directions: Vec<Direction>,
    shapes: Vec<Shape>,
}

impl Board {
    fn new(shapes: Vec<Shape>, directions: Vec<Direction>) -> Self {
        Board {
            occupied: HashSet::new(),
            shapes,
            directions,
        }
    }

    fn simulate(&mut self, number_of_pieces: usize) {
        let mut directions_iter = self.directions.iter().cycle();
        let shapes_iter = self.shapes.iter().cycle();

        for shape in shapes_iter.take(number_of_pieces) {
            let mut has_fallen = false;
            let mut left = 2;
            let mut bottom = self.tower_height() + 3;

            while !has_fallen {
                let direction = directions_iter.next().unwrap();

                match direction {
                    Direction::Left => {
                        if left > 0 && self.can_move(shape, left - 1, bottom) {
                            left -= 1
                        }
                    }
                    Direction::Right => {
                        if left + shape.width() < self.width()
                            && self.can_move(shape, left + 1, bottom)
                        {
                            left += 1;
                        }
                    }
                }

                if bottom > 0 && self.can_move(shape, left, bottom - 1) {
                    bottom -= 1;
                } else {
                    self.occupied
                        .extend(shape.from_left_and_bottom(left, bottom));
                    has_fallen = true;
                }
            }
        }
    }

    fn can_move(&self, shape: &Shape, left: usize, bottom: usize) -> bool {
        self.occupied
            .intersection(&shape.from_left_and_bottom(left, bottom))
            .count()
            == 0
    }

    fn width(&self) -> usize {
        // Can this be some sort of constant?
        7
    }

    fn tower_height(&self) -> usize {
        self.occupied
            .iter()
            .map(|(_, y)| y)
            .max()
            .map(|height| *height + 1)
            .unwrap_or(0)
    }
}

#[derive(Copy, Clone)]
enum Direction {
    Left,
    Right,
}

impl TryFrom<char> for Direction {
    type Error = anyhow::Error;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '<' => Ok(Direction::Left),
            '>' => Ok(Direction::Right),
            c => bail!("Invalid direction {}", c),
        }
    }
}

#[derive(Clone, Debug)]
enum Shape {
    /**
     * ####
     */
    Dash,
    /**
     * .#.
     * ###
     * .#.
     */
    Plus,
    /**
     * ..#
     * ..#
     * ###
     */
    J,
    /**
     * #
     * #
     * #
     * #
     */
    I,
    /**
     * ##
     * ##
     */
    O,
}

impl Shape {
    fn width(&self) -> usize {
        use Shape::*;

        match self {
            Dash => 4,
            Plus => 3,
            J => 3,
            I => 1,
            O => 2,
        }
    }

    fn from_left_and_bottom(&self, left: usize, bottom: usize) -> HashSet<(usize, usize)> {
        use Shape::*;

        match self {
            Dash => HashSet::from([
                (left, bottom),
                (left + 1, bottom),
                (left + 2, bottom),
                (left + 3, bottom),
            ]),
            Plus => HashSet::from([
                (left + 1, bottom),
                (left, bottom + 1),
                (left + 1, bottom + 1),
                (left + 2, bottom + 1),
                (left + 1, bottom + 2),
            ]),
            J => HashSet::from([
                (left, bottom),
                (left + 1, bottom),
                (left + 2, bottom),
                (left + 2, bottom + 1),
                (left + 2, bottom + 2),
            ]),
            I => HashSet::from([
                (left, bottom),
                (left, bottom + 1),
                (left, bottom + 2),
                (left, bottom + 3),
            ]),
            O => HashSet::from([
                (left, bottom),
                (left, bottom + 1),
                (left + 1, bottom),
                (left + 1, bottom + 1),
            ]),
        }
    }
}

fn read_lines() -> io::Result<io::Lines<io::BufReader<File>>> {
    let filename: String = env::args().skip(1).next().expect("Missing file path");
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
