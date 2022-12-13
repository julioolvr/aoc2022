#![feature(iter_array_chunks)]

use std::cmp::Ordering;
use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::str::FromStr;

use anyhow::{bail, Context};
use serde_json::Value;

fn main() {
    let mut packets: Vec<Packet> = read_lines()
        .expect("Unable to read file")
        .map(|line| line.expect("Unable to read line"))
        .filter(|line| line != "")
        .map(|line| {
            line.parse::<Packet>()
                .expect("Unable to parse line as data")
        })
        .collect();

    let part_1: usize = packets
        .iter()
        .array_chunks()
        .enumerate()
        .filter_map(|(i, [first, second])| if first < second { Some(i + 1) } else { None })
        .sum();
    println!("Part 1: {}", part_1);

    let divider_1: Packet = "[[2]]".parse().unwrap();
    let divider_2: Packet = "[[6]]".parse().unwrap();
    packets.push(divider_1.clone());
    packets.push(divider_2.clone());

    packets.sort();

    let divider_1_index = packets
        .iter()
        .position(|packet| packet == &divider_1)
        .unwrap();
    let divider_2_index = packets
        .iter()
        .position(|packet| packet == &divider_2)
        .unwrap();

    let part_2 = (divider_1_index + 1) * (divider_2_index + 1);
    println!("Part 2: {}", part_2);
}

#[derive(PartialEq, Eq, Clone)]
struct Packet {
    data: Data,
}

impl FromStr for Packet {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let data: Data = s.parse()?;

        match data {
            Data::List(_) => Ok(Packet { data }),
            Data::Value(_) => bail!("Packets require a list as top-level data"),
        }
    }
}

impl PartialOrd for Packet {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.data.partial_cmp(&other.data)
    }
}

impl Ord for Packet {
    fn cmp(&self, other: &Self) -> Ordering {
        self.data
            .partial_cmp(&other.data)
            .expect("Packets should be fully orderable")
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum Data {
    Value(usize),
    List(Vec<Data>),
}

impl PartialOrd for Data {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match (self, other) {
            (Data::Value(left), Data::Value(right)) => left.partial_cmp(right),
            (Data::Value(left), Data::List(_)) => {
                Data::List(vec![Data::Value(*left)]).partial_cmp(other)
            }
            (Data::List(_), Data::Value(right)) => {
                self.partial_cmp(&Data::List(vec![Data::Value(*right)]))
            }
            (Data::List(left), Data::List(right)) => {
                for i in 0..left.len() {
                    let left = &left[i];
                    let right = right.get(i);

                    match right {
                        Some(data) => match left.partial_cmp(data) {
                            ordering @ Some(Ordering::Less)
                            | ordering @ Some(Ordering::Greater) => return ordering,
                            _ => {}
                        },
                        None => return Some(Ordering::Greater),
                    }
                }

                if left.len() == right.len() {
                    None
                } else {
                    Some(Ordering::Less)
                }
            }
        }
    }
}

impl TryFrom<&Value> for Data {
    type Error = anyhow::Error;

    fn try_from(value: &Value) -> Result<Self, Self::Error> {
        match value {
            Value::Number(number) => Ok(Data::Value(
                number
                    .as_u64()
                    .context("Could not parse number as integer")? as usize,
            )),
            Value::Array(list) => Ok(Data::List(
                list.iter()
                    .map(|value| value.try_into())
                    .collect::<Result<Vec<Data>, _>>()?,
            )),
            _ => bail!("Unable to turn JSON into Data"),
        }
    }
}

impl FromStr for Data {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let as_json: Value = serde_json::from_str(s)?;
        (&as_json).try_into()
    }
}

fn read_lines() -> io::Result<io::Lines<io::BufReader<File>>> {
    let filename: String = env::args().skip(1).next().expect("Missing file path");
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
