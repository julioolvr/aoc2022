use std::collections::VecDeque;
use std::env;
use std::fs::File;
use std::io::{self, BufRead};

use regex::Regex;

fn main() {
    let mut monkeys: Vec<Monkey> = vec![];

    let is_part_1 = env::args().any(|arg| arg == "--part-1");

    let mut lines = read_lines()
        .expect("Unable to read file")
        .map(|line| line.expect("Unable to read line"));

    let number_regex = Regex::new(r"\d+").unwrap();

    while let Some(_) = lines.next() {
        let items: VecDeque<Item> = number_regex
            .find_iter(&lines.next().unwrap())
            .map(|worry_level| Item {
                worry: worry_level.as_str().parse().unwrap(),
            })
            .collect();

        let operation_line = lines.next().unwrap();
        let operation: Box<dyn Fn(usize) -> usize> = if operation_line.ends_with("old * old") {
            Box::new(|old| old * old)
        } else if operation_line.contains("old +") {
            let value: usize = number_regex
                .find(&operation_line)
                .unwrap()
                .as_str()
                .parse()
                .expect("Unable to parse operation constant");
            Box::new(move |old| old + value)
        } else {
            let value: usize = number_regex
                .find(&operation_line)
                .unwrap()
                .as_str()
                .parse()
                .expect("Unable to parse operation constant");
            Box::new(move |old| old * value)
        };

        let test_line = lines.next().unwrap();
        let test_divisor: usize = number_regex
            .find(&test_line)
            .unwrap()
            .as_str()
            .parse()
            .expect("Unable to parse test constant");

        let target_monkey_true: usize = number_regex
            .find(&lines.next().unwrap())
            .unwrap()
            .as_str()
            .parse()
            .expect("Unable to find true target monkey");
        let target_monkey_false: usize = number_regex
            .find(&lines.next().unwrap())
            .unwrap()
            .as_str()
            .parse()
            .expect("Unable to find false target monkey");

        monkeys.push(Monkey::new(
            items,
            operation,
            Test {
                divisor: test_divisor,
                on_true: target_monkey_true,
                on_false: target_monkey_false,
            },
        ));

        lines.next();
    }

    let lcm = monkeys
        .iter()
        .map(|monkey| monkey.test.divisor)
        .reduce(|acc, divisor| num::integer::lcm(acc, divisor))
        .unwrap()
        * 3;

    let rounds = if is_part_1 { 20 } else { 10_000 };

    for _ in 0..rounds {
        for i in 0..monkeys.len() {
            let items_count = monkeys[i].items.len();

            for _ in 0..items_count {
                let monkey = &mut monkeys[i];
                monkey.inspect_next_item(lcm);

                if is_part_1 {
                    monkey.adjust_worry_levels();
                }

                let target_index = monkey.next_target();
                let item = monkey.throw();
                monkeys[target_index].give(item);
            }
        }
    }

    let mut scores: Vec<usize> = monkeys
        .iter()
        .map(|monkey| monkey.inspections_count)
        .collect();
    scores.sort();
    scores.reverse();

    let part = if is_part_1 { 1 } else { 2 };
    let result = scores[0] * scores[1];
    println!("Part {}: {}", part, result);
}

struct Monkey {
    items: VecDeque<Item>,
    operation: Box<dyn Fn(usize) -> usize>,
    test: Test,
    inspections_count: usize,
}

impl Monkey {
    fn new(items: VecDeque<Item>, operation: Box<dyn Fn(usize) -> usize>, test: Test) -> Self {
        Monkey {
            items,
            operation,
            test,
            inspections_count: 0,
        }
    }

    fn inspect_next_item(&mut self, lcm: usize) {
        let new_worry = (self.operation)(self.items[0].worry);
        self.inspections_count += 1;
        self.items[0].worry = new_worry % lcm;
    }

    fn adjust_worry_levels(&mut self) {
        let new_worry = self.items[0].worry / 3;
        self.items[0].worry = new_worry;
    }

    fn next_target(&self) -> usize {
        self.test.evaluate(self.items[0].worry)
    }

    fn throw(&mut self) -> Item {
        self.items
            .pop_front()
            .expect("Tried to throw item from empty list")
    }

    fn give(&mut self, item: Item) {
        self.items.push_back(item);
    }
}

struct Item {
    worry: usize,
}

struct Test {
    divisor: usize,
    on_true: usize,
    on_false: usize,
}

impl Test {
    fn evaluate(&self, worry: usize) -> usize {
        if worry % self.divisor == 0 {
            self.on_true
        } else {
            self.on_false
        }
    }
}

fn read_lines() -> io::Result<io::Lines<io::BufReader<File>>> {
    let filename: String = env::args().skip(1).next().expect("Missing file path");
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
