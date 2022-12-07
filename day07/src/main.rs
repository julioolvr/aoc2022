use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::str::FromStr;

use anyhow::{bail, Context};
use petgraph::prelude::*;

const CAPACITY: usize = 70_000_000;
const UPDATE_SIZE: usize = 30_000_000;

/**
 * --- Day 7: No Space Left On Device ---
 *
 * The challenge provides input/output like a terminal running some `cd` and `ls` commands. It asks
 * to find different directory sizes following some rules.
 *
 * This program goes line by line inferring the filesystem from that. Lines can be either input
 * (commands) or output (node descriptions, where node is either a file or a directory). It
 * represents the filesystem with a graph structure that is used conceptually as a tree. Most of the
 * code is about interpreting the lines provided - calculating sizes is relatively straightforward
 * after that.
 *
 * Some things of the implementation are not enforced by the type-system and therefore rely on the
 * code not having bugs. One is that there's nothing preventing files from containing other nodes
 * (which would be a `Node::File` with outgoing edges). Another is that a given node can technically
 * have many parents (multiple incoming edges). The code assumes this isn't the case, but the data
 * structure does not prevent it.
 */
fn main() {
    let lines = read_lines().expect("Could not read file").map(|line| {
        line.expect("Could not read line")
            .parse()
            .expect("Could not parse line")
    });

    let mut filesystem = Filesystem::new(CAPACITY);
    filesystem.infer_from(lines);

    let part_1: usize = filesystem
        .directory_sizes()
        .filter(|size| *size <= 100_000)
        .sum();
    println!("Part 1: {}", part_1);

    let extra_space_needed = UPDATE_SIZE - filesystem.free_space();

    let part_2 = filesystem
        .directory_sizes()
        .filter(|size| *size >= extra_space_needed)
        .min()
        .expect("Could not find directory with minimum required space");
    println!("Part 2: {}", part_2);
}

struct Filesystem {
    storage: DiGraph<Node, ()>,
    capacity: usize,
}

impl Filesystem {
    fn new(capacity: usize) -> Self {
        let mut storage = DiGraph::new();
        storage.add_node(Node::Directory("/".into()));
        Filesystem { storage, capacity }
    }

    fn root(&self) -> NodeIndex {
        self.storage
            .externals(Incoming)
            .next()
            .expect("Could not find root directory")
    }

    fn free_space(&self) -> usize {
        self.capacity - self.node_size(self.root())
    }

    fn directory_sizes(&self) -> impl Iterator<Item = usize> + '_ {
        self.storage
            .node_indices()
            .filter_map(|node_index| match self.storage[node_index] {
                Node::Directory(_) => Some(self.node_size(node_index)),
                Node::File(_, _) => None,
            })
    }

    fn node_size(&self, node_index: NodeIndex) -> usize {
        match self.storage[node_index] {
            Node::File(_, size) => size,
            Node::Directory(_) => self
                .storage
                .edges_directed(node_index, Outgoing)
                .map(|edge| self.node_size(edge.target()))
                .sum(),
        }
    }

    fn infer_from(&mut self, lines: impl Iterator<Item = Line>) {
        let mut current_directory = self.root();

        for line in lines {
            match line {
                Line::Input(command) => match command {
                    Command::Cd(destination) => match destination.as_str() {
                        "/" => current_directory = self.root(),
                        ".." => {
                            let parent = self
                                .storage
                                .edges_directed(current_directory, Incoming)
                                .next()
                                .expect("Tried to find parent of root directory")
                                .source();
                            current_directory = parent;
                        }
                        name => {
                            let next_node_edge = self
                                .storage
                                .edges_directed(current_directory, Outgoing)
                                .find(|edge| {
                                    self.storage[edge.target()] == Node::Directory(name.into())
                                });

                            current_directory = if let Some(edge) = next_node_edge {
                                edge.target()
                            } else {
                                let new_directory =
                                    self.storage.add_node(Node::Directory(name.into()));
                                self.storage.add_edge(current_directory, new_directory, ());
                                new_directory
                            };
                        }
                    },
                    Command::Ls => {}
                },
                Line::Output(node_description) => {
                    let new_node = match node_description {
                        NodeDescription::File(name, size) => {
                            self.storage.add_node(Node::File(name, size))
                        }
                        NodeDescription::Directory(name) => {
                            self.storage.add_node(Node::Directory(name))
                        }
                    };

                    self.storage.add_edge(current_directory, new_node, ());
                }
            }
        }
    }
}

#[derive(PartialEq)]
enum Node {
    Directory(String),
    File(String, usize),
}

enum Line {
    Input(Command),
    Output(NodeDescription),
}

impl FromStr for Line {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.starts_with("$ ") {
            Ok(Line::Input(s.parse()?))
        } else {
            Ok(Line::Output(s.parse()?))
        }
    }
}

enum Command {
    Cd(String),
    Ls,
}

impl FromStr for Command {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.starts_with("$ cd ") {
            Ok(Command::Cd(s[5..].into()))
        } else if s == "$ ls" {
            Ok(Command::Ls)
        } else {
            bail!("Invalid command {}", s)
        }
    }
}

enum NodeDescription {
    Directory(String),
    File(String, usize),
}

impl FromStr for NodeDescription {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.starts_with("dir ") {
            Ok(NodeDescription::Directory(s[4..].into()))
        } else {
            let mut split = s.split(" ");
            let size = split
                .next()
                .context("Could not find file size")?
                .parse::<usize>()?;
            Ok(NodeDescription::File(
                split.next().context("Could not find file name")?.into(),
                size,
            ))
        }
    }
}

fn read_lines() -> io::Result<io::Lines<io::BufReader<File>>> {
    let filename: String = env::args().skip(1).next().expect("Missing file path");
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
