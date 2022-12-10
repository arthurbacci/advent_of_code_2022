use std::fs;

pub fn main() {
    let width = 40;
    let height = 6;
    let sz = width * height;

    let s = fs::read_to_string("src/day10-spinoff.txt").unwrap();
    let mut chars = s.chars();
    if (chars.next().unwrap(), chars.next().unwrap()) != ('#', '#') {
        println!("WARNING: the two first characters will be changed");
    }

    // TODO: fix for odd sz
    let mut x_reg = 1;
    for i in (2..sz).step_by(2) {
        let a = chars.next().unwrap();
        let b = chars.next().unwrap();
        
        let x = i % width + match (a, b) {
            ('#', '#') => 0,
            ('#', '.') => -1,
            ('.', '#') => 2,
            ('.', '.') => -2,
            d => panic!("Invalid data {d:?}"),
        };

        println!("addx {}", x - x_reg);
        x_reg = x
    }
    println!("noop");
    println!("noop");
}
