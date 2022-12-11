use std::fs::File;
use std::io::{BufRead, BufReader};
use std::iter;
use std::collections::VecDeque;

struct Monkey {
    items: VecDeque<u64>,
    operation: Box<dyn Fn(u64) -> u64>,
    divisible_by: u64,
    throw_to: (usize, usize),
}

impl Monkey {
    fn parse<T: Iterator<Item=String>>(lines: &mut T) -> Option<(usize, Monkey)> {
        let first = lines.next()?;

        let index: usize = first
            .strip_prefix("Monkey ")?
            .strip_suffix(":").unwrap()
            .parse().unwrap();
        
        let items: VecDeque<u64> = lines.next().unwrap()
            .strip_prefix("  Starting items: ").unwrap()
            .split(", ")
            .map(|x| x.parse().unwrap())
            .collect();

        let third_line = lines.next().unwrap();
        let operation_str = third_line
            .strip_prefix("  Operation: new = old ").unwrap();

        let operation = operation_str.chars().next().unwrap();
        let operand = operation_str[1..].strip_prefix(" ").unwrap();
        let operation: Box<dyn Fn(u64) -> u64> = match operand {
            "old" => match operation {
                '*' => Box::new(|x| x * x),
                '+' => Box::new(|x| x + x),
                _ => panic!("Unknown operation {operation}"),
            }
            n => {
                let n: u64 = n.parse().unwrap();
                match operation {
                    '*' => Box::new(move |x| x * n),
                    '+' => Box::new(move |x| x + n),
                    _ => panic!("Unknown operation {operation}"),
                }
            }
        };

        let divisible_by: u64 = lines.next().unwrap()
            .strip_prefix("  Test: divisible by ").unwrap()
            .parse().unwrap();
        
        let throw_to: (usize, usize) = (
            lines.next().unwrap()
                .strip_prefix("    If true: throw to monkey ").unwrap()
                .parse().unwrap(),
            lines.next().unwrap()
                .strip_prefix("    If false: throw to monkey ").unwrap()
                .parse().unwrap(),
        );
        

        Some((
            index,
            Monkey {items, operation, divisible_by, throw_to},
        ))
    }

    fn round(&mut self, divide: bool) -> Vec<(usize, u64)> {
        let mut ret = Vec::new();

        while let Some(mut item) = self.items.pop_front() {
            item = (self.operation)(item);
            if divide {
                item /= 3;
            }
            
            ret.push((
                if item % self.divisible_by == 0 {
                    self.throw_to.0
                } else {
                    self.throw_to.1
                },
                item,
            ));
        }

        ret
    }
}

fn both_parts(part2: bool) {
    let file = File::open("src/day11.txt").unwrap();
    let reader = BufReader::new(file);
    let mut lines = reader.lines()
        .map(|x| x.unwrap())
        .chain(iter::once(" ".to_string()));

    let mut monkeys = Vec::new();
    while let Some((index, monkey)) = Monkey::parse(&mut lines) {
        if index != monkeys.len() {
            panic!("Index doesn't matches")
        }
        monkeys.push(monkey);

        lines.next().unwrap();
    }

    let mut value_divided = 1;
    for monkey in &monkeys {
        value_divided *= monkey.divisible_by;
        println!("{} {}", monkey.divisible_by, value_divided);
    }

    let mut totals = vec![0; monkeys.len()];
    for _ in 0..(if part2 {10000} else {20}) {
        for i in 0..monkeys.len() {
            let to_adds = monkeys[i].round(!part2);
            totals[i] += to_adds.len();
            for (i, el) in to_adds {
                monkeys[i].items.push_back(el % value_divided);
            }
        }
    }

    let mut maximums = (0, 0);
    for i in totals {
        if i > maximums.0 {
            maximums = (i, maximums.0);
        } else if i > maximums.1 {
            maximums.1 = i;
        }
    }

    println!("{}", maximums.0 * maximums.1);
}

pub fn main() {
    both_parts(false);
    both_parts(true);
}

