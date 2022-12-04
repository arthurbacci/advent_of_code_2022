use std::io::{BufRead, BufReader};
use std::fs::File;

pub fn main() {
    let f = File::open("src/day4.txt").unwrap();
    let reader = BufReader::new(f);

    let mut total: u32 = 0;
    let mut total2: u32 = 0;

    for ln in reader.lines() {
        let ln = ln.unwrap();
        if ln.is_empty() {break}
        
        let n: Vec<u32> = ln.split(',')
            .map(|x| x.split('-'))
            .flatten()
            .map(|x| x.parse().unwrap())
            .collect();

        for (a, b) in [((n[0], n[1]), (n[2], n[3])), ((n[2], n[3]), (n[0], n[1]))] {
            if a.0 >= b.0 && a.1 <= b.1 {
                total += 1;
                break;
            }
        }

        if n[0].max(n[2]) <= n[1].min(n[3]) {
            total2 += 1;
        }
    }

    println!("{total} {total2}");
}


