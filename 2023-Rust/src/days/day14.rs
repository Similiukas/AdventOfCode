use regex::Regex;
use lazy_static::lazy_static;

lazy_static! {
    static ref RE_E: Regex = Regex::new(r"(O+)(\.+)").unwrap();
    static ref RE_W: Regex = Regex::new(r"(\.+)(O+)").unwrap();
}

fn check_equal(a: &Vec<Vec<char>>, b: &Vec<Vec<char>>) -> bool {
    for i in 0..a.len() {
        for j in 0..a[0].len() {
            if a[i][j] != b[i][j] {
                return false
            }
        }
    }
    true
}

// Custom very bad hashing function to not store the whole board in memory.
// Also, gets the result of the board itself.
// Was actually the pain, since hashing properly is hard and can have false positives which shittier hash
fn board_hash(board: &Vec<Vec<char>>) -> (usize, usize) {
    let mut hash = 0; let mut sum = 0;
    for i in 0..board.len() {
        for j in 0..board[0].len() {
            if board[i][j] == 'O' {
                hash += (i + 6) + (j + 7) * 91 * (j + 3);
                sum += board.len() - i;
            }
        }
    }
    (hash, sum)
}

fn move_north(board: &Vec<Vec<char>>) -> Vec<Vec<char>>{
    let mut _board = board.to_vec();
    for i in 1.._board.len() {
        for j in 0.._board[0].len() {
            if _board[i][j] == 'O' && _board[i - 1][j] == '.' {
                let mut k = (i as i32) - 1;
                while k >= 0 && _board[k as usize][j] == '.' {
                    _board[k as usize][j] = 'O';
                    _board[(k+1) as usize][j] = '.';
                    k -= 1;
                }
            }
        }
    }
    _board
}

fn move_south(board: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    let mut _board = board.to_vec();
    for i in (0.._board.len()-1).rev() {
        for j in 0.._board[0].len() {
            if _board[i][j] == 'O' && _board[i + 1][j] == '.' {
                let mut k = i + 1;
                while k < _board.len() && _board[k][j] == '.' {
                    _board[k][j] = 'O';
                    _board[k-1][j] = '.';
                    k += 1;
                }
            }
        }
    }
    _board
}

fn move_east(board: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    // Replace 0. with .0
    let mut _board = board.to_vec();
    for i in  0.._board.len() {
        let mut row: String = _board[i].iter().collect();
        while RE_E.captures(&*row).is_some() {
            row = RE_E.replace(&*row, "$2$1").to_string();
            _board[i] = row.chars().collect::<Vec<char>>();
        }
    }
    _board
}

fn move_west(board: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    // Replace .0 with 0.
    let mut _board = board.to_vec();
    for i in 0..board.len() {
        let mut row: String = _board[i].iter().collect();
        while RE_W.captures(&*row).is_some() {
            row = RE_W.replace(&*row, "$2$1").to_string();
            _board[i] = row.chars().collect::<Vec<char>>();
        }
    }
    _board
}

// Part two
fn get_loaded() -> usize {
    let mut sum = 0;
    let mut data: Vec<Vec<char>> = include_str!("../../input/day14/input.txt").lines().map(|line| line.chars().collect()).collect();
    let mut visited_boards: Vec<(usize, usize)> = vec![board_hash(&data)];

    // Need proper cycle detection. Need to store all boards and then see it any one of previous boards has been reached
    // The cycle will probably be less than after 1000 tilts
    for i in 0..1000 {
        // Tilt
        data = match i % 4 {
            0 => move_north(&data),
            1 => move_west(&data),
            2 => move_south(&data),
            _ => move_east(&data),
        };
        // Get hash and load
        let board_data = board_hash(&data);

        // Find if board was already visited
        if visited_boards.iter().find(|&x| x.0 == board_data.0).is_some() {
            let cycle_start = (visited_boards.iter().position(|&x| x.0 == board_data.0).unwrap() - 1) as i64;
            let cycle_length = i - cycle_start;
            // Finding on which cycle element we will finish
            // Ignore the first elements which are not part of the cycle (cycle_start), then the remainder is the answer
            let index = (4 * 1_000_000_000i64 - cycle_start) % cycle_length;
            // At the end, we need to add all elements from the start which aren't part of the cycle.
            return visited_boards[(index + cycle_start) as usize].1;
        }

        // Board is new, add it to the list
        visited_boards.push(board_data);
    }
    sum
}

// Part one
fn get_easy() -> usize {
    let data: Vec<Vec<char>> = include_str!("../../input/day14/input.txt").lines().map(|line| line.chars().collect()).collect();
    board_hash(&move_north(&data)).1
}

pub fn solution() {
    println!("While we wait for dinner, let's cook here");
    println!("Digididziau {}", get_loaded());
    println!("After many tries, finally done {}", get_easy());
}
