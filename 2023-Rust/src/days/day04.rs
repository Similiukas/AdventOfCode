use regex::Regex;
use lazy_static::lazy_static;

lazy_static! {
    static ref RE: Regex = Regex::new(r"Card\s+\d+?:\s+(.+) \| (.+)").unwrap();
}

fn get_winners() -> i32 {
    let mut sum = 0;
    let data: Vec<&str> = include_str!("../../input/day04/input.txt").lines().collect();
    for line in data {
        let captures = RE.captures(line).unwrap();
        let winners = captures.get(1).unwrap().as_str().split(" ").filter(|a| !a.is_empty()).collect::<Vec<&str>>();
        let numbers = captures.get(2).unwrap().as_str().split(" ").filter(|a| !a.is_empty()).collect::<Vec<&str>>();

        let mut prize = 1;
        for num in numbers {
            if winners.contains(&num) {
                prize *= 2;
            }
        }
        sum += prize / 2;
    }
    sum
}

fn get_recursive() -> i32 {
    let data: Vec<&str> = include_str!("../../input/day04/input.txt").lines().collect();
    let mut multiples: Vec<i32> = vec![1; data.len()];
    for (i, line) in data.iter().enumerate() {
        let captures = RE.captures(line).unwrap();
        let winners = captures.get(1).unwrap().as_str().split(" ").filter(|a| !a.is_empty()).collect::<Vec<&str>>();
        let numbers = captures.get(2).unwrap().as_str().split(" ").filter(|a| !a.is_empty()).collect::<Vec<&str>>();

        let num_of_winners: usize = numbers.iter().filter(|a| winners.contains(a)).collect::<Vec<&&str>>().len();

        let mul = multiples[i];
        for j in (i+1)..=(i+num_of_winners) {
            multiples[j] += 1 * mul;
        }
    }
    multiples.iter().sum()
}


pub fn solution() {
    println!("This fucking fuming day let's do coding shit");
    println!("Get those fuckers {}", get_winners());
    println!("Get these more interesting {}", get_recursive());
}