use std::io::{BufRead, BufReader};
use std::fs::File;


pub fn main() {
    let f = File::open("src/day8.txt").unwrap();
    let reader = BufReader::new(f);

    let mut trees = Vec::new();

    let lines = reader.lines().map(|x| x.unwrap());
    for ln in lines {
        if ln.is_empty() {return}

        let mut to_add = Vec::new();
        for i in ln.bytes() {
            to_add.push(i - b'0');
        }
        trees.push(to_add);
    }


    let mut from_right = trees.clone();
    for y in 0..from_right.len() {
        let mut max = 0;
        for x in (0..from_right[y].len()).rev() {
            if from_right[y][x] > max {
                max = from_right[y][x];
            } else {
                from_right[y][x] = max;
            }
        }
    }

    let mut from_down = trees.clone();
    for x in 0..from_down[0].len() {
        let mut max = 0;
        for y in (0..from_down.len()).rev() {
            if from_down[y][x] > max {
                max = from_down[y][x];
            } else {
                from_down[y][x] = max;
            }
        }
    }
    

    let mut total = 0;
    total += 2 * trees.len() + 2 * trees[0].len() - 4;

    let mut max_from_up = trees[0].clone();
    for row in 1..(trees.len() - 1) {
        let mut max_from_left = trees[row][0];
        for col in 1..(trees[row].len() - 1) {
            let cur = trees[row][col];
    
            if cur > max_from_up[col] || cur > max_from_left
                || cur > from_down[row + 1][col]
                || cur > from_right[row][col + 1]
            {
                total += 1;
            }

            if cur > max_from_up[col] {
                max_from_up[col] = cur;
            }
            if cur > max_from_left {
                max_from_left = cur;
            }
        }
    }

    println!("{total}");
}

