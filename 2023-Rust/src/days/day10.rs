fn start(board: &Vec<Vec<String>>, starting_position: (usize, usize)) -> (usize, usize) {
    if starting_position.0 > 0 && ["|", "7", "F"].contains(&&*board[starting_position.0 - 1][starting_position.1]) {
        return (starting_position.0 - 1, starting_position.1)
    }

    if starting_position.0 < board.len() && ["|", "L", "J"].contains(&&*board[starting_position.0 + 1][starting_position.1]) {
        return (starting_position.0 + 1, starting_position.1)
    }

    if starting_position.1 > 0 && ["-", "L", "F"].contains(&&*board[starting_position.0][starting_position.1 - 1]) {
        return (starting_position.0, starting_position.1 - 1)
    }

    (starting_position.0, starting_position.1 + 1)
}

fn find_next(board: &Vec<Vec<String>>, position: (usize, usize), prev_position: (usize, usize)) -> (usize, usize) {
    let symbol = &board[position.0][position.1];
    let diff = (position.0 as i32 - prev_position.0 as i32, position.1 as i32 - prev_position.1 as i32);

    return match (diff, symbol.as_str()) {
        ((1, _), "|") | ((_, 1), "7") | ((_, -1), "F") => (position.0 + 1, position.1),
        ((_, 1), "-") | ((-1, _), "F") | ((1, _), "L") => (position.0, position.1 + 1),
        ((_, -1), "-") | ((-1, _), "7") | ((1, _), "J") => (position.0, position.1 - 1),
        ((-1, _), "|") | ((_, 1), "J") | ((_, -1), "L") => (position.0 - 1, position.1),
        _ => (0, 0)
    };
}

fn go_furthest() -> i32 {
    let mut result = 0;
    let mut starting_position = (0, 0);
    let mut data: Vec<Vec<String>> = include_str!("../../input/day10/input.txt").lines().enumerate().map(|(i, a)| {
        a.chars().enumerate().map(|(j, b)| {
            if b == 'S' {
                starting_position = (i, j);
                "0".to_string()
            } else { b.to_string() }
        }).collect()
    }).collect();

    let mut previous_position = starting_position;
    let mut current_position = start(&data, starting_position);
    // println!("next {:?}", current_position);
    while data[current_position.0][current_position.1] != "0" {
        result = data[previous_position.0][previous_position.1].parse::<i32>().unwrap_or(0) + 1;
        let temp = current_position;
        current_position = find_next(&data, current_position, previous_position);
        previous_position = temp;
        data[previous_position.0][previous_position.1] = result.to_string();
        // println!("next {:?}", current_position);
    }

    for row in data {
        for column in row {
            print!("{column}")
        }
        println!()
    }

    (result + 1) / 2
}

// For part 2 would need to replace found path by for example X, then all pipes into Y or something.
// This is because 7 can be tricky, since it's a pipe but also a path
// Then, somehow try to and solve
pub fn solution() {
    println!("Let's try to have a productive week");
    println!("Maybe don't even need Dijkstra yet {}", go_furthest());
}