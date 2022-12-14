use std::collections::HashSet;
use std::env;
use std::fs::File;
use std::io::{self, BufRead};

fn main() {
    let rock_coordinates: Vec<Vec<(usize, usize)>> = read_lines()
        .expect("Unable to read file")
        .map(|line| {
            line.expect("Unable to read line")
                .split(" -> ")
                .map(|coordinates| {
                    let mut split = coordinates.split(',');
                    (
                        split
                            .next()
                            .expect("Unable to find rock x")
                            .parse::<usize>()
                            .expect("Unable to parse rock x"),
                        split
                            .next()
                            .expect("Unable to find rock x")
                            .parse::<usize>()
                            .expect("Unable to parse rock y"),
                    )
                })
                .collect()
        })
        .collect();

    let mut rock_set: HashSet<(usize, usize)> = HashSet::new();

    for structure in &rock_coordinates {
        for window in structure.windows(2) {
            let from = window[0];
            let to = window[1];
            rock_set.insert(from);

            let mut next = from;
            while next != to {
                if to.0 == from.0 && to.1 > from.1 {
                    next.1 += 1;
                } else if to.0 == from.0 && to.1 < from.1 {
                    next.1 -= 1;
                } else if to.1 == from.1 && to.0 > from.0 {
                    next.0 += 1;
                } else {
                    next.0 -= 1;
                }

                rock_set.insert(next);
            }

            rock_set.insert(to);
        }
    }

    let mut map: Map = rock_set.into();
    let part_1 = map.simulate_sand();
    println!("Part 1: {}", part_1);
}

struct Map {
    rock: HashSet<(usize, usize)>,
    sand: HashSet<(usize, usize)>,
    bottom: usize,
}

impl Map {
    fn new(rock: HashSet<(usize, usize)>) -> Self {
        let bottom: usize = *rock
            .iter()
            .map(|(_, y)| y)
            .max()
            .expect("Can't build map without any rock");

        Map {
            rock,
            sand: HashSet::new(),
            bottom,
        }
    }

    fn simulate_sand(&mut self) -> usize {
        'grain_of_sand: loop {
            let mut sand_position = (500, 0);

            while sand_position.1 < self.bottom {
                if self.is_free(&(sand_position.0, sand_position.1 + 1)) {
                    sand_position.1 += 1;
                } else if self.is_free(&(sand_position.0 - 1, sand_position.1 + 1)) {
                    sand_position.0 -= 1;
                    sand_position.1 += 1;
                } else if self.is_free(&(sand_position.0 + 1, sand_position.1 + 1)) {
                    sand_position.0 += 1;
                    sand_position.1 += 1;
                } else {
                    self.sand.insert(sand_position);
                    continue 'grain_of_sand;
                }
            }

            break;
        }

        self.sand.len()
    }

    fn is_free(&self, coordinates: &(usize, usize)) -> bool {
        !self.rock.contains(coordinates) && !self.sand.contains(coordinates)
    }
}

impl From<HashSet<(usize, usize)>> for Map {
    fn from(rock: HashSet<(usize, usize)>) -> Self {
        Map::new(rock)
    }
}

fn read_lines() -> io::Result<io::Lines<io::BufReader<File>>> {
    let filename: String = env::args().skip(1).next().expect("Missing file path");
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
