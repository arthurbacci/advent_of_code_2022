use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(PartialEq, Eq, Clone, Copy)]
enum Block {
    Air,
    Lava,
    Water,
}

fn propagate_water(b: &mut [[[Block; 20]; 20]; 20], mut p: [usize; 3]) {
    if b[p[0]][p[1]][p[2]] != Block::Air {
        return;
    }
    b[p[0]][p[1]][p[2]] = Block::Water;
    for i in 0..3 {
        if p[i] < 19 {
            p[i] += 1;
            propagate_water(b, p);
            p[i] -= 1;
        }
        if p[i] > 0 {
            p[i] -= 1;
            propagate_water(b, p);
            p[i] += 1;
        }
    }
}

pub fn main() {
    let f = File::open("src/day18.txt").unwrap();
    let reader = BufReader::new(f);

    let mut grid = [[[Block::Air; 20]; 20]; 20];

    for ln in reader.lines().map(|x| x.unwrap()) {
        if ln.is_empty() {break}

        let mut split = ln.split(',');
        let a: usize = split.next().unwrap().parse().unwrap();
        let b: usize = split.next().unwrap().parse().unwrap();
        let c: usize = split.next().unwrap().parse().unwrap();

        grid[a][b][c] = Block::Lava;
    }

    propagate_water(&mut grid, [19, 19, 19]);
    
    let mut total = 0;
    let mut total2 = 0;
    for x in 0..20 {
        for y in 0..20 {
            for z in 0..20 {
                if grid[x][y][z] != Block::Lava {continue}
                let mut area = 0;
                let mut area2 = 0;
                
                let mut coords = [x, y, z];
                for i in 0..3 {
                    if coords[i] < 19 {
                        coords[i] += 1;
                        if grid[coords[0]][coords[1]][coords[2]] != Block::Lava {
                            if grid[coords[0]][coords[1]][coords[2]] == Block::Water {
                                area2 += 1;
                            }
                            area += 1;
                        }
                        coords[i] -= 1;
                    } else {
                        area += 1;
                        area2 += 1;
                    }
                    if coords[i] > 0 {
                        coords[i] -= 1;
                        if grid[coords[0]][coords[1]][coords[2]] != Block::Lava {
                            if grid[coords[0]][coords[1]][coords[2]] == Block::Water {
                                area2 += 1;
                            }
                            area += 1;
                        }
                        coords[i] += 1;
                    } else {
                        area += 1;
                        area2 += 1;
                    }
                }

                total += area;
                total2 += area2;
            }
        }
    }
    println!("{total} {total2}");
}


