use std::env;
use std::fs::File;
use std::io::{self, BufRead};

/**
 * --- Day 1: Calorie Counting ---
 *
 * The input is a list of lists, each element is an elf and each element of the sublist is the
 * number of calories of the different snacks it carries.
 * Both parts of the problem care about the total calories carried by each elf, so this program
 * represents an elf as just a `usize` which is the total number of calories carried.
 * Not a whole lot going on - split lines, sum, sort, take the largest (or largest 3) to solve the
 * challenge.
 */
fn main() {
    let lines: Vec<String> = read_lines()
        .expect("Error reading file")
        .map(|line| line.expect("Error reading line"))
        .collect();

    let mut elves: Vec<usize> = lines
        .split(|line| line == "")
        .map(|calories| {
            calories
                .iter()
                .map(|calories_line| {
                    calories_line
                        .parse::<usize>()
                        .expect("Error parsing line as number")
                })
                .sum()
        })
        .collect();

    elves.sort_unstable();
    elves.reverse();

    let part_1 = elves.first().expect("Unable to find max calories");
    println!("Part 1: {}", part_1);

    let part_2: usize = elves.iter().take(3).sum();
    println!("Part 2: {}", part_2);
}

fn read_lines() -> io::Result<io::Lines<io::BufReader<File>>> {
    let filename: String = env::args().skip(1).next().expect("Missing file path");
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
