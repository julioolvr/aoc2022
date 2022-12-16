use std::collections::HashSet;
use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::str::FromStr;

use anyhow::Context;
use regex::Regex;

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

    let y = 2_000_000;

    let part_1 = (from_x..=to_x)
        .filter(|x| !beacons.contains(&(*x, y)))
        .filter(|x| circles.iter().any(|circle| circle.contains(&(*x, y))))
        .count();
    println!("Part 1: {}", part_1);
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
