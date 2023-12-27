use std::collections::HashMap;
use std::str::from_utf8;
use itertools::Itertools;

fn solve_row(row: &[u8], current_group_size: Option<usize>, remaining_groups: &[usize], memo: &mut HashMap<(String, usize, Vec<usize>), usize>) -> usize {
    // End of line
    if row.is_empty() {
        return match (current_group_size, remaining_groups.len()) {
            // If checking nothing, it's valid
            (None, 0) => 1,
            // Last group left and it the one we're checking should be that size
            (Some(x), 1) if x == remaining_groups[0] => 1,
            // If more than one group left or no groups but was checking something, then it's wrong
            _ => 0
        }
    }
    // Checking ### but there can't be any more groups
    if current_group_size.is_some() && remaining_groups.is_empty() {
        return 0;
    }

    // Dynamic programming part.
    // If it was already calculated, then just use the answer. Ugly key, prettier in git but it also works
    let key = (from_utf8(row).unwrap().to_string(), current_group_size.unwrap_or(0), remaining_groups.to_vec());
    if let Some(&x) = memo.get(&key) {
        return x;
    }

    let ways = match (row[0], current_group_size) {
        // Group ended but the got that it's not the proper size, hence returning 0 (####. 2)
        (b'.', Some(x)) if x != remaining_groups[0] => 0,
        // Group ended, need to solve the rest (####..# 4,1)
        (b'.', Some(_)) => solve_row(&row[1..], None, &remaining_groups[1..], memo),
        // Just go next
        (b'.', None)    => solve_row(&row[1..], None, remaining_groups, memo),
        // Go next
        (b'#', Some(_)) => solve_row(&row[1..], current_group_size.map(|a| a + 1), remaining_groups, memo),
        // Start new group
        (b'#', None)    => solve_row(&row[1..], Some(1), remaining_groups, memo),
        // Calculate both ways. Either continue # or also start . if current group works
        (b'?', Some(x)) => {
            let mut ans = solve_row(&row[1..], current_group_size.map(|a| a + 1), remaining_groups, memo);
            if x == remaining_groups[0] {
                ans += solve_row(&row[1..], None, &remaining_groups[1..], memo);
            }
            ans
        },
        // Calculate both ways. Either it's # or . so the answer is adding them together
        (b'?', None)    => solve_row(&row[1..], Some(1), remaining_groups, memo) + solve_row(&row[1..], None, remaining_groups, memo),
        _ => unreachable!()
    };

    // Dynamic programming part.
    // Store the calculated answer
    memo.insert(key, ways);
    ways
}

fn nonogram(is_part_two: bool) -> usize {
    let mut sum = 0;
    let size = if is_part_two { 5 } else { 1 };
    let data: Vec<(String, Vec<usize>)> = include_str!("../../input/day12/input.txt").lines().map(|a| {
        let (left, right) = a.split_once(" ").unwrap();
        let c: Vec<usize> = right.split(",").map(|d| d.parse::<usize>().unwrap()).collect();
        ((0..size).map(|_| left).join("?"), c.repeat(size))
    }).collect();

    let mut memo = HashMap::new();
    for row in data {
        sum += solve_row(row.0.as_bytes(), None, &row.1, &mut memo);
    }

    sum
}

pub fn solution() {
    println!("Looks like another math problem");
    println!("Let's try this probability {}", nonogram(false));
    println!("After learning DP, came back {}", nonogram(true));
}

// Dynamic programming explanation:
// https://www.youtube.com/watch?v=Hdr64lKQ3e4&pp=ygUTZHluYW1pYyBwcm9ncmFtbWluZw%3D%3D
// Essentially going recursion + memo of already calculated results. Best example is with Fibonacci numbers.
// To calculate F_3 = F_2 + F_1 = 1 + 1
// To calculate F_4 = F_3 + F_2 = F_2 + (from cache) F_3 = 1 + 2
// Essentially tree like form
// Still, the hardest part was to actually create the brute force recursion which was kinda copied from git
// Main idea is taken from here
// https://github.com/AxlLind/AdventOfCode2023/blob/main/src/bin/12.rs
