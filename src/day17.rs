use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::VecDeque;
use std::ops::{Index, IndexMut};

// This approach didn't seem to work, now I will try to keep a list for each
// column w/ a list of the N last and I will be pop-back-ing it (VecDeque) when
// all the columns have the back element true, so that it will not tend to get
// too big, but it will still be quite fast.
// Also, don't forget to keep track of the number of values popped so that I
// can index the `VecDeque`s

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
    p: (usize, usize),
}

impl Rock {
    fn new(p: (usize, usize), t: RockType) -> Rock {Rock {t, p}}
    fn pieces(&self) -> impl Iterator<Item=(usize, usize)> + '_ {
        match self.t {
            RockType::Dash  => vec![(0, 0), (1, 0), (2, 0), (3, 0)],
            RockType::Cross => vec![(1, 2), (0, 1), (1, 1), (2, 1), (1, 0)],
            RockType::L     => vec![(2, 2), (2, 1), (0, 0), (1, 0), (2, 0)],
            RockType::Vbar  => vec![(0, 3), (0, 2), (0, 1), (0, 0)],
            RockType::Block => vec![(0, 1), (1, 1), (0, 0), (1, 0)],
        }.into_iter()
            .map(|(x, y)| (x + self.p.0, y + self.p.1))
    }
    fn bottom(&self) -> usize {
        self.p.1
    }
    fn top(&self) -> usize {
        self.bottom() + match self.t {
            RockType::Dash  => 0,
            RockType::Cross => 2,
            RockType::L     => 2,
            RockType::Vbar  => 3,
            RockType::Block => 1,
        }
    }
    fn left(&self) -> usize {
        self.p.0
    }
    fn right(&self) -> usize {
        self.left() + match self.t {
            RockType::Dash  => 3,
            RockType::Cross => 2,
            RockType::L     => 2,
            RockType::Vbar  => 0,
            RockType::Block => 1,
        }
    }

    fn collides(&self, grid: &Grid) -> bool {
        for (x, y) in self.pieces() {
            if grid[y][x] {
                return true;
            }
        }
        false
    }
}

struct Grid {
    v: VecDeque<[bool; 7]>,
    o: usize,
}

impl Grid {
    fn trim(&mut self) {
        while !self.v.is_empty() && self.v[0] == [true; 7] {
            self.v.pop_front();
        }
    }
}

impl Index<usize> for Grid {
    type Output = [bool; 7];
    fn index(&self, i: usize) -> &Self::Output {
        if i < self.o {
            &[true; 7]
        } else if i < self.v.len() + self.o {
            &self.v[i - self.o]
        } else {
            &[false; 7]
        }
    }
}
impl IndexMut<usize> for Grid {
    fn index_mut(&mut self, i: usize) -> &mut Self::Output {
        if i < self.o {
            panic!("Attemped to change the past, action universally acknowledged to be impossible")
        } else if i < self.v.len() + self.o {
            &mut self.v[i - self.o]
        } else {
            while i >= self.v.len() + self.o {
                self.v.push_back([false; 7]);
            }
            &mut self.v[i - self.o]
        }
    }
}

impl Grid {
    fn new() -> Grid {
        Grid {
            v: VecDeque::new(),
            o: 0,
        }
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

    let mut grid = Grid::new();

    for loop_count in 0 as usize..10_000_000 {
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
        for (x, y) in rock.pieces() {
            grid[y][x] = true;
        }
        
        if loop_count % 1000 == 0 {
            grid.trim();
        }
    }

    println!("R: {max}");
}


