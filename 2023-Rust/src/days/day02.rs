use std::cmp::max;
use regex::Regex;
use lazy_static::lazy_static;

lazy_static! {
    static ref RE_1: Regex = Regex::new(r"Game (\d+): (.+)").unwrap();
    static ref RE_2: Regex = Regex::new(r"(\d+) (\w+)").unwrap();
}

fn get_invalid_sum() -> i32 {
    let mut sum = 0;
    let data: Vec<&str> = include_str!("../../input/day02/input.txt").split("\n").collect();
    for line in data.iter() {
        if line.is_empty() { continue }
        let captures_1 = RE_1.captures(line).unwrap();
        let id = captures_1.get(1).unwrap().as_str().parse::<i32>().unwrap();
        let sets = captures_1.get(2).unwrap().as_str();

        let mut wrong = false;
        for set in sets.split(&[';', ','][..]) {
            let captures_2 = RE_2.captures(set).unwrap();
            let num = captures_2.get(1).unwrap().as_str().parse::<i32>().unwrap();
            let colour = captures_2.get(2).unwrap().as_str();

            if (colour == "red" && num > 12) || (colour == "green" && num > 13) || (colour == "blue" && num > 14) {
                wrong = true;
                break;
            }
        }

        if !wrong { sum += id }
    }
    sum
}

fn get_power_sums() -> i32 {
    let mut sum = 0;
    let data: Vec<&str> = include_str!("../../input/day02/input.txt").lines().collect();
    for line in data {
        let captures_1 = RE_1.captures(line).unwrap();
        let mut min_red = 0; let mut min_green = 0; let mut min_blue = 0;
        for set in captures_1.get(2).unwrap().as_str().split(&[',', ';']) {
            let captures_2 = RE_2.captures(set).unwrap();
            let num = captures_2.get(1).unwrap().as_str().parse::<i32>().unwrap();
            let colour = captures_2.get(2).unwrap().as_str();

            if colour == "red" {
                min_red = max(min_red, num);
            } else if colour == "green" {
                min_green = max(min_green, num);
            } else {
                min_blue = max(min_blue, num);
            }
        }

        sum += min_blue * min_red * min_green;
    }
    sum
}

pub fn solution() {
    println!("Maybe this day will be a bit easier");
    println!("Wrong IDs: {}", get_invalid_sum());
    println!("Power sum: {}", get_power_sums());
}
