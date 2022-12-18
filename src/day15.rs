use std::fs::File;
use std::io::{BufRead, BufReader};
use std::fmt;
use std::collections::HashSet;

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
struct R {
    a: i64,
    b: i64,
}

impl R {
    fn new(a: i64, b: i64) -> R {R {a, b}}

    fn intersects(self, other: R) -> bool {
        if self.a <= other.b && self.b >= other.a {
            true
        } else {
            false
        }
    }

    fn combine(self, other: R) -> Option<R> {
        if !self.intersects(other) {
            None
        } else {
            Some(R::new(self.a.min(other.a), self.b.max(other.b)))
        }
    }

    fn size(&self) -> i64 {
        self.b - self.a
    }

    fn intersect(self, other: R) -> Option<R> {
        let a = self.a.max(other.a);
        let b = self.b.min(other.b);
        if b >= a {
            Some(R::new(a, b))
        } else {
            None
        }
    }
}

impl fmt::Debug for R {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[{}, {}]", self.a, self.b)
    }
}

#[derive(Debug)]
struct RList {
    set: HashSet<R>,
}

impl RList {
    fn new() -> RList {RList {set: HashSet::new()}}

    fn add(&mut self, mut r: R) {
        let mut to_rmv = vec![];

        for i in &self.set {
            if let Some(x) = r.combine(*i) {
                r = x;
                to_rmv.push(*i);
            }
        }
        for i in to_rmv {
            self.set.remove(&i);
        }
        self.set.insert(r);
    }

    fn size(&self) -> i64 {
        let mut r = 0;
        for i in &self.set {
            r += i.size();
        }
        r
    }

    fn intersect(&self, r: R) -> RList {
        let mut set = HashSet::new();
        for &i in &self.set {
            if let Some(x) = r.intersect(i) {
                set.insert(x);
            }
        }
        RList {set}
    }
}

pub fn main() {
    let f = File::open("src/day15.txt").unwrap();
    let reader = BufReader::new(f);

    let part1 = 2_000_000;
    let part2 = 4_000_000;

    let mut beacons = HashSet::new();
    for ln in reader.lines().map(|x| x.unwrap()) {
        if ln.is_empty() {break}

        let ln: Vec<_> = ln.split(&['=', ',', ':']).collect();
        let sx: i64 = ln[1].parse().unwrap();
        let sy: i64 = ln[3].parse().unwrap();
        let bx: i64 = ln[5].parse().unwrap();
        let by: i64 = ln[7].parse().unwrap();

        let d = (bx - sx).abs() + (by - sy).abs();
        beacons.insert((sx, sy, d));
    }

    let target_y = part1;
    let mut ranges = RList::new();

    for &(x, y, d) in &beacons {
        let dtoy = (target_y - y).abs();

        if d > dtoy {
            let range = R::new(x + dtoy - d, x + d - dtoy);
            ranges.add(range);
        }
    }


    println!("{ranges:?}");
    println!("{}", ranges.size());

    'outer: for target_y in 0..=part2 {
        if target_y % 10000 == 0 {
            println!("{target_y}");
        }

        let mut ranges = RList::new();

        for &(x, y, d) in &beacons {
            let dtoy = (target_y - y).abs();

            if d > dtoy {
                let range = R::new(x + dtoy - d, x + d - dtoy);
                ranges.add(range);
            }
        }

        let inter = ranges.intersect(R::new(0, part2));
        if inter.size() < part2 {
            for i in 0..=part2 {
                if ranges.intersect(R::new(i, i)).set.len() == 0 {
                    println!("{:?}", (i, target_y));
                    println!("{}", i * 4_000_000 + target_y);
                    break 'outer;
                }
            }
        }
    }
}



