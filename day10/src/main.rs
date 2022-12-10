use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::str::FromStr;

use anyhow::{bail, Context};

fn main() {
    let mut cpu = Cpu::new();
    let program: Vec<Instruction> = read_lines()
        .expect("Unable to read file")
        .map(|line| {
            line.expect("Unable to read line")
                .parse()
                .expect("Unable to parse instruction")
        })
        .collect();

    cpu.load(program);

    let mut part_1 = 0;
    cpu.run_until_cycle(20);
    part_1 += cpu.signal_strength();
    cpu.run_until_cycle(60);
    part_1 += cpu.signal_strength();
    cpu.run_until_cycle(100);
    part_1 += cpu.signal_strength();
    cpu.run_until_cycle(140);
    part_1 += cpu.signal_strength();
    cpu.run_until_cycle(180);
    part_1 += cpu.signal_strength();
    cpu.run_until_cycle(220);
    part_1 += cpu.signal_strength();

    cpu.run_until_end();

    println!("\nPart 1: {}", part_1);
}

struct Cpu {
    cycle: usize,
    register_x: isize,
    program: Vec<Instruction>,
    program_pointer: f32,
}

impl Cpu {
    fn new() -> Self {
        Cpu {
            cycle: 1,
            register_x: 1,
            program: vec![],
            program_pointer: 0.0,
        }
    }

    fn load(&mut self, program: Vec<Instruction>) {
        self.program = program;
        self.program_pointer = 0.0;
    }

    fn run_until_cycle(&mut self, cycle_limit: usize) {
        while self.cycle < cycle_limit {
            let x_offset = ((self.cycle - 1) % 40) as isize;

            if x_offset == 0 {
                println!("")
            };

            if x_offset >= self.register_x - 1 && x_offset <= self.register_x + 1 {
                print!("#");
            } else {
                print!(".");
            }

            match self.program.get(self.program_pointer.trunc() as usize) {
                Some(Instruction::AddX(n)) => {
                    self.program_pointer += 0.5;

                    if self.program_pointer.fract() == 0.0 {
                        self.register_x += n;
                        // println!("New register_x {}", self.register_x);
                    }
                }
                Some(Instruction::Noop) => self.program_pointer += 1.0,
                None => break,
            }

            self.cycle += 1;
        }
    }

    fn run_until_end(&mut self) {
        // Heh
        self.run_until_cycle(999_999_999);
    }

    fn signal_strength(&self) -> isize {
        self.register_x * (self.cycle as isize)
    }
}

enum Instruction {
    Noop,
    AddX(isize),
}

impl FromStr for Instruction {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s == "noop" {
            Ok(Instruction::Noop)
        } else if s.starts_with("addx ") {
            let mut split = s.split(" ");
            split.next();
            Ok(Instruction::AddX(
                split
                    .next()
                    .context("Did not find addx value")?
                    .parse::<isize>()?,
            ))
        } else {
            bail!("Invalid instruction {}", s)
        }
    }
}

fn read_lines() -> io::Result<io::Lines<io::BufReader<File>>> {
    let filename: String = env::args().skip(1).next().expect("Missing file path");
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
