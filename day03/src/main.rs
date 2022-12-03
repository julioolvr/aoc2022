#![feature(iter_array_chunks)]

use std::collections::HashSet;
use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::str::FromStr;

use anyhow::bail;

/**
 * --- Day 3: Rucksack Reorganization ---
 *
 * The input is a list of rucksacks where each character is an item contained in it. Each rucksack
 * has two compartments, the first half of the items are in one and the second half in the other.
 * Each item is represented by a single character and has a "priority" based on which character it
 * is.
 * Part 1 of the challenge asks to find, for each rucksack, the only item that is in both
 * compartments and add up the priorities of all of them. Part 2 asks to find, for each group of 3
 * consecutive rucksacks, the only item that is in all three of them and add up the priorities of
 * the shared item of each group.
 * The solution uses two sets to represents each rucksack, one for each compartment. This makes it
 * easy to do an intersection of both compartments to find the element required in part 1, and
 * to make an union of them to find all the items in a single rucksack and then do an intersection
 * with the other rucksacks for part 2.
 * The fact that there's only one item matching the condition on either part is not checked, and
 * the code would work even if there were multiple.
 */

fn main() -> anyhow::Result<()> {
    let rucksacks: Vec<Rucksack> = read_lines()
        .expect("Unable to read lines")
        .map(|line| {
            line.map_err(anyhow::Error::from)
                .and_then(|line| line.parse())
        })
        .collect::<Result<Vec<Rucksack>, _>>()?;

    let part_1: usize = rucksacks
        .iter()
        .flat_map(|rucksack| rucksack.items_in_common())
        .map(|item| item.priority())
        .sum();
    println!("Part 1: {}", part_1);

    let part_2: usize = rucksacks
        .iter()
        .array_chunks()
        .map(|[a, b, c]| {
            a.items()
                .intersection(&b.items())
                .map(|&item| item)
                .collect::<HashSet<&Item>>()
                .intersection(&c.items())
                .map(|item| item.priority())
                .sum::<usize>()
        })
        .sum();
    println!("Part 2: {}", part_2);

    Ok(())
}

struct Rucksack {
    first_compartment: HashSet<Item>,
    second_compartment: HashSet<Item>,
}

impl Rucksack {
    // Returns *all* items in the Rucksack
    fn items(&self) -> HashSet<&Item> {
        self.first_compartment
            .union(&self.second_compartment)
            .collect()
    }

    // Returns only the items that exist in *both* compartments of the Rucksack
    fn items_in_common(&self) -> HashSet<&Item> {
        self.first_compartment
            .intersection(&self.second_compartment)
            .collect()
    }
}

impl FromStr for Rucksack {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut chars = s.chars();

        Ok(Rucksack {
            first_compartment: chars
                .by_ref()
                .take(s.len() / 2)
                .map(|char| char.try_into())
                .collect::<Result<HashSet<Item>, _>>()?,
            second_compartment: chars
                .by_ref()
                .map(|char| char.try_into())
                .collect::<Result<HashSet<Item>, _>>()?,
        })
    }
}

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
struct Item {
    letter: char,
}

impl Item {
    fn priority(&self) -> usize {
        // Since the letters have values that increase from A to Z, we start with each letter's
        // ASCII value and offset it to simplify the calculation.
        match self.letter {
            'a'..='z' => self.letter as usize - 96,
            'A'..='Z' => self.letter as usize - 38,
            _ => unreachable!(),
        }
    }
}

impl TryFrom<char> for Item {
    type Error = anyhow::Error;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            'a'..='z' | 'A'..='Z' => Ok(Item { letter: value }),
            c => bail!("Invalid character {}", c),
        }
    }
}

fn read_lines() -> io::Result<io::Lines<io::BufReader<File>>> {
    let filename: String = env::args().skip(1).next().expect("Missing file path");
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
