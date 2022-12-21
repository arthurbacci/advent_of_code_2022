use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::{HashMap, HashSet};
use std::iter;

/*
 * To solve day 16, I must make sure that I never open any of the pipes more
 * than 1 time
 */

fn consume<'a>(to_consume: &str, to_be_consumed: &'a str) -> Option<((), &'a str)> {
    to_be_consumed.strip_prefix(to_consume).map(|x| ((), x))
}

fn consume_char(to_be_consumed: &str) -> Option<(char, &str)> {
    let mut i = to_be_consumed.char_indices();
    let (_, c) = i.next()?;
    let index = i.next()?.0;
    Some((c, &to_be_consumed[index..]))
}

fn consume_num(to_be_consumed: &str) -> Option<(u64, &str)> {
    let mut index = to_be_consumed.len();
    for (i, c) in to_be_consumed.char_indices() {
        if !c.is_numeric() {
            index = i;
            break;
        }
    }
    if index == 0 {
        None
    } else {
        Some((
            to_be_consumed[0..index].parse().unwrap(),
            &to_be_consumed[index..],
        ))
    }
}

type Id = [char; 2];

#[derive(Debug, Clone)]
struct Score {
    terms: HashMap<Id, u64>,
}

impl Score {
    fn new(i: impl IntoIterator<Item=(Id, u64)>) -> Self {
        Self {
            terms: HashMap::from_iter(i),
        }
    }
}

#[derive(Debug)]
struct Choice {
    scores: Vec<Score>,
}

impl Choice {
    fn new(scores: Vec<Score>) -> Self {
        Self {scores}
    }
}

pub fn main() {
    let f = File::open("src/day16.txt").unwrap();
    let reader = BufReader::new(f);

    let mut valves = HashMap::new();

    for ln in reader.lines().map(|x| x.unwrap()) {
        if ln.is_empty() {break}

        let (_, ln) = consume("Valve ", &ln).unwrap();
        let (c1, ln) = consume_char(&ln).unwrap();
        let (c2, ln) = consume_char(&ln).unwrap();
        let valve: Id = [c1, c2];
        let (_, ln) = consume(" has flow rate=", &ln).unwrap();
        let (rate, ln) = consume_num(&ln).unwrap();
        let (_, ln) = consume("; tunnel", &ln).unwrap();
        let (_, ln) = consume("s", &ln).unwrap_or(((), ln));
        let (_, ln) = consume(" lead", &ln).unwrap();
        let (_, ln) = consume("s", &ln).unwrap_or(((), ln));
        let (_, ln) = consume(" to valve", &ln).unwrap();
        let (_, ln) = consume("s", &ln).unwrap_or(((), ln));
        let (_, ln) = consume(" ", &ln).unwrap();

        let leads: Vec<_> = ln.split(", ")
            .map(|x| if x.len() == 2 {
                let mut chars = x.chars();
                [chars.next().unwrap(), chars.next().unwrap()]
            } else {
                panic!("Wrong length for leads")
            }).collect();

        valves.insert(valve, (rate, leads));
    }
    
    let mut choices0 = HashMap::new();
    for (&k, _) in &valves {
        choices0.insert(
            k,
            vec![Choice::new(vec![Score::new(iter::once((k, 1)))])]
        );
    }

    // I need to keep two of the last choices, otherwise I won't be able to
    // calculate the case where they valve is open


    let mut choices1 = HashMap::new();
    for (&k, (_, leads)) in &valves {
        let mut choices = Vec::new();
        for i in leads.iter().cloned().chain(iter::once(k)) {
            choices.push(Score::new([(i, 1)]));
        }
        choices1.insert(k, Choice::new(choices));
    }

    for _ in 4..=4 {
        let mut newchoices = HashMap::new();
        

        for (&k, (_, leads)) in &valves {
            let mut choices = Vec::new();
            for i in leads {
                for j in &choices1.get(i).unwrap().scores {
                    choices.push(j.clone());
                }
            }
            // TODO: when opening too
            newchoices.insert(k, Choice::new(choices));
        }

        println!("{newchoices:#?}");
    }
}

