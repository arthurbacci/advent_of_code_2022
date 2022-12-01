use std::io::{BufRead, BufReader};
use std::fs::File;
use std::iter;

pub fn main() {
    let f = File::open("src/day1.txt").unwrap();
    let reader = BufReader::new(f);

    let mut total: u64 = 0;
    let mut greatest_totals: (u64, u64, u64) = (0, 0, 0);

    for ln in reader.lines().chain(iter::once(Ok("".to_string()))) {
        let ln = ln.unwrap();

        if !ln.is_empty() {
            total += ln.parse::<u64>().unwrap();
        } else {
            if total > greatest_totals.0 {
                greatest_totals = (total, greatest_totals.0, greatest_totals.1);
            } else if total > greatest_totals.1 {
                greatest_totals = (greatest_totals.0, total, greatest_totals.1);
            } else if total > greatest_totals.2 {
                greatest_totals.2 = total;
            }
            total = 0;
        }
    }

    println!("Greatest total: {}", greatest_totals.0);
    
    println!(
        "Greatest totals sum: {}",
        greatest_totals.0 + greatest_totals.1 + greatest_totals.2
    );

}


