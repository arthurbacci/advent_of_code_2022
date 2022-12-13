use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::HashSet;
use std::mem;


fn possible_neighbours(matrix: &Vec<Vec<u8>>, me: (usize, usize)) -> Vec<(usize, usize)> {
    let mut candidates = Vec::new();
    if me.1 > 0 {
        candidates.push((me.0, me.1 - 1));
    }
    if me.0 > 0 {
        candidates.push((me.0 - 1, me.1));
    }
    if me.1 + 1 < matrix.len() {
        candidates.push((me.0, me.1 + 1));
    }
    if me.0 + 1 < matrix[0].len() {
        candidates.push((me.0 + 1, me.1));
    }
    let r = candidates.into_iter()
        .filter(|x| matrix[x.1][x.0] + 1 >= matrix[me.1][me.0])
        .collect();
    r
}

fn pathfind(matrix: &Vec<Vec<u8>>, start: (usize, usize), end: (usize, usize)) -> (usize, usize) {
    let mut part2 = usize::MAX;
    let mut visited = HashSet::new();
    let mut to_visit = HashSet::new();
    to_visit.insert(end);
    for i in 0.. {
        if to_visit.is_empty() {break}
        
        let mut old_to_visit = HashSet::new();
        mem::swap(&mut to_visit, &mut old_to_visit);
        for el in old_to_visit {
            if matrix[el.1][el.0] == 0 && part2 == usize::MAX {
                part2 = i;
            }
            if el == start {
                return (i, part2);
            }


            let candidates = possible_neighbours(matrix, el);
            let next_visits: Vec<_> = candidates.into_iter()
                .filter(|x| visited.get(x).is_none())
                .collect();

            for i in next_visits {
                to_visit.insert(i);
            }

            visited.insert(el);
        }

        println!("-----{i}-----");
        for (i, eli) in matrix.iter().enumerate() {
            for (j, elj) in eli.iter().enumerate() {
                let ca = visited.get(&(j, i)).is_some();
                let cb = to_visit.get(&(j, i)).is_some();
                if ca && cb {
                    print!("&");
                } else if ca {
                    print!(".");
                } else if cb {
                    print!("#");
                } else {
                    print!("{}", (elj + b'a') as char);
                }
            }
            println!();
        }
        println!();
    }
    panic!("No paths found")
}

pub fn main() {
    let f = File::open("src/day12.txt").unwrap();
    let reader = BufReader::new(f);
    
    let mut matrix = Vec::new();
    let mut start = (0, 0);
    let mut end = (0, 0);

    for (y, ln) in reader.lines().map(|x| x.unwrap()).enumerate() {
        if ln.is_empty() {break}

        let mut to_add = Vec::new();
        for (x, b) in ln.bytes().enumerate() {
            to_add.push(match b {
                b'S' => {
                    start = (x, y);
                    0
                }
                b'E' => {
                    end = (x, y);
                    25
                }
                c => c - b'a',
            });
        }
        matrix.push(to_add);
    }

    println!("{start:?} {end:?}");


    println!("{:?}", pathfind(&matrix, start, end));
}

