fn get_column(board: &[&str], column: usize) -> String {
    board.iter().fold("".to_owned(), |acc, elem| acc + &elem[column..column + 1])
}

fn count_differences(a: String, b: String) -> i32 {
    if a == b { return 0; }
    // Count how many pairs are different
    a.chars().zip(b.chars()).fold(0, |acc, elem| if elem.0 != elem.1 { acc + 1 } else { acc })
}

fn find_reflection(board: &Vec<&str>, max_diff: i32) -> usize {
    // rows
    for row in 1..board.len() {
        let mut i = row;
        let mut j = i - 1;
        let mut left_diff = max_diff - count_differences(board[i].to_string(), board[j].to_string());
        // Found same row, then going outside left and right to check if they're also the same
        while left_diff >= 0 {
            if j == 0 || i == board.len() - 1 {
                // For part two, we need to change exactly one smudge
                if max_diff == 1 && left_diff == 1 {
                    break;
                }
                return 100 * row;
            }
            i += 1;
            j -= 1;
            left_diff -= count_differences(board[i].to_string(), board[j].to_string());
        }
    }

    // Can combine these loops into one but then it's hard to read
    for column in 1..board[0].len() {
        let mut i = column;
        let mut j = i - 1;
        let mut left_diff = max_diff - count_differences(get_column(board, i), get_column(board, j));
        while left_diff >= 0 {
            if j == 0 || i == board[0].len() - 1 {
                if max_diff == 1 && left_diff == 1 {
                    break;
                }
                return column;
            }
            i += 1;
            j -= 1;
            left_diff -= count_differences(get_column(board, i), get_column(board, j));
        }
    }

    panic!("Should never happen");
}

fn get_those_reflections(is_part_two: bool) -> usize {
    let mut sum = 0;
    let data: Vec<&str> = include_str!("../../input/day13/input.txt").split("\n\n").collect();

    for pattern in data {
        // For part one, rows/columns need to be exactly the same while for part two exactly one needs to change
        // Don't care exactly one hence, counting how many change and if we do change 1, then can no longer change.
        sum += find_reflection(&pattern.lines().collect::<Vec<&str>>(), if is_part_two { 1 } else { 0 });
    }

    sum
}

pub fn solution() {
    println!("After some time off, back here again");
    println!("Maybe part one is not too bad {}", get_those_reflections(false));
    println!("With a hint it's nice {}", get_those_reflections(true));
}
