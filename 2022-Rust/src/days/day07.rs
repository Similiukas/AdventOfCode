use std::collections::HashMap;

fn calculate_low_sums() -> u64 {
    let mut result: u64 = 0;
    let data = include_str!("../../input/day07/input.txt").lines();
    let mut current_path: Vec<String> = Vec::new();
    let mut dir_sizes: HashMap<String, u64> = HashMap::new();
    for command in data {
        match &command[..4] {
            "$ cd" => {
                // Changing directories
                let change_to = &command[5..];
                if change_to == ".." {
                    current_path.pop();
                } else {
                    current_path.push(format!("/{}", change_to.replace("//", "/")));
                }
            }
            "$ ls" => {
                // println!("ls command of path {}", current_path.concat());
            }
            "dir " => {
                // println!("Found another directory of name {}", &command[4..]);
            }
            _ => {
                // Calculating the size of the current directory items (excluding child directories)
                let a: Vec<&str> = command.split(' ').collect();
                // dir_sizes[&path_string] = dir_sizes[&path_string] + a[0].parse::<u32>().unwrap_or(0); Cannot do this: https://stackoverflow.com/a/30414450/9819103
                *dir_sizes.entry(current_path.concat()).or_insert(0) += a[0].parse::<u64>().unwrap_or(0);
            }
        }
    }

    // Going through all directories and then going through them again to see if `path` has any sub_paths
    // If it does, then adding sub directory size to the parent (or grand-parent) total size
    // It shouldn't matter that the HashMap is not ordered because we go through all the kids, grandchildren, etc.
    let temp = &dir_sizes.clone();
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
pub fn solution() {
    println!("Another one for today");
    println!("So the final result is: {}", calculate_low_sums());
}