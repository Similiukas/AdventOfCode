use std::collections::HashMap;

fn calculate_low_sums() {
    let data = include_str!("../../input/day07/input.txt").lines();
    let mut current_path: Vec<String> = Vec::new();
    let mut dir_sizes: HashMap<String, u32> = HashMap::new();
    for command in data {
        match &command[..4] {
            "$ cd" => {
                println!("cd to {}", &command[5..]);
                let change_to = &command[5..];
                if change_to == ".." {
                    current_path.pop();
                } else {
                    current_path.push(format!("/{}", change_to).replace("//", "/"));
                }
            }
            "$ ls" => {
                println!("ls command of path {}", current_path.concat());
            }
            "dir " => {
                println!("Found another directory of name {}", &command[4..]);
            }
            _ => {
                let a = command.split(' ').collect::<Vec<&str>>();
                println!("that's the size {} of {}", a[0], a[1]);
                let path_string: String = current_path.concat();
                // dir_sizes[&path_string] = dir_sizes[&path_string] + a[0].parse::<u32>().unwrap_or(0); Cannot do this: https://stackoverflow.com/a/30414450/9819103
                *dir_sizes.entry(path_string).or_insert(0) += a[0].parse::<u32>().unwrap_or(0);
            }
        }
    }
    for path in dir_sizes {
        println!("Sizes?: {} {}", path.0.replace("//", "/"), path.1);
    }
}

pub fn solution() {
    println!("Another one for today");
    calculate_low_sums();
}