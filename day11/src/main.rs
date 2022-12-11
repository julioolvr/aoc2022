use std::collections::VecDeque;
use std::env;
use std::fs::File;
use std::io::{self, BufRead};

use regex::Regex;

fn main() {
    let mut monkeys: Vec<Monkey> = vec![];

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
        let test_value: usize = number_regex
            .find(&test_line)
            .unwrap()
            .as_str()
            .parse()
            .expect("Unable to parse test constant");
        let test = Box::new(move |value: usize| value % test_value == 0);

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
                condition: test,
                on_true: target_monkey_true,
                on_false: target_monkey_false,
            },
        ));

        lines.next();
    }

    for _ in 0..20 {
        for i in 0..monkeys.len() {
            let items_count = monkeys[i].items.len();

            for _ in 0..items_count {
                let monkey = &mut monkeys[i];
                monkey.inspect_next_item();
                monkey.adjust_worry_levels();
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
    let part_1 = scores[0] * scores[1];
    println!("Part 1: {}", part_1);
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

    fn inspect_next_item(&mut self) {
        let new_worry = (self.operation)(self.items[0].worry);
        self.inspections_count += 1;
        self.items[0].worry = new_worry;
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
    condition: Box<dyn Fn(usize) -> bool>,
    on_true: usize,
    on_false: usize,
}

impl Test {
    fn evaluate(&self, worry: usize) -> usize {
        if (self.condition)(worry) {
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
