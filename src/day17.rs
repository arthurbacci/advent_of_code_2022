use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::HashSet;

// I need to store the maximums of each column and remove all the elements
// which are below the minimum of the maximums

#[derive(Debug, Clone, Copy)]
enum RockType {
    Dash,
    Cross,
    L,
    Vbar,
    Block,
}

#[derive(Debug, Clone)]
struct Rock {
    t: RockType,
    p: (u64, u64),
}

impl Rock {
    fn new(p: (u64, u64), t: RockType) -> Rock {Rock {t, p}}
    fn pieces(&self) -> impl Iterator<Item=(u64, u64)> + '_ {
        match self.t {
            RockType::Dash  => vec![(0, 0), (1, 0), (2, 0), (3, 0)],
            RockType::Cross => vec![(1, 2), (0, 1), (1, 1), (2, 1), (1, 0)],
            RockType::L     => vec![(2, 2), (2, 1), (0, 0), (1, 0), (2, 0)],
            RockType::Vbar  => vec![(0, 3), (0, 2), (0, 1), (0, 0)],
            RockType::Block => vec![(0, 1), (1, 1), (0, 0), (1, 0)],
        }.into_iter()
            .map(|(x, y)| (x + self.p.0, y + self.p.1))
    }
    fn bottom(&self) -> u64 {
        self.p.1
    }
    fn top(&self) -> u64 {
        self.bottom() + match self.t {
            RockType::Dash  => 0,
            RockType::Cross => 2,
            RockType::L     => 2,
            RockType::Vbar  => 3,
            RockType::Block => 1,
        }
    }
    fn left(&self) -> u64 {
        self.p.0
    }
    fn right(&self) -> u64 {
        self.left() + match self.t {
            RockType::Dash  => 3,
            RockType::Cross => 2,
            RockType::L     => 2,
            RockType::Vbar  => 0,
            RockType::Block => 1,
        }
    }

    fn collides(&self, grid: &HashSet<(u64, u64)>) -> bool {
        for i in self.pieces() {
            if grid.get(&i).is_some() {
                return true;
            }
        }
        false
    }
}


pub fn main() {
    let f = File::open("src/day17.txt").unwrap();
    let reader = BufReader::new(f);
    let line = reader.lines().next().unwrap().unwrap();

    let mut rocks = [
        RockType::Dash,
        RockType::Cross,
        RockType::L,
        RockType::Vbar,
        RockType::Block,
    ].iter().cycle();
    let mut pattern = line
        .chars()
        .map(|x| match x {
            '<' | '>' => x,
            _ => panic!("Invalid direction {x}"),
        }).cycle();

    let mut max = 0;
    let mut grid = HashSet::new();

    for _ in 0 as usize..2022 {
        let mut rock = Rock::new((2, max + 1), *rocks.next().unwrap());

        for _ in 0..3 {
            match pattern.next().unwrap() {
                '<' => if rock.left() > 0 {rock.p.0 -= 1}
                '>' => if rock.right() < 6 {rock.p.0 += 1}
                _ => panic!(),
            }
        }
        
        'outer: loop {
            match pattern.next().unwrap() {
                '<' => if rock.left() > 0 {
                    rock.p.0 -= 1;
                    if rock.collides(&grid) {
                        rock.p.0 += 1;
                    }
                }
                '>' => if rock.right() < 6 {
                    rock.p.0 += 1;
                    if rock.collides(&grid) {
                        rock.p.0 -= 1;
                    }
                }
                _ => panic!(),
            }
            rock.p.1 -= 1;

            if rock.p.1 == 0 || rock.collides(&grid) {
                rock.p.1 += 1;
                break 'outer;
            }
        }

        max = max.max(rock.top());
        for i in rock.pieces() {
            grid.insert(i);
        }
    }

    println!("R: {max}");
}


