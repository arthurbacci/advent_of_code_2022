use std::io::{BufRead, BufReader};
use std::fs::File;
use std::collections::HashSet;

pub fn main() {
    let f = File::open("src/day9.txt").unwrap();
    let reader = BufReader::new(f);

    let mut h_pos = (0i32, 0i32);
    let mut t_pos = (0i32, 0i32);
    let mut t_history = HashSet::new();
    t_history.insert(t_pos);
    
    let mut lines = reader.lines();
    let mut acc = 0u32;
    let mut dir = '_';
    loop {
        if acc == 0 {
            let ln = match lines.next() {
                Some(ln) => ln,
                None => break,
            }.unwrap();
            let ln: Vec<&str> = ln.split(' ').collect();
            if ln.len() != 2 {return}
            
            dir = ln[0].chars().next().unwrap();
            acc = ln[1].parse().unwrap();
        }
        acc -= 1;
        
        match dir {
            'U' => h_pos.1 -= 1,
            'D' => h_pos.1 += 1,
            'L' => h_pos.0 -= 1,
            'R' => h_pos.0 += 1,
            c => panic!("Invalid character {c:?}"),
        }

        if (t_pos.1 - h_pos.1).abs() > 1 || (t_pos.0 - h_pos.0).abs() > 1 {
            if t_pos.1 != h_pos.1  {
                t_pos.1 += if h_pos.1 > t_pos.1 {1} else {-1};
            }
            if t_pos.0 != h_pos.0 {
                t_pos.0 += if h_pos.0 > t_pos.0 {1} else {-1};
            }
        }

        t_history.insert(t_pos);


        for y in -6..6 {
            for x in -6..6 {
                if (x, y) == h_pos {
                    print!("H");
                } else if (x, y) == t_pos {
                    print!("T");
                } else if (x, y) == (0, 0) {
                    print!("s");
                } else {
                    print!(".");
                }
            }
            println!();
        }
        println!();
    }

    println!("{}", t_history.len());
}


