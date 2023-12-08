use std::collections::HashMap;
use regex::Regex;
use lazy_static::lazy_static;

lazy_static! {
    static ref RE: Regex = Regex::new(r"(\w+) = \((\w+), (\w+)\)").unwrap();
}

fn _gcd(a: usize, b: usize) -> usize {
    return if b == 0 { a } else { _gcd(b, a % b ) }
}
fn gcd(nums: &Vec<usize>) -> usize {
    nums.iter().fold(0, |acc, &elem| _gcd(acc, elem))
}

// The idea is that there are multiple rings. That is, there are multiple starting positions which
// lead to the same ending positions (1A -> 2B -> 3Z -> 1A) and (XA -> 4D -> XZ -> XA) and these rings
// do not intercept. If they did, then it would be a bigger ring or some starting positions would have no end
fn traverse(is_part_two: bool) -> usize {
    let data: Vec<&str> = include_str!("../../input/day08/input.txt").lines().collect();
    let directions: Vec<char> = data[0].chars().collect();

    let mut starting_positions: Vec<&str> = vec![];
    let mut map: HashMap<&str, (&str, &str)> = HashMap::new();
    for group in &data[2..] {
        let captures = RE.captures(group).unwrap();
        let (pos, L, R) = (captures.get(1).unwrap().as_str(), captures.get(2).unwrap().as_str(), captures.get(3).unwrap().as_str());
        map.insert(pos, (L, R));

        if is_part_two && &pos[2..] == "A" {
            starting_positions.push(pos);
        }
    }

    // For part one, we're looking only for AAA ring
    if !is_part_two {
        starting_positions.push("AAA");
    }

    // Go through every ring independently and get their sizes
    let mut i: Vec<usize> = vec![0; starting_positions.len()];
    for (j, &starting_position) in starting_positions.iter().enumerate() {
        let mut current_position = starting_position;
        while &current_position[2..] != "Z" {
            if directions[i[j] % directions.len()] == 'L' {
                current_position = map.get(current_position).unwrap().0;
            } else {
                current_position = map.get(current_position).unwrap().1;
            }
            i[j] += 1;
        }
    }

    // All the rings reset at LCM
    let gcd = gcd(&i);
    // Need to get LCM = (a * b) / GCD. Starting from GCD because we're starting from GCD
    i.iter().fold(gcd, |acc, elem| acc * elem / gcd)
}

pub fn solution() {
    println!("Last day here and on we move back home");
    println!("Let's try to see the easy looking part 1 {}", traverse(false));
    println!("Well, kinda saw hints about rings for this part {}", traverse(true));
}
