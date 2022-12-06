use std::fs;

fn all_different<T: Eq>(arr: &[T]) -> bool {
    for i in 0..(arr.len() - 1) {
        for j in (i + 1)..arr.len() {
            if arr[i] == arr[j] {
                return false;
            }
        }
    }
    true
}

pub fn both_parts(s: &str, len: usize) -> usize {
    let mut last = vec!['\0'; len];
    for (i, el) in s.chars().enumerate() {
        for i in 0..(last.len() - 1) {
            last[i] = last[i + 1];
        }
        last[len - 1] = el;
        if i >= 3 && all_different(&last) {
            return i + 1;
        }
    }
    panic!("Didn't got any value")
}

pub fn main() {
    let s = fs::read_to_string("src/day6.txt").unwrap();
    
    let part1 = both_parts(&s, 4);
    let part2 = both_parts(&s, 14);

    println!("{part1} {part2}");
}


