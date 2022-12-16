use std::collections::HashSet;
use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::str::FromStr;

use anyhow::Context;
use regex::Regex;

const ROW_TO_CHECK: isize = 2_000_000;
const SIZE: isize = 4_000_000;

fn main() {
    let circles: Vec<ManhattanCircle> = read_lines()
        .expect("Couldn't read file")
        .map(|line| line.expect("Couldn't read line"))
        .map(|line| {
            line.parse::<ManhattanCircle>()
                .expect("Unable to parse circle")
        })
        .collect();

    let from_x = circles.iter().map(|circle| circle.left()).min().unwrap();
    let to_x = circles.iter().map(|circle| circle.right()).max().unwrap();
    let beacons: HashSet<(isize, isize)> = circles.iter().map(|circle| circle.beacon).collect();

    let part_1 = (from_x..=to_x)
        .filter(|x| !beacons.contains(&(*x, ROW_TO_CHECK)))
        .filter(|x| {
            circles
                .iter()
                .any(|circle| circle.contains(&(*x, ROW_TO_CHECK)))
        })
        .count();
    println!("Part 1: {}", part_1);

    let candidate_points = circles
        .iter()
        .map(|circle| circle.just_outside_of_range())
        .reduce(|acc, positions| {
            acc.union(&positions)
                .filter(|(x, y)| *x >= 0 && *x <= SIZE && *y >= 0 && *y <= SIZE)
                .copied()
                .collect()
        })
        .unwrap();

    let missing_beacon = candidate_points
        .iter()
        .find(|point| !circles.iter().any(|circle| circle.contains(point)))
        .unwrap();

    let part_2 = 4_000_000 * missing_beacon.0 + missing_beacon.1;

    println!("Part_2: {}", part_2);
}

#[derive(Debug)]
struct ManhattanCircle {
    center: (isize, isize),
    beacon: (isize, isize),
    radius: usize,
}

impl ManhattanCircle {
    fn new(center: (isize, isize), beacon: (isize, isize)) -> Self {
        ManhattanCircle {
            center,
            beacon,
            radius: ((beacon.0 - center.0).abs() + (beacon.1 - center.1).abs()) as usize,
        }
    }

    fn left(&self) -> isize {
        self.center.1 - self.radius as isize
    }

    fn right(&self) -> isize {
        self.center.1 + self.radius as isize
    }

    fn contains(&self, (x, y): &(isize, isize)) -> bool {
        (self.center.0 - x).abs() + (self.center.1 - y).abs() <= self.radius as isize
    }

    fn just_outside_of_range(&self) -> HashSet<(isize, isize)> {
        let distance = self.radius as isize + 1;
        ((self.center.1 - distance)..=(self.center.1 + distance))
            .flat_map(|y| {
                let y_distance = (self.center.1 - y).abs();

                [
                    (self.center.0 - distance + y_distance, y),
                    (self.center.0 + distance - y_distance, y),
                ]
            })
            .collect()
    }
}

impl FromStr for ManhattanCircle {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let re =
            Regex::new(r"Sensor at x=(\d+), y=(\d+): closest beacon is at x=(-?\d+), y=(-?\d+)")
                .unwrap();
        let captures = re.captures_iter(s).next().context("Didn't match sensor")?;
        let x: isize = captures[1].parse()?;
        let y: isize = captures[2].parse()?;
        let beacon_x: isize = captures[3].parse()?;
        let beacon_y: isize = captures[4].parse()?;

        Ok(ManhattanCircle::new((x, y), (beacon_x, beacon_y)))
    }
}

fn read_lines() -> io::Result<io::Lines<io::BufReader<File>>> {
    let filename: String = env::args().skip(1).next().expect("Missing file path");
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
