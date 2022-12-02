use std::env;
use std::fs::File;
use std::io::{self, BufRead};

use anyhow::{bail, Context};

/**
 * --- Day 2: Rock Paper Scissors ---
 *
 * The challenge provides an input list to be interpreted as Rock-Paper-Scissors games, but each
 * part asks to interpret the input in different ways. Each line has two characters. The first part
 * asks to interpret the first character as the opponent's play, and the second one as our play.
 * The second part interprets the first character as the opponent's play, and the second one as the
 * expected result of the match. There are rules to calculate a score for each match depending on
 * our own play and the result, and the solution is the score for the given input.
 *
 * The program has two entry points, one for each part. It parses the lines accordingly, and then
 * handles it to the logic that represents the shapes of the game to find the missing piece (the
 * result in part 1, and our own play in part 2).
 */

fn main() -> anyhow::Result<()> {
    let lines = read_lines()?
        .map(|line| split_line(&line?).context("Unable to split line"))
        .collect::<Result<Vec<(char, char)>, _>>()?;

    let part_1: usize = lines
        .iter()
        .try_fold(0, |acc, line| -> Result<usize, anyhow::Error> {
            Ok(acc + parsed_as_plays(*line)?)
        })?;
    println!("Part 1: {}", part_1);

    let part_2: usize = lines
        .iter()
        .try_fold(0, |acc, line| -> Result<usize, anyhow::Error> {
            Ok(acc + parsed_as_results(*line)?)
        })?;
    println!("Part 2: {}", part_2);

    Ok(())
}

fn parsed_as_plays((them, me): (char, char)) -> anyhow::Result<usize> {
    let them = Shape::from_their_play(them)?;

    let me = match me {
        'X' => Shape::Rock,
        'Y' => Shape::Paper,
        'Z' => Shape::Scissors,
        c => bail!("Invalid character for own play: {}", c),
    };

    let result = me.play_against(&them);

    Ok(me.score() + result.score())
}

fn parsed_as_results((them, result): (char, char)) -> anyhow::Result<usize> {
    let them = Shape::from_their_play(them)?;

    let result = match result {
        'X' => MatchResult::Lose,
        'Y' => MatchResult::Draw,
        'Z' => MatchResult::Win,
        c => bail!("Invalid character for result: {}", c),
    };

    let me = them.necessary_for_result(&result);

    Ok(me.score() + result.score())
}

#[derive(PartialEq, Clone)]
enum Shape {
    Rock,
    Paper,
    Scissors,
}

impl Shape {
    fn from_their_play(char: char) -> anyhow::Result<Shape> {
        let shape = match char {
            'A' => Shape::Rock,
            'B' => Shape::Paper,
            'C' => Shape::Scissors,
            c => bail!("Invalid character for other's play: {}", c),
        };

        Ok(shape)
    }

    fn score(&self) -> usize {
        use Shape::*;

        match self {
            Rock => 1,
            Paper => 2,
            Scissors => 3,
        }
    }

    fn defeats(&self) -> Shape {
        use Shape::*;

        match self {
            Rock => Scissors,
            Paper => Rock,
            Scissors => Paper,
        }
    }

    fn defeated_by(&self) -> Shape {
        // This is a beat of a cheat. We know that there are three shapes in RPS, and that they form
        // a loop on each one beating the other. So if we start with R, we have
        // R > S > P > R. Calling `.defeats()` on R gets us S, and calling `.defeats()` on S gives
        // us P, which defeats R, which is what we're looking for.
        self.defeats().defeats()
    }

    fn play_against(&self, other: &Shape) -> MatchResult {
        if self.beats(other) {
            MatchResult::Win
        } else if other.beats(self) {
            MatchResult::Lose
        } else {
            MatchResult::Draw
        }
    }

    fn beats(&self, other: &Shape) -> bool {
        self.defeats() == *other
    }

    // Given a result, it will return which other shape is needed to achieve the expected result
    // against self.
    fn necessary_for_result(&self, result: &MatchResult) -> Shape {
        match result {
            MatchResult::Draw => self.clone(),
            MatchResult::Lose => self.defeats(),
            MatchResult::Win => self.defeated_by(),
        }
    }
}

#[derive(PartialEq)]
enum MatchResult {
    Win,
    Draw,
    Lose,
}

impl MatchResult {
    fn score(&self) -> usize {
        use MatchResult::*;

        match self {
            Win => 6,
            Draw => 3,
            Lose => 0,
        }
    }
}

fn read_lines() -> io::Result<io::Lines<io::BufReader<File>>> {
    let filename: String = env::args().skip(1).next().expect("Missing file path");
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn split_line(line: &str) -> Option<(char, char)> {
    let mut chars = line.chars();
    let a = chars.next();
    chars.next(); // drop whitespace character
    let b = chars.next();

    match (a, b) {
        (Some(a), Some(b)) => Some((a, b)),
        _ => None,
    }
}
