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
                return (new_head, tail);
            }
        }
    }
    // Tail is not close, then tail becomes the last head position
    (new_head, head)
}

fn get_to_head(last_head: (i32, i32), new_head: (i32, i32), tail: (i32, i32)) -> (i32, i32) {
    // Still touching
    for i in -1..=1 {
        for j in -1..=1 {
            if tail.0 == new_head.0 + i && tail.1 == new_head.1 + j {
                return tail;
            }
        }
    }
    // Need to move diagonally
    if (tail.0 + 2 == new_head.0 && tail.1 + 1 == new_head.1) || (tail.0 + 1 == new_head.0 && tail.1 + 2 == new_head.1) || (tail.0 + 2 == new_head.0 && tail.1 + 2 == new_head.1) { // Diagonally up right
        return (tail.0 + 1, tail.1 + 1);
    }
    else if (tail.0 - 2 == new_head.0 && tail.1 + 1 == new_head.1) || (tail.0 - 1 == new_head.0 && tail.1 + 2 == new_head.1) || (tail.0 - 2 == new_head.0 && tail.1 + 2 == new_head.1) { // Diagonally down right
        return (tail.0 - 1, tail.1 + 1);
    }
    else if (tail.0 - 2 == new_head.0 && tail.1 - 1 == new_head.1) || (tail.0 - 1 == new_head.0 && tail.1 - 2 == new_head.1) || (tail.0 - 2 == new_head.0 && tail.1 - 2 == new_head.1) { // Diagonally down left
        return (tail.0 - 1, tail.1 - 1);
    }
    else if (tail.0 + 2 == new_head.0 && tail.1 - 1 == new_head.1) || (tail.0 + 1 == new_head.0 && tail.1 - 2 == new_head.1) || (tail.0 + 2 == new_head.0 && tail.1 - 2 == new_head.1) { // Diagonally up left
        return (tail.0 + 1, tail.1 - 1);
    }

    if tail.0 == new_head.0 && tail.1 + 2 == new_head.1 {
        return (tail.0, tail.1 + 1);
    } else if tail.0 == new_head.0 && tail.1 - 2 == new_head.1 {
        return (tail.0, tail.1 - 1);
    } else if tail.0 - 2 == new_head.0 && tail.1 == new_head.1 {
        return (tail.0 - 1, tail.1);
    } else if tail.0 + 2 == new_head.0 && tail.1 == new_head.1 {
        return (tail.0 + 1, tail.1);
    }
    println!("usual {tail:?} {new_head:?}");
    return last_head;
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
            // let (new_head, new_tail) = simulate_single(heads[0], heads[1], direction);
            // heads[0] = new_head;
            // println!("New head position {:?} {:?}",heads[0], heads[1]);
            // (2..10).for_each(|i| {
            //     let (_, new_new_tail) = simulate_single(new_tail, heads[i], "");
            //     heads[i-1] = new_tail;
            //     println!("New tail position {:?} {i}", heads);
            // });


            let mut temp_1: (i32, i32); let mut temp_2: (i32, i32);
            (heads[0], temp_1) = simulate_single(heads[0], heads[1], direction);

            temp_2 = get_to_head(heads[1], temp_1, heads[2]);
            heads[1] = temp_1;

            temp_1 = get_to_head(heads[2], temp_2, heads[3]);
            heads[2] = temp_2;

            temp_2 = get_to_head(heads[3], temp_1, heads[4]);
            heads[3] = temp_1;

            temp_1 = get_to_head(heads[4], temp_2, heads[5]);
            heads[4] = temp_2;

            temp_2 = get_to_head(heads[5], temp_1, heads[6]);
            heads[5] = temp_1;

            temp_1 = get_to_head(heads[6], temp_2, heads[7]);
            heads[6] = temp_2;

            temp_2 = get_to_head(heads[7], temp_1, heads[8]);
            heads[7] = temp_1;

            heads[9] = get_to_head(heads[8], temp_2, heads[9]);
            heads[8] = temp_2;

            // (1..8).for_each(|i| {
            //     temp_2 = get_to_head(heads[i], temp_1, heads[i+1]);
            //     heads[i] = temp_1;

            //     temp_1 = get_to_head(heads[i+1], temp_2, heads[i+2]);
            //     heads[i+1] = temp_2;
            // });

            // println!("New head position {:?} {:?} {:?}",heads[0], temp, heads[1]);
            // (2..10).for_each(|i| {
            //     // simulate
            //     (_, temp) = simulate_single(heads[i-1], heads[i], direction);
            //     heads[i-1] = temp;
            //     println!("New tail position {:?} {i}", heads);
            // });

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