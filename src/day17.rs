use std::fs::File;
use std::io::{BufRead, BufReader};

enum Rock {
    Dash,
    Cross,
    L,
    Vbar,
    Block,
}

impl Rock {
    /// Sorted by y, reversed
    fn pieces(&self) -> &[(i64, i64)] {
        match self {
            Rock::Dash  => &[(0, 0), (1, 0), (2, 0), (3, 0)],
            Rock::Cross => &[(1, 2), (0, 1), (1, 1), (2, 1), (1, 0)],
            Rock::L     => &[(2, 2), (2, 1), (0, 0), (1, 0), (2, 0)],
            Rock::Vbar  => &[(0, 3), (0, 2), (0, 1), (0, 0)],
            Rock::Block => &[(0, 1), (1, 1), (0, 0), (1, 0)],
        }
    }
}

pub fn main() {
    let f = File::open("src/day17.txt").unwrap();
    let reader = BufReader::new(f);
    let line = reader.lines().next().unwrap().unwrap();
    let mut pattern = line
        .chars()
        .map(|x| match x {
            '<' | '>' => x,
            _ => panic!("Invalid direction {x}"),
        }).cycle();

    for y in (-1..=4).rev() {
        for x in -1..=4 {
            if Rock::Block.pieces().contains(&(x, y)) {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!();
    }
}


