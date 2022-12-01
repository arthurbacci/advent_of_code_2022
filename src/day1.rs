use std::io::{BufRead, BufReader};
use std::fs::File;
use std::iter;

fn part1(reader: impl BufRead) -> u64 {
    let mut total: u64 = 0;
    let mut greatest_total: u64 = 0;

    for ln in reader.lines().chain(iter::once(Ok("".to_string()))) {
        let ln = ln.unwrap();

        if !ln.is_empty() {
            total += ln.parse::<u64>().unwrap();
        } else {
            if total > greatest_total {
                greatest_total = total;
            }
            total = 0;
        }
    }

    greatest_total
}

fn part2(reader: impl BufRead) -> u64 {
    let mut total: u64 = 0;
    let mut greatest_totals: (u64, u64, u64) = (0, 0, 0);

    for ln in reader.lines().chain(iter::once(Ok("".to_string()))) {
        let ln = ln.unwrap();

        if !ln.is_empty() {
            total += ln.parse::<u64>().unwrap();
        } else {
            if total > greatest_totals.0 {
                greatest_totals.2 = greatest_totals.1;
                greatest_totals.1 = greatest_totals.0;
                greatest_totals.0 = total;
            } else if total > greatest_totals.1 {
                greatest_totals.2 = greatest_totals.1;
                greatest_totals.1 = total;
            } else if total > greatest_totals.2 {
                greatest_totals.2 = total;
            }
            total = 0;
        }
    }

    greatest_totals.0 + greatest_totals.1 + greatest_totals.2
}

pub fn main() {
    let f = File::open("src/day1.txt").unwrap();
    let reader = BufReader::new(f);

    let greatest_total = part1(reader);
    println!("Greatest total: {greatest_total}");


    let f = File::open("src/day1.txt").unwrap();
    let reader = BufReader::new(f);

    let greatest_totals_sum = part2(reader);
    println!("Greatest totals sum: {greatest_totals_sum}");

}


