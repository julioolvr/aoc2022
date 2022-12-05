use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::str::FromStr;

use anyhow::Context;

/**
 * --- Day 5: Supply Stacks ---
 *
 * The challenge provides several stacks, each of them with crates (where each
 * crate is identified by a single letter). Then it provides a series of
 * instructions that do operations on crates, moving them across stacks.
 *
 * The program parses initial stacks and instructions separately. Then creates
 * two different cranes with different `run` implementations, gives each a copy
 * of the initial positions of the stacks and runs the instructions. The solution
 * is formed by taking the top element of each stack.
 */

fn main() {
    let mut lines = read_lines()
        .expect("Unable to read file")
        .map(|line| line.expect("Unable to read line"));

    let starting_position: Vec<String> = lines.by_ref().take_while(|line| line != "").collect();
    let instructions: Vec<Instruction> = lines
        .map(|line| line.parse().expect("Unable to parse instruction line"))
        .collect();

    let starting_stacks = parse_stacks(starting_position);

    let mut crane = CrateMover9000 {
        stacks: starting_stacks.clone(),
    };
    crane.run(&instructions);
    println!("Part 1 {:?}", crane.top_krates());

    let mut crane = CrateMover9001 {
        stacks: starting_stacks.clone(),
    };
    crane.run(&instructions);
    println!("Part 2 {:?}", crane.top_krates());
}

fn parse_stacks(mut lines: Vec<String>) -> Vec<Stack> {
    let mut stacks = vec![];

    // Reversing the lines simplifies parsing. That way the first line will have the
    // ids of the stacks, and we go from bottom to top for each of them (which is the
    // order that is more convenient to "push" into each stack).
    lines.reverse();

    for _ in lines.first().unwrap().split_whitespace() {
        // We could parse this, but we know that it will be the numbers 1, 2, 3...
        stacks.push(Stack::new());
    }

    for line in lines.iter().skip(1) {
        let mut chars = line.chars();

        // We know the lines will have the structure [X] [Y] [Z] with different
        // amounts of whitespace between them, and that whitespace will be
        // significant in order to know which stack each crate belongs to.
        // So we go character by character, dropping the square brackets and
        // whitespaces and if we find a letter we add it to the corresponding stack
        for stack in stacks.iter_mut() {
            chars.next(); // Drop [ or whitespace
            let krate = chars.next();

            match krate {
                Some(krate) => {
                    if krate != ' ' {
                        stack.push(krate);
                    }

                    chars.next(); // Drop ] or whitespace
                    chars.next(); // Drop whitespace after the crate
                }
                // None means that we reached end of line, no need to keep looking
                // for crates.
                None => break,
            };
        }
    }

    stacks
}

trait Crane {
    fn run(&mut self, instructions: &Vec<Instruction>);

    fn stacks(&self) -> &Vec<Stack>;

    fn top_krates(&self) -> String {
        self.stacks()
            .iter()
            .filter_map(|stack| stack.peek())
            .collect()
    }
}

struct CrateMover9000 {
    stacks: Vec<Stack>,
}

impl Crane for CrateMover9000 {
    fn stacks(&self) -> &Vec<Stack> {
        &self.stacks
    }

    fn run(&mut self, instructions: &Vec<Instruction>) {
        for instruction in instructions {
            for _ in 0..instruction.movement {
                let krate = self.stacks[instruction.from]
                    .pop()
                    .expect("Popping from an empty stack");
                self.stacks[instruction.to].push(krate);
            }
        }
    }
}

struct CrateMover9001 {
    stacks: Vec<Stack>,
}

impl Crane for CrateMover9001 {
    fn stacks(&self) -> &Vec<Stack> {
        &self.stacks
    }

    fn run(&mut self, instructions: &Vec<Instruction>) {
        for instruction in instructions {
            let next_group = self.stacks[instruction.from].take_from_top(instruction.movement);
            self.stacks[instruction.to].extend(next_group);
        }
    }
}

#[derive(Debug, Clone)]
struct Stack(Vec<char>);

impl Stack {
    fn new() -> Stack {
        Stack(vec![])
    }

    fn pop(&mut self) -> Option<char> {
        self.0.pop()
    }

    fn push(&mut self, krate: char) {
        self.0.push(krate)
    }

    fn peek(&self) -> Option<&char> {
        self.0.last()
    }

    fn take_from_top(&mut self, amount: usize) -> Vec<char> {
        let stack_length = self.0.len();
        self.0.split_off(stack_length - amount)
    }

    fn extend(&mut self, other: Vec<char>) {
        self.0.extend(other);
    }
}

#[derive(Debug)]
struct Instruction {
    movement: usize,
    from: usize,
    to: usize,
}

impl FromStr for Instruction {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut splits = s.split_whitespace();
        splits.next(); // Drop word "move"
        let movement: usize = splits.next().context("Missing move amount")?.parse()?;
        splits.next(); // Drop word "from"
        let from: usize = splits.next().context("Missing move from")?.parse()?;
        splits.next(); // Drop word "to"
        let to: usize = splits.next().context("Missing move to")?.parse()?;

        // Instructions are 1-indexed but that's inconvenient so we parse them to
        // 0-indexed early.
        Ok(Instruction {
            movement,
            from: from - 1,
            to: to - 1,
        })
    }
}

fn read_lines() -> io::Result<io::Lines<io::BufReader<File>>> {
    let filename: String = env::args().skip(1).next().expect("Missing file path");
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
