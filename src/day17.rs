use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::HashSet;

// I need to store the maximums of each column and remove all the elements
// which are below the minimum of the maximums

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

    fn collides(&self, grid: &HashSet<(usize, usize)>) -> bool {
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
    let mut maxes = [0; 7];
    let mut grid = HashSet::new();

    for _ in 0 as usize..1_000_000 {
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
            if i.1 > maxes[i.0] {
                maxes[i.0] = i.1;
            }
        }
        
        let min_max = maxes.iter().min().unwrap();

        let mut newgrid = HashSet::new();
        for i in grid.drain() {
            if i.1 + 10 >= *min_max {
                newgrid.insert(i);
            }
        }
        grid = newgrid;
    }

    println!("R: {max}");
}


