use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::str::FromStr;

use anyhow::Context;

/**
 * --- Day 4: Camp Cleanup ---
 *
 * The challenge provides a list of range pairs. Part 1 asks whether one of the ranges in the pair
 * entirely contains another. Part 2 asks whether they overlap.
 * This program pretty much implements that literally, most of the code is parsing. A custom struct
 * is used instead of Rust's `RangeInclusive` for the small convenience of implementing `FromStr` on
 * it. Plus, Rust's ranges are iterators but here we only care about start and end values.
 */
fn main() -> anyhow::Result<()> {
    let ranges: Vec<(AssignmentRange, AssignmentRange)> = read_lines()
        .expect("Unable to read file")
        .map(|line| line.expect("Unable to read line"))
        .map(|line| parse_line(&line))
        .collect::<Result<_, _>>()?;

    let part_1 = ranges
        .iter()
        .filter(|(first_range, second_range)| {
            first_range.contains(second_range) || second_range.contains(first_range)
        })
        .count();
    println!("Part 1: {}", part_1);

    let part_2 = ranges
        .iter()
        .filter(|(first_range, second_range)| first_range.overlaps(second_range))
        .count();
    println!("Part 2: {}", part_2);

    Ok(())
}

struct AssignmentRange {
    from: usize,
    to: usize,
}

impl AssignmentRange {
    fn contains(&self, other: &AssignmentRange) -> bool {
        self.from <= other.from && self.to >= other.to
    }

    fn overlaps(&self, other: &AssignmentRange) -> bool {
        (self.from <= other.from && self.to >= other.from)
            || (other.from <= self.from && other.to >= self.from)
    }
}

impl FromStr for AssignmentRange {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut split = s.split('-');
        let first_range = split.next().context("Could not find range start")?;
        let second_range = split.next().context("Could not find range end")?;

        Ok(AssignmentRange {
            from: first_range.parse()?,
            to: second_range.parse()?,
        })
    }
}

fn parse_line(line: &str) -> anyhow::Result<(AssignmentRange, AssignmentRange)> {
    let mut split = line.split(',');

    Ok((
        split
            .next()
            .context("Unable to get first range definition")?
            .parse()?,
        split
            .next()
            .context("Unable to get second range definition")?
            .parse()?,
    ))
}

fn read_lines() -> io::Result<io::Lines<io::BufReader<File>>> {
    let filename: String = env::args().skip(1).next().expect("Missing file path");
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
