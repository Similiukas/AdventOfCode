use regex::{Match, Regex};
use lazy_static::lazy_static;
use std::collections::HashMap;

lazy_static! {
    static ref RE: Regex = Regex::new(r"1|2|3|4|5|6|7|8|9|one|two|three|four|five|six|seven|eight|nine|eno|owt|eerht|ruof|evif|xis|neves|thgie|enin").unwrap();
}

fn get_digits() -> u32 {
    let mut sum = 0;
    let data: Vec<&str> = include_str!("../../input/day01/input.txt").split("\n").collect();
    for line in data.iter() {
        if line.is_empty() { continue; }
        let mut first: u32 = 99; let mut last: u32 = 99;
        for char in line.chars() {
            if char.is_digit(10) {
                if first == 99 {
                    first = char.to_digit(10).unwrap();
                } else {
                    last = char.to_digit(10).unwrap();
                }
            }
        }
        sum += if last > 10 { first * 10 + first } else { first * 10 + last };
    }
    sum
}

fn get_fancy_digits() -> u32 {
    let mut sum = 0;
    let hm = HashMap::from([
        ("1", 1), ("one", 1), ("eno", 1),
        ("2", 2), ("two", 2), ("owt", 2),
        ("3", 3), ("three", 3), ("eerht", 3),
        ("4", 4), ("four", 4), ("ruof", 4),
        ("5", 5), ("five", 5), ("evif", 5),
        ("6", 6), ("six", 6), ("xis", 6),
        ("7", 7), ("seven", 7), ("neves", 7),
        ("8", 8), ("eight", 8), ("thgie", 8),
        ("9", 9), ("nine", 9), ("enin", 9)
    ]);
    let data: Vec<&str> = include_str!("../../input/day01/input.txt").split("\n").collect();
    for line in data.iter() {
        if line.is_empty() { continue; }

        let captures_l: Vec<Match> = RE.find_iter(line).collect();
        // Reversing the line and checking from right to left to get the last digit.
        // For that need to HashMap and RegEx to have reversed words
        let reversed = line.chars().rev().collect::<String>();
        let captures_r: Vec<Match> = RE.find_iter(&*reversed).collect();

        sum +=  hm[captures_l[0].as_str()] * 10 + hm[captures_r[0].as_str()];
    }
    sum
}

pub fn solution() {
    println!("Well here we go again. For the last time in the UK. A bit melancholic, ngl");
    println!("The sum is simple: {}", get_digits());
    println!("Bro, wtf is this day 1 {}", get_fancy_digits());
}
