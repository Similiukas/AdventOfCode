// Part one
fn count_visible(data: &Vec<Vec<char>>) -> usize {
    let mut result = 0;
    for (i, line) in data[1..data.len()-1].iter().enumerate() {
        for (j, tree) in line[1..line.len()-1].iter().enumerate() {
            let not_visible_left: bool = data[i+1][..j+1].iter().any(|item| item >= tree);
            let not_visible_right: bool = data[i+1][j+2..line.len()].iter().any(|item| item >= tree);
            let not_visible_up: bool = data[..i+1].iter().any(|item| item[j+1] >= *tree);
            let not_visible_down: bool = data[i+2..].iter().any(|item| item[j+1] >= *tree);
            
            if !not_visible_left || !not_visible_right || !not_visible_up || !not_visible_down {
                result += 1;
            }

        }
    }
    // Adding edge trees
    result + data.len() * 2 + data[0].len() * 2 - 4
}

// Part two
fn get_max_scenic_score(data: &Vec<Vec<char>>) -> usize {
    let mut max_scenic_score: usize = 0;
    for (i, line) in data[1..data.len()-1].iter().enumerate() {
        for (j, tree) in line[1..line.len()-1].iter().enumerate() {
            let mut scenic_left: usize = 0; let mut scenic_right: usize = 0; let mut scenic_up: usize = 0;let mut scenic_down: usize = 0;
            for item in data[i+1][..j+1].iter().rev() {
                scenic_left += 1;
                if item >= tree { break; }
            }
            for item in data[i+1][j+2..line.len()].iter() {
                scenic_right += 1;
                if item >= tree { break; }
            }
            
            for item in data[..i+1].iter().rev() {
                scenic_up += 1;
                if &item[j+1] >= tree { break; }
            }
            
            for item in data[i+2..].iter() {
                scenic_down += 1;
                if &item[j+1] >= tree { break; }
            }
            max_scenic_score = std::cmp::max(max_scenic_score, scenic_right * scenic_left * scenic_up * scenic_down);
        }
    }
    max_scenic_score
}

pub fn solution() {
    println!("Well maybe this day will be better");
    let data: Vec<Vec<char>> = include_str!("../../input/day08/input.txt").lines().map(|line| line.chars().collect::<Vec<char>>()).collect();
    println!("This is not pretty stuff: {}", count_visible(&data));
    println!("Same but different: {}", get_max_scenic_score(&data));
}