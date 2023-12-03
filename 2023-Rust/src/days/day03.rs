fn check_adjacent(board: &Vec<Vec<char>>, row: i32, num: (i32, i32)) -> bool {
    for column in num.0..=num.1 {
        for i in -1..=1 {
            if row + i < 0 || row + i >= board.len() as i32 { continue }
            for j in -1..=1 {
                if column + j < 0 || column + j >= board[(row + i) as usize].len() as i32 || (i == 0 && j == 0) { continue }

                let element = board[(row + i) as usize][(column + j) as usize];
                if element != '.' && !element.is_digit(10) || (element.is_digit(10) && i != 0) {
                    return true;
                }

            }
        }
    }

    false
}

fn check_adjacent_numbers(board: &Vec<Vec<char>>, (row, column): (i32, i32)) -> Vec<(i32, i32)> {
    let mut found_digits: Vec<(i32, i32)> = vec![];
    for i in -1..=1 {
        if row + i < 0 || row + i >= board.len() as i32 { continue }
        for j in -1..=1 {
            if column + j < 0 || column + j >= board[(row + i) as usize].len() as i32 || (i == 0 && j == 0) { continue }

            if board[(row + i) as usize][(column + j) as usize].is_digit(10) {
                if !found_digits.is_empty() && found_digits.last().unwrap() == &(row+i, column+j-1) {
                    let temp = found_digits.len() - 1;
                    found_digits[temp] = (row+i, column+j);
                //     Saving only the left-most digit of same number
                } else {
                    found_digits.push((row+i, column+j));
                }
            }
        }
    }
    found_digits
}

// Building a number from it's position
fn build_number(board: &Vec<Vec<char>>, (row, column): (i32, i32)) -> u32 {
    let mut num = 0; let mut col = column; let mut m = 1;

    // First need to go left-most since adjacent might not be the most left-most of number itself
    while col + 1 < board[row as usize].len() as i32 && board[row as usize][(col + 1) as usize].is_digit(10) {
        col += 1;
    }

    while col >= 0 && board[row as usize][col as usize].is_digit(10) {
        num += board[row as usize][col as usize].to_digit(10).unwrap() * m;
        m *= 10;
        col -= 1;
    }
    num
}

// Part one
fn adjacent_stuff() -> i32 {
    let mut sum = 0;
    let data: Vec<Vec<char>> = include_str!("../../input/day03/input.txt").lines().map(|a| a.chars().collect()).collect();

    for (i, row) in data.iter().enumerate() {
        let mut num: (i32, i32) = (-1, -1);
        for (j, symbol) in row.iter().enumerate() {
            if symbol.is_digit(10) {
                if num.0 == -1 {
                    num = (j as i32, j as i32)
                } else {
                    num.1 = j as i32;
                }
            } else if num.0 != -1 {
                if check_adjacent(&data, i as i32, num) {
                    let new_num = (&row[num.0 as usize..=num.1 as usize]).iter().collect::<String>().parse::<i32>().unwrap();
                    sum += new_num;
                }
                num = (-1, -1);
            }
        }
        // Number might be at the end of line ....826
        if num.0 != -1 {
            if check_adjacent(&data, i as i32, num) {
                let new_num = (&row[num.0 as usize..=num.1 as usize]).iter().collect::<String>().parse::<i32>().unwrap();
                sum += new_num;
            }
        }
        println!();
    }
    sum
}

// Part two
fn ratio_levels() -> u32 {
    let mut sum = 0;
    let data: Vec<Vec<char>> = include_str!("../../input/day03/input.txt").lines().map(|a| a.chars().collect()).collect();
    for (i, row) in data.iter().enumerate() {
        for (j, symbol) in row.iter().enumerate() {
            if symbol == &'*' {
                // Looking through neighbours and getting indexes
                let adjacent = check_adjacent_numbers(&data, (i as i32, j as i32));
                // If * has two adjacent numbers, only then counting them
                if adjacent.len() == 2 {
                    sum += build_number(&data, adjacent[0]) * build_number(&data, adjacent[1]);
                }
            }
        }
    }
    sum
}

pub fn solution() {
    println!("Need to catch up with all the folks");
    println!("Again with the adjacent stuff {}", adjacent_stuff());
    println!("Now harder adjacent: {}", ratio_levels());
}
