use std::collections::HashSet;

fn get_instructions(line: &str) -> (&str, u8) {
    let arr: Vec<&str> = line.split(" ").collect();
    (arr[0], arr[1].parse().unwrap_or(0))
}

fn check_if_touching(head: (i32, i32), tail: (i32, i32)) -> bool {
    for i in -1..=1 {
        for j in -1..=1 {
            if tail.0 == head.0 + i && tail.1 == head.1 + j {
                return true;
            }
        }
    }
    return false;
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
    if check_if_touching(new_head, tail) { return (new_head, tail) }
    // Tail is not close, then tail becomes the last head position
    (new_head, head)
}

fn get_to_head(head: (i32, i32), tail: (i32, i32)) -> (i32, i32) {
    // If still touching
    if check_if_touching(head, tail) { return tail; }

    return match (head.0 - tail.0, head.1 - tail.1) {
        (0, 2)  => (tail.0, tail.1 + 1),
        (0, -2) => (tail.0, tail.1 - 1),
        (2, 0)  => (tail.0 + 1, tail.1),
        (-2, 0) => (tail.0 - 1, tail.1),
        (2, 2) | (1, 2) | (2, 1) => (tail.0 + 1, tail.1 + 1),
        (-2, 2) | (-1, 2) | (-2, 1) => (tail.0 - 1, tail.1 + 1),
        (-2, -2) | (-1, -2) | (-2, -1) => (tail.0 - 1, tail.1 - 1),
        (2, -2) | (1, -2) | (2, -1) => (tail.0 + 1, tail.1 - 1),
        _ => tail
    };
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
            // Move the main head and it's tail accordingly
            (heads[0], heads[1]) = simulate_single(heads[0], heads[1], direction);
            // Go through the rest of the tails and move them
            (2..10).for_each(|i| {
                heads[i] = get_to_head(heads[i-1], heads[i]);
            });

            visited_positions.insert(heads[9]);
        });
    });
    visited_positions.len()
}

pub fn solution() {
    println!("After a break we're finally back");
    println!("Total visited places: {}", simulate());
    println!("Now with more heads! {}", simulate_larger());
}