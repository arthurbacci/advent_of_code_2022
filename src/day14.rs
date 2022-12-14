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

pub fn main() {
    let f = File::open("src/day14.txt").unwrap();
    let reader = BufReader::new(f);

    let mut rocks = HashSet::new();
    let mut maximums = HashMap::new();
    let mut maximum = i64::MIN;

    for ln in reader.lines().map(|x| x.unwrap()) {
        if ln.is_empty() {break}

        let mut positions = ln.split(" -> ")
            .map(|x| x.split(",").map(|y| y.parse::<i64>().unwrap()))
            .map(|mut x| (x.next().unwrap(), x.next().unwrap()));


        let mut pos = positions.next().unwrap();
        for i in positions {
            let dir = direction_to(pos, i);
            loop {
                match maximums.get(&pos.0) {
                    None => {
                        maximums.insert(pos.0, pos.1);
                    }
                    Some(&x) if pos.1 > x => {
                        maximums.insert(pos.0, pos.1);
                    }
                    _ => {}
                }
                if pos.1 > maximum {
                    maximum = pos.1;
                }
                rocks.insert((pos.0, pos.1));
                
                if pos == i {break}
                
                pos.0 += dir.0;
                pos.1 += dir.1;
            }
        }
    }

    let old_rocks = rocks.clone();

    /* SAND~SIMULATION */
    'outer: for i in 0.. {
        let mut pos = (500, 0);

        while pos.1 <= *maximums.get(&pos.0).unwrap_or(&i64::MIN) {
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
                match maximums.get(&pos.0) {
                    None => {
                        maximums.insert(pos.0, pos.1);
                    }
                    Some(&x) if pos.1 > x => {
                        maximums.insert(pos.0, pos.1);
                    }
                    _ => {}
                }
                continue 'outer;
            }
        }
        
        println!("{i}");
        break;
    }

    let mut rocks = old_rocks;

    /* PART~TWO */
    'outer: for i in 1.. {
        let mut pos = (500, 0);

        loop {
            let down = (pos.0, pos.1 + 1);
            let left = (pos.0 - 1, pos.1 + 1);
            let right = (pos.0 + 1, pos.1 + 1);

            let blocked = pos.1 == maximum + 1;

            if !blocked && rocks.get(&down).is_none() {
                pos = down;
            } else if !blocked && rocks.get(&left).is_none() {
                pos = left;
            } else if !blocked && rocks.get(&right).is_none() {
                pos = right;
            } else {
                rocks.insert((pos.0, pos.1));

                if pos == (500, 0) {break}

                continue 'outer;
            }
        }
        
        println!("{i}");
        break;
    }

}

