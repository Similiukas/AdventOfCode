use std::collections::HashSet;

fn get_instructions(line: &str) -> (&str, u8) {
    let arr: Vec<&str> = line.split(" ").collect();
    (arr[0], arr[1].parse().unwrap_or(0))
}

fn simulate_single(head: (i32, i32), tail: (i32, i32), direction: &str) -> ((i32, i32), (i32, i32)) {
    let new_head: (i32, i32) = match direction {
        "R" => (head.0, head.1 + 1),
        "L" => (head.0, head.1 - 1),
        "U" => (head.0 + 1, head.1),
        "D" => (head.0 - 1, head.1),
        _ => head
    };
    // If tail is in proximity to head, then no need to change tail
    for i in -1..=1 {
        for j in -1..=1 {
            if tail.0 == new_head.0 + i && tail.1 == new_head.1 + j {
                // println!("[{k}] New head: {new_head:?} {tail:?}");
                return (new_head, tail);
            }
        }
    }
    // Tail is not close, then tail becomes the last head position
    // println!("[{k}] New head: {new_head:?} {head:?}");
    (new_head, head)
}

// Part one
fn simulate() -> usize {
    let mut head = (0, 0); let mut tail = (0, 0);
    let mut visited_positions: HashSet<(i32, i32)> = HashSet::from([(0, 0)]);
    include_str!("../../input/day09/input.txt").lines().for_each(|line| {
        let (direction, move_count) = get_instructions(line);
        (0..move_count).for_each(|_| {
            (head, tail) = simulate_single(head, tail, direction);
            visited_positions.insert(tail);
        })
    });
    visited_positions.len()
}

// Part two
fn simulate_larger() -> usize {
    // heads[0] -> first head (H)
    // heads[9] -> tail (9)
    let mut heads: [(i32, i32); 10] = [(0, 0); 10];
    let mut visited_positions: HashSet<(i32, i32)> = HashSet::from([(0, 0)]);
    include_str!("../../input/day09/input.txt").lines().for_each(|line| {
        let (direction, move_count) = get_instructions(line);
        (0..move_count).for_each(|_| {
            (heads[0], heads[1]) = simulate_single(heads[0], heads[1], direction);
            println!("New head position {:?}",heads[0]);
            (2..10).for_each(|i| {
                (_, heads[i]) = simulate_single(heads[i-1], heads[i], "");
                println!("New tail position {:?} {i}", heads);
            });
            visited_positions.insert(heads[9]);
        });
        println!("heads: {heads:?}");
    });
    visited_positions.len()
}

pub fn solution() {
    println!("After a break we're finally back");
    println!("Total visited places: {}", simulate());
    println!("Now with more heads! {}", simulate_larger());
}