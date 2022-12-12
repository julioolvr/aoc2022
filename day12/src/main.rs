use std::collections::HashSet;
use std::env;
use std::fs::File;
use std::io::{self, BufRead};

fn main() {
    let mut origin = (0, 0);
    let mut destination = (0, 0);

    let map: Vec<Vec<usize>> = read_lines()
        .expect("Unable to read file")
        .map(|line| line.expect("Unable to read line"))
        .enumerate()
        .map(|(y, line)| {
            line.chars()
                .enumerate()
                .map(|(x, c)| match c {
                    'S' => {
                        origin = (x, y);
                        'a' as usize
                    }
                    'E' => {
                        destination = (x, y);
                        'z' as usize
                    }
                    c => c as usize,
                })
                .collect()
        })
        .collect();

    let map = ElevationMap {
        map,
        origin,
        destination,
    };

    let part_1 = map.shortest_path(map.origin).unwrap();
    println!("Part 1: {}", part_1);

    let part_2 = map
        .starting_candidates()
        .filter_map(|candidate| map.shortest_path(candidate))
        .min()
        .unwrap();
    println!("Part 2: {}", part_2);
}

struct ElevationMap {
    map: Vec<Vec<usize>>,
    origin: (usize, usize),
    destination: (usize, usize),
}

impl ElevationMap {
    fn starting_candidates(&self) -> impl Iterator<Item = (usize, usize)> + '_ {
        (0..self.map.len()).flat_map(move |y| {
            (0..self.map[0].len()).filter_map(move |x| {
                if self.map[y][x] == 'a' as usize {
                    Some((x, y))
                } else {
                    None
                }
            })
        })
    }

    fn shortest_path(&self, start: (usize, usize)) -> Option<usize> {
        // Dijkstra
        let mut unvisited: HashSet<(usize, usize)> = (0..self.map.len())
            .flat_map(move |y| (0..self.map[0].len()).map(move |x| (x, y)))
            .collect();

        let mut tentative_distances: Vec<Vec<Option<usize>>> = self
            .map
            .iter()
            .map(|row| row.iter().map(|_| None).collect())
            .collect();

        let mut current_node = start;
        tentative_distances[current_node.1][current_node.0] = Some(0);

        while unvisited.len() > 0 {
            let current_distance = tentative_distances[current_node.1][current_node.0].unwrap();

            if current_node.0 > 0 {
                let left = (current_node.0 - 1, current_node.1);
                if unvisited.contains(&left) && self.can_move(current_node, left) {
                    tentative_distances[left.1][left.0] = tentative_distances[left.1][left.0]
                        .map_or(Some(current_distance + 1), |previous_distance| {
                            if previous_distance < current_distance + 1 {
                                Some(previous_distance)
                            } else {
                                Some(current_distance + 1)
                            }
                        });
                }
            }

            if current_node.0 < self.map[0].len() - 1 {
                let right = (current_node.0 + 1, current_node.1);
                if unvisited.contains(&right) && self.can_move(current_node, right) {
                    tentative_distances[right.1][right.0] = tentative_distances[right.1][right.0]
                        .map_or(Some(current_distance + 1), |previous_distance| {
                            if previous_distance < current_distance + 1 {
                                Some(previous_distance)
                            } else {
                                Some(current_distance + 1)
                            }
                        });
                }
            }

            if current_node.1 > 0 {
                let up = (current_node.0, current_node.1 - 1);
                if unvisited.contains(&up) && self.can_move(current_node, up) {
                    tentative_distances[up.1][up.0] = tentative_distances[up.1][up.0].map_or(
                        Some(current_distance + 1),
                        |previous_distance| {
                            if previous_distance < current_distance + 1 {
                                Some(previous_distance)
                            } else {
                                Some(current_distance + 1)
                            }
                        },
                    );
                }
            }

            if current_node.1 < self.map.len() - 1 {
                let down = (current_node.0, current_node.1 + 1);
                if unvisited.contains(&down) && self.can_move(current_node, down) {
                    tentative_distances[down.1][down.0] = tentative_distances[down.1][down.0]
                        .map_or(Some(current_distance + 1), |previous_distance| {
                            if previous_distance < current_distance + 1 {
                                Some(previous_distance)
                            } else {
                                Some(current_distance + 1)
                            }
                        });
                }
            }

            unvisited.remove(&current_node);

            if !unvisited.contains(&self.destination) {
                return Some(tentative_distances[self.destination.1][self.destination.0].unwrap());
            }

            let next_node = unvisited
                .iter()
                .filter(|(x, y)| tentative_distances[*y][*x].is_some())
                .min_by_key(|(x, y)| tentative_distances[*y][*x].unwrap());

            if let Some(next_node) = next_node {
                current_node = *next_node;
            } else {
                return None;
            }
        }

        unreachable!();
    }

    fn can_move(&self, (from_x, from_y): (usize, usize), (to_x, to_y): (usize, usize)) -> bool {
        let origin_value = self.map[from_y][from_x];
        let destination_value = self.map[to_y][to_x];

        destination_value <= origin_value + 1
    }
}

fn read_lines() -> io::Result<io::Lines<io::BufReader<File>>> {
    let filename: String = env::args().skip(1).next().expect("Missing file path");
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
