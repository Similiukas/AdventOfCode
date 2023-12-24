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

fn move_north(_board: &Vec<Vec<char>>) -> Vec<Vec<char>>{
    let mut board = _board.to_vec();
    for i in 1..board.len() {
        for j in 0..board[0].len() {
            if board[i][j] == 'O' && board[i - 1][j] == '.' {
                let mut k = (i as i32) - 1;
                while k >= 0 && board[k as usize][j] == '.' {
                    board[k as usize][j] = 'O';
                    board[(k+1) as usize][j] = '.';
                    k -= 1;
                }
            }
        }
    }
    board
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
//     Replace 0. with .0
}

fn get_loaded() -> usize {
    let mut sum = 0;
    let data: Vec<Vec<char>> = include_str!("../../input/day14/input.txt").lines().map(|line| line.chars().collect()).collect();


    // Need proper cycle detection. Need to store all boards and then see it any one of previous boards has been reached
    // let mut prev_board = data.to_vec();
    // let mut new_board = move_north(&data);
    // for i in 0..1000000000 {
    //     if check_equal(&new_board, &prev_board) {
    //         println!("Found cycle {i}");
    //         break;
    //     }
    //     prev_board = new_board.to_vec();
    //     new_board = move_north(&new_board);
    // }

    // println!("same {}", check_equal(&data, &data));

    let new_board = move_south(&data);
    for i in 0..new_board.len() {
        for j in 0..new_board[0].len() {
            print!("{}", new_board[i][j]);
            if new_board[i][j] == 'O' {
                sum += data.len() - i
            }
        }
        println!();
    }
    sum
}

pub fn solution() {
    println!("While we wait for dinner, let's cook here");
    println!("Digididziau {}", get_loaded());
}