use std::iter::Iterator;
use std::fs::File;
use std::io::{self, BufRead};
// use std::iter::FromIterator;

// fn sort_string_by_char(input: &str) -> String {
//     let mut arr: Vec<char> = input.chars().collect();
//     arr.sort_by(|a, b| a.cmp(b));
//     return String::from_iter(arr);
// }

fn find_item_priorities() -> u32 {
    let result: u32 = include_str!("../../input/day03/input.txt")
        .split('\n')
        .map(|line| {
            // Split the line to two rucksacks
            let (a, b) = line.trim().split_at(line.len() / 2);
            // Go through first one
            a.chars().map(|item| {
                // If second one contains an item from first one
                if b.contains(item){
                    if item as u32 > 96 { item as u32 - 96 } else { item as u32 - 38 }
                } else { 0 }
            // Only take the first one above 0
            }).filter(|x| x > &0).next().unwrap_or(0)
        }).sum();
    return result;
}

fn find_badges() -> u32 {
    let mut result: u32 = 0; let mut line_index: u32 = 0;
    let mut line1 = "".to_string(); let mut line2 = "".to_string();

    let file = File::open("input/day03/input.txt").unwrap();
    let data = io::BufReader::new(file).lines();

    for line_wrap in data {
        let line = line_wrap.unwrap_or("".to_string());
        if line_index == 0 {
            line1 = line.clone();
            line_index += 1;
        } else if line_index == 1 {
            line2 = line.clone();
            line_index += 1;
        } else if line_index == 2 {
            line_index = 0;
            for item in line1.chars() {
                if line2.contains(item) {
                    if line.chars().any(|x| x == item) {
                        result += if item as u32 > 96 { item as u32 - 96 } else { item as u32 - 38 };
                        println!("Found in all lines {} {} {} {} {}", item, if item as u32 > 96 { item as u32 - 96 } else { item as u32 - 38 }, line1, line2, line);
                    }
                    // break;
                }
            }
        }

    }
    return result;
}

pub fn solution() {
    println!("Another day, another work solver");
    // println!("Result: {}", find_item_priorities());
    println!("Second one: {}", find_badges());
}