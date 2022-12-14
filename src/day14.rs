use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::{HashMap, HashSet};
use std::cmp::Ordering;


fn direction_to(a: (i64, i64), b: (i64, i64)) -> (i64, i64) {
    (
        match a.0.cmp(&b.0) {
            Ordering::Less => 1,
            Ordering::Equal => 0,
            Ordering::Greater => -1,
        },
        match a.1.cmp(&b.1) {
            Ordering::Less => 1,
            Ordering::Equal => 0,
            Ordering::Greater => -1,
        },
    )
}

fn print_sand(rocks: &HashSet<(i64, i64)>) {
    for y in 0..=10 {
        for x in 490..=505 {
            if rocks.get(&(x, y)).is_some() {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!();
    }
    println!();
}

pub fn main() {
    let f = File::open("src/day14.txt").unwrap();
    let reader = BufReader::new(f);

    let mut rocks = HashSet::new();
    let mut minimums = HashMap::new();

    for ln in reader.lines().map(|x| x.unwrap()) {
        if ln.is_empty() {break}

        let mut positions = ln.split(" -> ")
            .map(|x| x.split(",").map(|y| y.parse::<i64>().unwrap()))
            .map(|mut x| (x.next().unwrap(), x.next().unwrap()));


        let mut pos = positions.next().unwrap();
        for i in positions {
            let dir = direction_to(pos, i);
            loop {
                match minimums.get(&pos.0) {
                    None => {
                        minimums.insert(pos.0, pos.1);
                    }
                    Some(&x) if pos.1 > x => {
                        minimums.insert(pos.0, pos.1);
                    }
                    _ => {}
                }
                rocks.insert((pos.0, pos.1));
                
                if pos == i {break}
                
                pos.0 += dir.0;
                pos.1 += dir.1;
            }
        }
    }

    /* SAND~SIMULATION */
    'outer: for i in 0.. {
        let mut pos = (500, 0);

        while pos.1 <= *minimums.get(&pos.0).unwrap_or(&i64::MIN) {
            let down = (pos.0, pos.1 + 1);
            let left = (pos.0 - 1, pos.1 + 1);
            let right = (pos.0 + 1, pos.1 + 1);

            if rocks.get(&down).is_none() {
                pos = down;
            } else if rocks.get(&left).is_none() {
                pos = left;
            } else if rocks.get(&right).is_none() {
                pos = right;
            } else {
                rocks.insert((pos.0, pos.1));
                match minimums.get(&pos.0) {
                    None => {
                        minimums.insert(pos.0, pos.1);
                    }
                    Some(&x) if pos.1 > x => {
                        minimums.insert(pos.0, pos.1);
                    }
                    _ => {}
                }
                print_sand(&rocks);
                continue 'outer;
            }
        }
        
        println!("{i}");
        break;
    }

}

