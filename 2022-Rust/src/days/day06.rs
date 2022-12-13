use std::collections::VecDeque;

fn contains_duplicates(arr: &VecDeque<char>) -> bool {
    for i in 0..arr.len() {
        for j in i+1..arr.len() {
            if arr[i] == arr[j] {
                return true
            }
        }
    }
    false
}

fn find_marker(is_part_1: bool) -> usize {
    let data = include_str!("../../input/day06/input.txt");
    let mut marker: VecDeque<char> = (&data[..(if is_part_1 {4} else {14})]).chars().collect::<VecDeque<char>>();
    // Checking the initial marker
    if !contains_duplicates(&marker) { return if is_part_1 {4} else {14} }

    for (i, item) in data[(if is_part_1 {4} else {14})..].chars().enumerate() {
        marker.pop_front();
        marker.push_back(item);
        if !contains_duplicates(&marker) {
            return i + (if is_part_1 {5} else {15});
        }
    }
    0
}

pub fn solution() {
    println!("Well, I guess still nothing else better to do");
    println!("This shouldn't be hard, right?: {}", find_marker(true));
    println!("Just a really simple change: {}", find_marker(false));
}