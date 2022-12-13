use regex::Regex;
use lazy_static::lazy_static;

// Optimization to compile Regex only once
lazy_static! {
    static ref RE: Regex = Regex::new(r"move (\d+) from (\d+) to (\d+)").unwrap();
}

fn format_stacks(input: &str) -> Vec<Vec<char>> {
    // let data = input.lines();
    // let mut result: Vec<Vec<char>> = vec![vec![]];
    // for line in data {
    //     for (i, input) in (line[1..]).chars().enumerate().step_by(4) {
    //         if input == ' ' { continue; }
    //         println!("{i} [{input}]");
    //         let mut r: &mut Vec<char> = &mut result[0]; // DOES NOT WORK HERE as it's getting inside a for loop
    //         r.push(input);
    //     }
    //     println!("After the line");
    //     for i in result {
    //         println!("Result: {:#?}", i.iter().collect::<String>());
    //     }
    // }

    // Read data horizontally
    let a: Vec<Vec<char>> = input.lines()
        .map(|line| {
            // Read characters by skipping first one and stepping every 4th one
            (line[1..]).chars().step_by(4).collect::<Vec<char>>()
            // line
        }).collect();

    // Transpose it
    // Go through array rows to fill it in columns
    let b: Vec<Vec<char>> = (0..a[0].len()).map(|i| {
            // Skip the last row as it's just the numbers
            a[0..a.len() - 1].iter().map(|inner| {
                inner[i].clone()
            // Filtering non-empty values and reversing it from FIFO to LIFO
            }).filter(|i| *i != ' ').rev().collect::<Vec<char>>()
        })
        .collect();
    b
}

fn move_crates(mut stacks: Vec<Vec<char>>, rest: &str) -> String {
    for line in rest.lines() {
        let captures = RE.captures(line).unwrap();
        let from = captures.get(2).unwrap().as_str().parse::<usize>().unwrap_or(0) - 1;
        let to = captures.get(3).unwrap().as_str().parse::<usize>().unwrap_or(0) - 1;
        // Moving one crate at a time
        for _ in 0..captures.get(1).unwrap().as_str().parse::<i32>().unwrap_or(0) {
            let temp = stacks[from].pop().unwrap();
            stacks[to].push(temp);
        }
    }

    stacks.iter().map(|line| line.last().unwrap_or(&' ')).collect::<String>()
}

fn move_multiple_crates(mut stacks: Vec<Vec<char>>, rest: &str) -> String {
    for line in rest.lines() {
        let captures = RE.captures(line).unwrap();
        let count = captures.get(1).unwrap().as_str().parse::<usize>().unwrap_or(0);
        let from = captures.get(2).unwrap().as_str().parse::<usize>().unwrap_or(0) - 1;
        let to = captures.get(3).unwrap().as_str().parse::<usize>().unwrap_or(0) - 1;
        // Taking the crates that are being moved by slicing
        let moving_crates = (&stacks[from][stacks[from].len() - count..]).to_vec();
        stacks[to] = [stacks[to].clone(), moving_crates].concat();
        stacks[from] = (stacks[from][..stacks[from].len() - count]).to_vec();
    }

    stacks.iter().map(|line| line.last().unwrap_or(&' ')).collect::<String>()
}

pub fn solution() {
    println!("Well how am I gonna read this input?");
    // NOTE: It might not work depending on the system. Might need to split \n\r\n if set to CRLF
    let data: Vec<&str> = include_str!("../../input/day05/input.txt").split("\n\n").collect();
    let stacks = format_stacks(data[0]);
    println!("God damn did it took a long time to read the data: {}", move_crates(stacks.clone(), data[1]));
    println!("Moving multiple now: {}", move_multiple_crates(stacks.clone(), data[1]));
}