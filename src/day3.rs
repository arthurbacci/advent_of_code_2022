use std::io::{BufRead, BufReader};
use std::fs::File;

fn to_priority(s: u8) -> u8 {
    match s {
        b'a'..=b'z' => s - b'a' + 1,
        b'A'..=b'Z' => s - b'A' + 27,
        _ => panic!("Unexpected character"),
    }
}

fn divide(s: &str) -> (&str, &str) {
    (&s[..(s.len() / 2)], &s[(s.len() / 2)..])
}

fn part1() -> u32 {
    let f = File::open("src/day3.txt").unwrap();
    let reader = BufReader::new(f);

    let mut total: u32 = 0;

    for ln in reader.lines() {
        let ln = ln.unwrap();
        if ln.is_empty() {break}

        let mut in1 = [false; 53];
        let mut in2 = [false; 53];

        let (part1, part2) = divide(&ln);
        

        for (i, j) in part1.chars().zip(part2.chars()) {
            in1[to_priority(i.try_into().unwrap()) as usize] = true;
            in2[to_priority(j.try_into().unwrap()) as usize] = true;
        }

        for (n, (i, j)) in in1.iter().zip(in2.iter()).enumerate() {
            if *i && *j {
                total += n as u32;
            }
        }
    }

    total
}

fn part2() -> u32 {
    let f = File::open("src/day3.txt").unwrap();
    let reader = BufReader::new(f);

    let mut total: u32 = 0;

    let mut iter = reader.lines();

    loop {
        let ln = (iter.next(), iter.next(), iter.next());
        let ln = match ln {
            (Some(Ok(a)), Some(Ok(b)), Some(Ok(c))) => (a, b, c),
            _ => break,
        };
        
        let parts = [&ln.0, &ln.1, &ln.2];
        
        let mut arrs = [[false; 53]; 3];
        for i in 0..3 {
            for j in parts[i].chars() {
                arrs[i][to_priority(j.try_into().unwrap()) as usize] = true;
            }
        }

        for i in 0..53 {
            let mut t = true;
            for j in arrs {
                t = t && j[i];
            }
            if t {
                total += i as u32;
            }
        }
    }

    total
}

pub fn main() {
    let total = part1();
    println!("{total}");

    let total = part2();
    println!("{total}");
}


