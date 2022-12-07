use std::fs;

fn get_fattest_elf() -> i32 {
    let data: String = fs::read_to_string("input/day01/input.txt").expect("Unable to read file");
    let mut max: i32 = 0; let mut sum: i32 = 0;

    for line in data.split("\n") {
        if !line.trim().is_empty() {
            sum += line.trim().parse::<i32>().unwrap_or(0);
        } else {
            max = std::cmp::max(max, sum);
            sum = 0;
        }
    }

    return max;
}

fn get_three_fattest_elfs() -> i32 {
    let data: String = fs::read_to_string("input/day01/input.txt").expect("Unable to read file");
    let mut max: i32 = 0; let mut mid: i32 = 0; let mut min: i32 = 0; let mut sum: i32 = 0;

    for line in data.split('\n') {
        if !line.trim().is_empty() {
            sum += line.trim().parse::<i32>().unwrap_or(0);
        } else {
            if sum > max {
                min = mid;
                mid = max;
                max = sum;
            } else if sum > mid {
                min = mid;
                mid = sum;
            } else if sum > min {
                min = sum;
            }
            sum = 0;
        }
    }
    return max + mid + min;
}

pub fn solution() {
    println!("Well lesgo in work by starting a bit late");
    println!("Result: {}", get_fattest_elf());
    println!("All three together: {}", {get_three_fattest_elfs()});
}
