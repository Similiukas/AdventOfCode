use std::collections::HashMap;

fn calculate_wins(games: HashMap<&str, i32>) -> i32 {
    let result: _ = include_str!("../../input/day02/input.txt")
        .split('\n')
        .map(|m|games[m]).sum::<i32>();

    return result;
}

pub fn solution() {
    println!("Second for the day, need to catch up");
    let part1 = HashMap::from([
        ("A X", 1+3),   // Elf -> Rock      | Me -> Rock =>     draw
        ("A Y", 2+6),   // Elf -> Rock      | Me -> Paper =>    win
        ("A Z", 3+0),   // Elf -> Rock      | Me -> Scissors => lose
        ("B X", 1+0),   // Elf -> Paper     | Me -> Rock =>     lose  
        ("B Y", 2+3),   // Elf -> Paper     | Me -> Paper =>    draw
        ("B Z", 3+6),   // Elf -> Paper     | Me -> Scissors => win
        ("C X", 1+6),   // Elf -> Scissors  | Me -> Rock =>     win
        ("C Y", 2+0),   // Elf -> Scissors  | Me -> Paper =>    lose
        ("C Z", 3+3),   // Elf -> Scissors  | Me -> Scissors => draw
        ("", 0)
    ]);
    let part2 = HashMap::from([
        ("A X", 3+0),   // Elf -> Rock      | Me -> Scissors => lose
        ("A Y", 1+3),   // Elf -> Rock      | Me -> Rock =>     draw
        ("A Z", 2+6),   // Elf -> Rock      | Me -> Paper =>    win
        ("B X", 1+0),   // Elf -> Paper     | Me -> Rock =>     lose  
        ("B Y", 2+3),   // Elf -> Paper     | Me -> Paper =>    draw
        ("B Z", 3+6),   // Elf -> Paper     | Me -> Scissors => win
        ("C X", 2+0),   // Elf -> Scissors  | Me -> Paper =>    lose
        ("C Y", 3+3),   // Elf -> Scissors  | Me -> Scissors => draw
        ("C Z", 1+6),   // Elf -> Scissors  | Me -> Rock =>     win
        ("", 0)
    ]);
    println!("Result: {}", calculate_wins(part1));
    println!("Result: {}", calculate_wins(part2));
}