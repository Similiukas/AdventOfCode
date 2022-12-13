fn find_item_priorities() -> u32 {
    let result: u32 = include_str!("../../input/day03/input.txt")
        .split('\n')
        .map(|line| {
            // Split the line to two rucksacks
            let (a, b) = line.trim().split_at(line.len() / 2);
            // Go through first one
            a.chars().map(|item| {
                // If second one contains an item from first one
                if b.contains(item){
                    if item as u32 > 96 { item as u32 - 96 } else { item as u32 - 38 }
                } else { 0 }
            // Only take the first one above 0
            }).filter(|x| x > &0).next().unwrap_or(0)
        }).sum();
    return result;
}

fn find_badges() -> u32 {
    let mut result: u32 = 0;

    let lines = include_str!("../../input/day03/input.txt").lines().collect::<Vec<&str>>();

    for line in lines.chunks(3) {
        for item in line[0].chars() {
            if line[1].contains(item) && line[2].chars().any(|x| x == item) {
                result += if item as u32 > 96 { item as u32 - 96 } else { item as u32 - 38 };
                break;
            }
        }
    }
    return result;
}

pub fn solution() {
    println!("Another day, another work solver");
    println!("Result: {}", find_item_priorities());
    println!("Second one: {}", find_badges());
}