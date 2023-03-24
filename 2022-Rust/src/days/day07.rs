use std::collections::HashMap;

fn calculate_low_sums() -> u64 {
    let mut result: u64 = 0;
    let mut current_path: Vec<String> = Vec::new();
    let mut dir_sizes: HashMap<String, u64> = HashMap::new();
    for command in include_str!("../../input/day07/input.txt").lines() {
        match command.split(" ").collect::<Vec<&str>>()[..] {
            ["$", "cd", "/"] => { current_path.push("/ROOT".to_string()); }
            ["$", "cd", ".."] => { current_path.pop(); }
            ["$", "cd", dir] => {
                current_path.push(format!("/{dir}"));
                dir_sizes.insert(current_path.concat(), 0);
            }
            ["$", "ls"] | ["dir", _] => {}
            [size, _] => {
                // dir_sizes[&path_string] = dir_sizes[&path_string] + size.parse::<u32>().unwrap_or(0); Cannot do this: https://stackoverflow.com/a/30414450/9819103
                // Directory should already exist since we create it when we change into it
                *dir_sizes.entry(current_path.concat()).or_insert(0) += size.parse::<u64>().unwrap_or(0);
            }
            _ => {}
        }
    }

    // Going through all directories and then going through them again to see if `path` has any sub_paths
    // If it does, then adding sub directory size to the parent (or grand-parent) total size
    // It shouldn't matter that the HashMap is not ordered because we go through all the kids, grandchildren, etc.
    let temp: &HashMap<String, u64> = &dir_sizes.clone();
    for (path, size) in &mut dir_sizes {
        for (sub_path, sub_size) in temp {
            if path != sub_path && sub_path.starts_with(path) {
                println!("This {path} is a parent of {sub_path} ({sub_size})");
                *size += sub_size;
            }
        }
    }

    // Finding the result
    for (path, size) in dir_sizes {
        println!("Sizes?: {} {}", path, size);
        if size <= 100_000 {
            result += size;
        }
    }
    result
}

// So for the given example it works nicely, however for the test input we get 1344257 is too low
// But finally solved with the help of https://github.com/gahjelle/advent_of_code/blob/main/python/2022/07_no_space_left_on_device/aoc202207.py
// Didn't use their solution only made tests cases to see where the problem was (if directory contained only other directories but no files)
pub fn solution() {
    println!("Another one for today");
    println!("\nSo the final result is: {}", calculate_low_sums());
}