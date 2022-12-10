use std::io::{BufRead, BufReader};
use std::fs::File;

pub mod spinoff;

struct Machine {
    x: i64,
    c: u64,
    buf: String,
}

impl Machine {
    fn new() -> Self {
        Machine {x: 1, c: 1, buf: String::new()}
    }

    fn cycle(&mut self) -> i64 {
        let r = if self.c % 40 == 20 && self.c <= 220 {
            self.c as i64 * self.x
        } else {
            0
        };
        
        self.buf.push(if ((self.buf.len() % 40) as i64 - self.x).abs() <= 1 {
            '#'
        } else {
            '.'
        });

        self.c += 1;
        r
    }
}

pub fn main() {
    let f = File::open("src/day10.txt").unwrap();
    let reader = BufReader::new(f);

    let mut machine = Machine::new();
    let mut total = 0;
    for ln in reader.lines().map(|x| x.unwrap()) {
        let cmd: Vec<_> = ln.split(' ').collect();

        match &cmd[..] {
            &["addx", n] => {
                total += machine.cycle();
                total += machine.cycle();
                machine.x += n.parse::<i64>().unwrap();
            }
            &["noop"] => {
                total += machine.cycle();
            }
            &[] | &[""] => break,
            _ => panic!("Unknown command {cmd:?}"),
        }
    }

    println!("{total:?}");

    for i in (0..=200).step_by(40) {
        println!("{}", &machine.buf[i..i + 40]);
    }
}


