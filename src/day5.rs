use std::io::{BufRead, BufReader};
use std::fs::File;

fn get_element(ln: &str, index: &mut usize) -> Option<char> {
    match ln.chars().nth(*index).unwrap() {
        '[' => {
            if ln.len() - *index < 2 {
                None
            } else {
                *index += 4;
                Some(ln.chars().nth(*index - 3).unwrap())
            }
        }
        ' ' => {
            if ln.len() - *index < 2 {
                None
            } else {
                if ln.chars().nth(*index + 1).unwrap() != ' ' {
                    return None
                }
                
                *index += 4;
                Some(' ')
            }
        }
        _ => {
            None
        }
    }
}

fn get_row(ln: String) -> Option<Vec<char>> {
    let mut ret = Vec::new();
    let mut index = 0;

    loop {
        if index >= ln.len() {break}

        let el = get_element(&ln, &mut index)?;
        ret.push(el);
    }
    
    Some(ret)
}

fn both_parts(part2: bool) {
    let f = File::open("src/day5.txt").unwrap();
    let reader = BufReader::new(f);
    let mut lines = reader.lines();

    let mut stacks = Vec::new();

    loop {
        let ln = match lines.next() {
            Some(x) => x,
            _ => break,
        }.unwrap();
        if ln.is_empty() {break}

        println!("{ln}");
        
        match get_row(ln) {
            Some(sts) => {
                for (i, el) in sts.into_iter().enumerate() {
                    if stacks.len() <= i {
                        stacks.push(Vec::new());
                    }

                    if el == ' ' {continue}

                    stacks[i].push(el);
                }
            }
            None => break,
        }
    }

    let mut newstacks = Vec::new();
    for mut i in stacks {
        let mut s = Vec::new();

        while !i.is_empty() {
            s.push(i.pop().unwrap())
        }

        newstacks.push(s);
    }
    let mut stacks = newstacks;


    lines.next().unwrap().unwrap();
    for ln in lines {
        let ln = ln.unwrap();
        if ln.is_empty() {break}

        let s: Vec<usize> = ln.split(' ')
            .skip(1)
            .step_by(2)
            .map(|x| x.parse().unwrap())
            .collect();

        if !part2 {
            for _ in 0..s[0] {
                let p = stacks[s[1] - 1].pop().unwrap();
                stacks[s[2] - 1].push(p);
            }
        } else {
            let i = stacks[s[1] - 1].len() - s[0];
            for _ in 0..s[0] {
                let r = stacks[s[1] - 1].remove(i);
                stacks[s[2] - 1].push(r);
            }
        }
    }

    for i in &stacks {
        print!("{}", i[i.len() - 1]);
    }
    println!();
}

pub fn main() {
    both_parts(false);
    both_parts(true);
}


