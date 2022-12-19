use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug, Clone, Copy)]
enum Rock {
    Dash,
    Cross,
    L,
    Vbar,
    Block,
}

impl Rock {
    fn pieces(&self) -> &[(i64, i64)] {
        match self {
            Rock::Dash  => &[(0, 0), (1, 0), (2, 0), (3, 0)],
            Rock::Cross => &[(1, 2), (0, 1), (1, 1), (2, 1), (1, 0)],
            Rock::L     => &[(2, 2), (2, 1), (0, 0), (1, 0), (2, 0)],
            Rock::Vbar  => &[(0, 3), (0, 2), (0, 1), (0, 0)],
            Rock::Block => &[(0, 1), (1, 1), (0, 0), (1, 0)],
        }
    }
    fn top(&self) -> i64 {
        match self {
            Rock::Dash  => 0,
            Rock::Cross => 2,
            Rock::L     => 2,
            Rock::Vbar  => 3,
            Rock::Block => 1,
        }
    }
    fn right(&self) -> i64 {
        match self {
            Rock::Dash  => 3,
            Rock::Cross => 2,
            Rock::L     => 2,
            Rock::Vbar  => 0,
            Rock::Block => 1,
        }
    }
    fn tops(&self) -> &[i64] {
        match self {
            Rock::Dash  => &[0, 0, 0, 0],
            Rock::Cross => &[1, 2, 1],
            Rock::L     => &[0, 0, 2],
            Rock::Vbar  => &[3],
            Rock::Block => &[1, 1],
        }
    }
    fn bottoms(&self) -> &[i64] {
        match self {
            Rock::Dash  => &[0, 0, 0, 0],
            Rock::Cross => &[1, 0, 1],
            Rock::L     => &[0, 0, 0],
            Rock::Vbar  => &[0],
            Rock::Block => &[0, 0],
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
    let mut pieces = [Rock::Dash, Rock::Cross, Rock::L, Rock::Vbar, Rock::Block]
        .iter().cycle();

    let mut max = 0;
    let mut maxes = [0; 7];


    for _ in 0..2022 {
        let mut pos = (2, max + 1);
        let piece = pieces.next().unwrap();

        for _ in 0..3 {
            match pattern.next().unwrap() {
                '<' => if pos.0 > 0 {pos.0 -= 1}
                '>' => if pos.0 + piece.right() < 6 {pos.0 += 1}
                _ => panic!(),
            }
        }
        
        'outer: loop {
            match pattern.next().unwrap() {
                '<' => if pos.0 > 0 {
                    let mut collision = false;
                    for (i, &h) in piece.bottoms().iter().enumerate() {
                        if pos.1 + h <= maxes[i + pos.0 as usize - 1] {
                            collision = true;
                            break;
                        }
                    }
                    if !collision {
                        pos.0 -= 1
                    }
                }
                '>' => if pos.0 + piece.right() < 6 {
                    let mut collision = false;
                    for (i, &h) in piece.bottoms().iter().enumerate() {
                        if pos.1 + h <= maxes[i + pos.0 as usize + 1] {
                            collision = true;
                            break;
                        }
                    }
                    if !collision {
                        pos.0 += 1
                    }
                }
                _ => panic!(),
            }
            pos.1 -= 1;

            for (i, &h) in piece.bottoms().iter().enumerate() {
                if pos.1 + h <= maxes[i + pos.0 as usize] {
                    pos.1 += 1;
                    break 'outer;
                }
            }
        }

        max = max.max(pos.1 + piece.top());
        for (i, &h) in piece.tops().iter().enumerate() {
            maxes[i + pos.0 as usize] = maxes[i + pos.0 as usize].max(pos.1 + h);
        }
    }

    println!("R: {max}");
}


