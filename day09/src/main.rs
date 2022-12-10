use std::collections::HashSet;
use std::fs::File;
use std::io::{self, BufRead};
use std::str::FromStr;
use std::{env, iter};

use anyhow::{bail, Context};

fn main() {
    let movements: Vec<Move> = read_lines()
        .expect("Unable to read file")
        .map(|line| {
            line.expect("Unable to read line")
                .parse()
                .expect("Unable to parse movement")
        })
        .collect();

    let mut rope = Rope::new(2);
    rope.follow_movements(&movements);

    let part_1 = rope.visited_spots();
    println!("Part 1: {}", part_1);

    let mut rope = Rope::new(10);
    rope.follow_movements(&movements);

    let part_2 = rope.visited_spots();
    println!("Part 2: {}", part_2);
}

struct Rope {
    segments: Vec<(isize, isize)>,
    visited: HashSet<(isize, isize)>,
}

impl Rope {
    fn new(length: usize) -> Self {
        Rope {
            segments: iter::repeat((0, 0)).take(length).collect(),
            visited: HashSet::from([(0, 0)]),
        }
    }

    fn follow_movements(&mut self, movements: &Vec<Move>) {
        use Move::*;

        let rope_length = self.segments.len();

        for movement in movements {
            let (x_movement, y_movement, &distance) = match movement {
                Right(n) => (1, 0, n),
                Left(n) => (-1, 0, n),
                Up(n) => (0, 1, n),
                Down(n) => (0, -1, n),
            };

            for _ in 0..distance {
                let head = self.segments[0];
                let mut last_segment_position = (head.0 + x_movement, head.1 + y_movement);
                self.segments[0] = last_segment_position;

                for i in 1..rope_length {
                    let current_segment_position = self.segments[i];
                    let mut next_position = current_segment_position;

                    if last_segment_position.0 == current_segment_position.0 {
                        if last_segment_position.1 - current_segment_position.1 > 1 {
                            next_position.1 += 1;
                        } else if current_segment_position.1 - last_segment_position.1 > 1 {
                            next_position.1 -= 1;
                        }
                    } else if last_segment_position.1 == current_segment_position.1 {
                        if last_segment_position.0 - current_segment_position.0 > 1 {
                            next_position.0 += 1;
                        } else if current_segment_position.0 - last_segment_position.0 > 1 {
                            next_position.0 -= 1;
                        }
                    } else {
                        if last_segment_position.0.abs_diff(current_segment_position.0) > 1
                            || last_segment_position.1.abs_diff(current_segment_position.1) > 1
                        {
                            if last_segment_position.0 > current_segment_position.0 {
                                next_position.0 += 1;
                            }

                            if last_segment_position.0 < current_segment_position.0 {
                                next_position.0 -= 1;
                            }

                            if last_segment_position.1 > current_segment_position.1 {
                                next_position.1 += 1;
                            }

                            if last_segment_position.1 < current_segment_position.1 {
                                next_position.1 -= 1;
                            }
                        }
                    }

                    self.segments[i] = next_position;
                    last_segment_position = next_position;
                }

                self.visited
                    .insert(*self.segments.last().expect("Empty rope"));
            }
        }
    }

    fn visited_spots(&self) -> usize {
        self.visited.len()
    }
}

#[derive(Debug)]
enum Move {
    Right(isize),
    Left(isize),
    Up(isize),
    Down(isize),
}

impl FromStr for Move {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut split = s.split(" ");
        let direction = split.next().context("Could not get movement direction")?;
        let distance = split
            .next()
            .context("Could not get movement distance")?
            .parse::<isize>()?;

        use Move::*;

        Ok(match direction {
            "R" => Right(distance),
            "L" => Left(distance),
            "U" => Up(distance),
            "D" => Down(distance),
            other => bail!("Invalid direction {}", other),
        })
    }
}

fn read_lines() -> io::Result<io::Lines<io::BufReader<File>>> {
    let filename: String = env::args().skip(1).next().expect("Missing file path");
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
