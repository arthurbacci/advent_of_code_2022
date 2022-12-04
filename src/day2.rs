use std::io::{BufRead, BufReader};
use std::fs::File;
use std::iter;
use std::str::FromStr;

#[derive(Debug, PartialEq, Clone, Copy)]
enum Shape {
    Rock = 1,
    Paper = 2,
    Scissors = 3,
}

#[derive(Debug, PartialEq, Clone, Copy)]
enum Outcome {
    Win = 6,
    Draw = 3,
    Lost = 0,
}

impl Shape {
    fn versus(&self, other: &Shape) -> Outcome {
        match (self, other) {
            (  Shape::Rock    , Shape::Scissors)
            | (Shape::Paper   , Shape::Rock    )
            | (Shape::Scissors, Shape::Paper   ) => Outcome::Win,
            (a, b) if a == b => Outcome::Draw,
            _ => Outcome::Lost,
        }
    }

    fn loses_to(&self) -> Self {
        match self {
            Shape::Rock => Shape::Paper,
            Shape::Paper => Shape::Scissors,
            Shape::Scissors => Shape::Rock,
        }
    }

    fn wins_from(&self) -> Self {
        match self {
            Shape::Rock => Shape::Scissors,
            Shape::Paper => Shape::Rock,
            Shape::Scissors => Shape::Paper,
        }
    }
}

impl FromStr for Shape {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() != 1 {
            return Err(());
        }
        match &s[0..=0] {
            "A" | "X" => Ok(Shape::Rock),
            "B" | "Y" => Ok(Shape::Paper),
            "C" | "Z" => Ok(Shape::Scissors),
            _ => Err(()),
        }
    }
}


pub fn main() {
    let f = File::open("src/day2.txt").unwrap();
    let reader = BufReader::new(f);

    let mut total: u32 = 0;
    let mut total2: u32 = 0;

    for ln in reader.lines().chain(iter::once(Ok("".to_string()))) {
        let ln = ln.unwrap();
        if ln.is_empty() {break}
        
        /* PART 1 */
        let he: Shape = ln[0..=0].parse().unwrap();
        let me: Shape = ln[2..=2].parse().unwrap();

        total += me as u32 + me.versus(&he) as u32;
        

        /* PART 2 */
        let me = match me {
            // Needs to lose
            Shape::Rock => he.wins_from(),
            // Needs to draw
            Shape::Paper => he,
            // Needs to win
            Shape::Scissors => he.loses_to(),
        };

        total2 += me as u32 + me.versus(&he) as u32;
    }
    
    println!("{total}");
    println!("{total2}");
}


