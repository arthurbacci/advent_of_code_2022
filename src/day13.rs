use std::fs::File;
use std::io::{BufRead, BufReader};
use std::fmt;
use std::cmp::Ordering;

#[derive(Clone)]
enum Packet {
    Number(u64),
    List(Vec<Box<Packet>>),
}

impl fmt::Debug for Packet {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Packet::Number(n) => write!(f, "{:?}", n),
            Packet::List(v) => {
                f.debug_list().entries(v.iter()).finish()
            }
        }
    }
}

impl fmt::Display for Packet {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Packet::Number(n) => write!(f, "{}", n),
            Packet::List(v) => {
                write!(f, "[")?;
                if let Some(x) = v.iter().next() {
                    write!(f, "{}", x)?;
                }
                for i in v.iter().skip(1) {
                    write!(f, ",{}", i)?;
                }
                write!(f, "]")
            }
        }
    }
}

impl Packet {
    fn parse(s: &str) -> Self {
        let mut els_stack = vec![vec![]];
        
        let mut chars = s.chars().peekable();
        while let Some(c) = chars.next() {
            match c {
                '[' => els_stack.push(vec![]),
                ']' => {
                    let to_add = els_stack.pop().unwrap();
                    els_stack.last_mut().unwrap().push(
                        Box::new(Packet::List(to_add))
                    );
                }
                ',' => {}
                c => {
                    let mut number_t = if c.is_ascii_digit() {
                        let c: u8 = c.try_into().unwrap();
                        (c - b'0') as u64
                    } else {
                        panic!("Not an ascii number")
                    };
                    while let Some(c) = chars.peek() {
                        if !c.is_ascii_digit() {break}
                        let c: u8 = chars.next().unwrap().try_into().unwrap();
                        number_t *= 10;
                        number_t += (c - b'0') as u64;
                    }
                    els_stack.last_mut().unwrap().push(
                        Box::new(Packet::Number(number_t))
                    );
                }
            }
        }

        if els_stack.len() == 1 {
            *els_stack.remove(0).remove(0)
        } else {
            panic!("More than one element in the parsed string");
        }
    }

    fn compare(&self, other: &Self) -> Ordering {
        match (self, other) {
            (Packet::Number(a), Packet::Number(b)) => a.cmp(b),
            (Packet::List(a), Packet::List(b)) => {
                for i in 0..(a.len().min(b.len())) {
                    let c = a[i].compare(&b[i]);
                    if c != Ordering::Equal {
                        return c;
                    }
                }
                
                if a.len() == b.len() {
                    Ordering::Equal
                } else if a.len() > b.len() {
                    Ordering::Greater
                } else {
                    Ordering::Less
                }
            }
            (Packet::List(_), Packet::Number(_)) => {
                self.compare(&Packet::List(vec![Box::new(other.clone())]))
            }
            (Packet::Number(_), Packet::List(_)) => {
                Packet::List(vec![Box::new(self.clone())]).compare(&other)
            }
        }
    }
}

pub fn main() {
    let f = File::open("src/day13.txt").unwrap();
    let reader = BufReader::new(f);
    let mut lines = reader.lines().map(|x| x.unwrap());

    let mut everything = Vec::new();

    let mut total = 0;
    for i in 1.. {
        let fst = if let Some(fst) = lines.next() {fst} else {break};
        let snd = lines.next().unwrap();

        let (fst, snd) = (Packet::parse(&fst), Packet::parse(&snd));

        if fst.compare(&snd) == Ordering::Less {
            total += i;
        }

        everything.push(fst);
        everything.push(snd);

        if lines.next().is_none() {break};
    }

    println!("{total}");

    
    let mut before_len = 0;
    let mut middle_len = 0;

    let pivot1 = Packet::parse("[[2]]");
    let pivot2 = Packet::parse("[[6]]");

    for i in everything {
        match i.compare(&pivot1) {
            Ordering::Less => before_len += 1,
            Ordering::Greater => match i.compare(&pivot2) {
                Ordering::Less => middle_len += 1,
                Ordering::Greater => {}
                _ => panic!("It may not work"),
            }
            _ => panic!("It may not work"),
        }
    }

    println!("{}", (before_len + 1) * (before_len + middle_len + 2));
}

