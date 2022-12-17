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
        if self.a <= other.b && self.b >= self.a {
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
}

pub fn main() {
    let f = File::open("src/day15.txt").unwrap();
    let reader = BufReader::new(f);

    let target_y = 2_000_000;
    let mut ranges = RList::new();

    for ln in reader.lines().map(|x| x.unwrap()) {
        if ln.is_empty() {break}

        let ln: Vec<_> = ln.split(&['=', ',', ':']).collect();
        let sx: i64 = ln[1].parse().unwrap();
        let sy: i64 = ln[3].parse().unwrap();
        let bx: i64 = ln[5].parse().unwrap();
        let by: i64 = ln[7].parse().unwrap();

        let d = (bx - sx).abs() + (by - sy).abs();
        let dtoy = (target_y - sy).abs();

        if d > dtoy {
            let range = R::new(sx + dtoy - d, sx + d - dtoy);
            ranges.add(range);
        }
    }

    println!("{ranges:?}");
    println!("{}", ranges.size());
}


