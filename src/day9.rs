use std::io::{BufRead, BufReader};
use std::fs::File;
use std::collections::HashSet;

fn follow(me: &mut (i32, i32), to_follow: (i32, i32)) {
    if (me.1 - to_follow.1).abs() > 1 || (me.0 - to_follow.0).abs() > 1 {
        if me.1 != to_follow.1  {
            me.1 += if to_follow.1 > me.1 {1} else {-1};
        }
        if me.0 != to_follow.0 {
            me.0 += if to_follow.0 > me.0 {1} else {-1};
        }
    }
}

pub fn main() {
    let f = File::open("src/day9.txt").unwrap();
    let reader = BufReader::new(f);

    let mut h_pos = (0i32, 0i32);
    let mut followers = [(0i32, 0i32); 9];
    let mut t_history = HashSet::new();
    let mut tail_history = HashSet::new();
    t_history.insert(followers[0]);
    tail_history.insert(followers[8]);
    
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

        follow(&mut followers[0], h_pos);
        for i in 1..9 {
            let before = followers[i - 1];
            follow(&mut followers[i], before);
        }
        t_history.insert(followers[0]);
        tail_history.insert(followers[8]);


        /*for y in -6..6 {
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
        println!();*/
    }

    println!("{} {}", t_history.len(), tail_history.len());
}


