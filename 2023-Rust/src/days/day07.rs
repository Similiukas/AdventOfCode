use std::cmp::Ordering;

fn get_symbol_rank(symbol: char, is_part_two: bool) -> i32 {
    return match symbol {
        'A' => 0,
        'K' => 1,
        'Q' => 2,
        'J' => if is_part_two { 12 } else { 3 },
        'T' => 4 - is_part_two as i32,
        '9' => 5 - is_part_two as i32,
        '8' => 6 - is_part_two as i32,
        '7' => 7 - is_part_two as i32,
        '6' => 8 - is_part_two as i32,
        '5' => 9 - is_part_two as i32,
        '4' => 10 - is_part_two as i32,
        '3' => 11 - is_part_two as i32,
        _ => 12 - is_part_two as i32
    }
}

fn get_type(hand: &str) -> i32 {
    let mut symbols: Vec<char> = hand.chars().collect();
    symbols.sort();
    // Five of a kind
    if symbols[0] == symbols[4] {
        return 0;
    }
    // Four of a kind
    if symbols[0] == symbols[3] || symbols[1] == symbols[4] {
        return 1;
    }
    // Full house
    if (symbols[0] == symbols[1] && symbols[2] == symbols[4]) || (symbols[0] == symbols[2] && symbols[3] == symbols[4]) {
        return 2;
    }
    // Three of a kind
    if symbols[0] == symbols[2] || symbols[1] == symbols[3] || symbols[2] == symbols[4] {
        return 3;
    }
    // Two pair
    if (symbols[0] == symbols[1] && (symbols[2] == symbols[3] || symbols[3] == symbols[4])) || (symbols[1] == symbols[2] && symbols[3] == symbols[4]) {
        return 4;
    }
    // One pair
    if symbols[0] == symbols[1] || symbols[1] == symbols[2] || symbols[2] == symbols[3] || symbols[3] == symbols[4] {
        return 5;
    }
    return 6;
}

fn get_max_type(hand: &str, is_part_two: bool) -> i32 {
    let mut rank = i32::MAX;
    // If part two, then replace J with something and check if it's a better hand
    if is_part_two && hand.contains("J") {
        for symbol in ["A", "K", "Q", "T", "9", "8", "7", "6", "5", "4", "3", "2"] {
            rank = rank.min(get_type(&hand.replace("J", symbol)));
            // Found the best one, no need to check more
            if rank == 0 { break }
        }
    } else {
        return get_type(hand);
    }
    rank
}
fn get_stronger(a: &(&str, usize), b: &(&str, usize), is_part_two: bool) -> Ordering {
    // First, try to sort by type
    let rank_a = get_max_type(a.0, is_part_two);
    let rank_b = get_max_type(b.0, is_part_two);
    if rank_a < rank_b {
        return Ordering::Greater
    } else if rank_a > rank_b {
        return Ordering::Less
    }
    // Same type, look symbol by symbol
    for (symbol_a, symbol_b) in a.0.chars().zip(b.0.chars()) {
        let symbol_rank_a = get_symbol_rank(symbol_a, is_part_two);
        let symbol_rank_b = get_symbol_rank(symbol_b, is_part_two);
        if symbol_rank_a < symbol_rank_b {
            return Ordering::Greater
        } else if symbol_rank_a > symbol_rank_b {
            return Ordering::Less
        }
    }
    // Should never happen
    return Ordering::Equal
}

fn get_winnings(is_part_two: bool) -> usize {
    let mut sum = 0;
    let data: Vec<Vec<&str>> = include_str!("../../input/day07/input.txt").lines().map(|a| a.split(" ").collect::<Vec<&str>>()).collect();
    let mut hands: Vec<(&str, usize)> = data.iter().map(|a| (a[0], a[1].parse::<usize>().unwrap())).collect();

    hands.sort_by(|a, b| get_stronger(a, b, is_part_two));

    for (i, &hand) in hands.iter().enumerate() {
        sum += (i + 1) * hand.1;
    }

    sum
}

pub fn solution() {
    println!("Well let's code poker then");
    println!("Maybe it's not too hard at first {}", get_winnings(false));
    println!("You got to be joking! {}", get_winnings(true));
}
