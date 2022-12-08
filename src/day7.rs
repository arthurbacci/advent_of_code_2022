use std::io::{BufRead, BufReader};
use std::fs::File;
use std::collections::HashMap;

fn add_sz(size_map: &mut HashMap<Vec<String>, u64>, key: &[String], add: u64) {
    if let Some(x) = size_map.get_mut(key) {
        *x += add;
    } else {
        size_map.insert(key.to_vec(), add);
    }
}

pub fn main() {
    let f = File::open("src/day7.txt").unwrap();
    let reader = BufReader::new(f);

    let lines = reader.lines().map(|x| x.unwrap());
    let mut path = vec!["/".to_string()];
    let mut size_map = HashMap::new();

    for ln in lines {
        let ln: Vec<_> = ln.split(' ').collect();
        if ln.is_empty() {continue}

        
        if ln[0] == "$" {
            if ln[1] == "cd" {
                match ln[2] {
                    ".." => {path.pop();}
                    "/" => path.truncate(1),
                    s => path.push(s.to_string()),
                }
            }
        } else {
            if let Ok(x) = ln[0].parse::<u64>() {
                for i in 1..=path.len() {
                    add_sz(&mut size_map, &path[0..i], x);
                }
            }
        }
    }

    let need_to_free = size_map.get(&vec!["/".to_string()]).unwrap() - 40000000;

    let mut total = 0;
    let mut less_enough = u64::MAX;
    for (_, v) in size_map {
        if v >= need_to_free && v < less_enough {
            less_enough = v;
        }

        if v <= 100000 {
            total += v;
        }
    }
    println!("{total} {less_enough}");
}

