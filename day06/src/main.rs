use std::collections::HashSet;
use std::env;
use std::fs::File;
use std::io::{self, BufRead};

/**
 * --- Day 6: Tuning Trouble ---
 *
 * The challenge provides a string that represent a signal. It's made out a bunch of characters.
 * Then it asks to find "markers", which are made of substrings of a given length with no repeated
 * characters.
 *
 * The program creates windows over the provided string and checks the number of unique elements on
 * each window by creating a set out of it. If it matches the length we're looking for, the marker
 * has been found.
 */
fn main() {
    let signal: Vec<char> = read_lines()
        .expect("Unable to read file")
        .next()
        .expect("Unable to find first line")
        .expect("Unable to read line")
        .chars()
        .collect();

    let part_1 = find_marker(&signal, 4).expect("Could not find start-of-packet marker");
    println!("Part 1: {}", part_1);

    let part_2 = find_marker(&signal, 14).expect("Could not find start-of-message marker");
    println!("Part 2: {}", part_2);
}

fn find_marker(signal: &Vec<char>, length: usize) -> Option<usize> {
    signal
        .windows(length)
        .map(|window| HashSet::<&char>::from_iter(window.iter()).len())
        .enumerate()
        .find_map(|(i, unique_char_count)| {
            if unique_char_count == length {
                Some(i + length)
            } else {
                None
            }
        })
}

fn read_lines() -> io::Result<io::Lines<io::BufReader<File>>> {
    let filename: String = env::args().skip(1).next().expect("Missing file path");
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
