use regex::Regex;
use lazy_static::lazy_static;

// Optimization to compile Regex only once
lazy_static! {
    static ref RE: Regex = Regex::new(r"(\d+)-(\d+),(\d+)-(\d+)").unwrap();
}

fn get_values(line: &str) -> (i32, i32, i32, i32) {
    let captures = RE.captures(line).unwrap();
    let a = captures.get(1).unwrap().as_str().parse::<i32>().unwrap();
    let b = captures.get(2).unwrap().as_str().parse::<i32>().unwrap();
    let c = captures.get(3).unwrap().as_str().parse::<i32>().unwrap();
    let d = captures.get(4).unwrap().as_str().parse::<i32>().unwrap();
    (a, b, c, d)
}

fn count_overlaps() -> i32 {
    let result: i32 = include_str!("../../input/day04/input.txt").lines()
        .map(|pair| {
            let (a, b, c, d) = get_values(pair);
            if (a <= c && b >= d) || (c <= a && d >= b) { 1 } else { 0 }
        }).sum();
    result
}

fn count_total_overlaps() -> i32 {
    let result: i32 = include_str!("../../input/day04/input.txt").lines()
        .map(|pair| {
            let (a, b, c, d) = get_values(pair);
            // Search the opposite: if they don't overlap, then 0, else 1
            if b < c || a > d { 0 } else { 1 }
        }).sum();
    result
}

pub fn solution() {
    println!("Oof, do not have lunch at work, gonna be tough");
    println!("Overlap count: {}", count_overlaps());
    println!("Part two: {}", count_total_overlaps());
}