use regex::Regex;
use lazy_static::lazy_static;

use crate::helpers::re_to_int;

// Optimization to compile Regex only once
lazy_static! {
    static ref RE: Regex = Regex::new(r"Monkey \d+:\s+Starting items: (.+)\s+Operation: new = old (.+)\s+Test: divisible by (\d+)\s+If true: throw to monkey (\d+)\s+If false: throw to monkey (\d+)").unwrap();
}

#[derive(Debug)]
struct Monkey<'a> {
    items: Vec<usize>,
    operation: &'a str,
    test: usize,
    success: usize,
    failure: usize,
    inspected_an_item: usize
}

fn build_monkey(line: &str) -> Monkey {
    let captures = RE.captures(line).unwrap();
    let items: Vec<usize> = captures.get(1).unwrap().as_str().split(", ").map(|x| x.parse().unwrap_or(0)).collect();
    let operation: &str = captures.get(2).unwrap().as_str();
    let test: usize = re_to_int(&captures, 3);
    let success: usize = re_to_int(&captures, 4);
    let failure: usize = re_to_int(&captures, 5);
    Monkey { items, operation, test, success, failure, inspected_an_item: 0 }
}

fn perform_operation(operation: &str, old: usize) -> usize {
    let operand: usize = operation[2..].parse().unwrap_or(old);
    return match &operation[0..1] {
        "*" => old * operand,
        "+" => old + operand,
        // "/" => old / operand, -> never used, the numbers only get larger
        // _ => old - operand,
        _ => old
    }
}

fn get_monkey_business() -> usize {
    let mut largest = 0; let mut smaller = 0;
    let mut monkeys: Vec<Monkey> = include_str!("../../input/day11/input.txt").split("\n\n").map(|res| {
        build_monkey(res)
    }).collect();

    for r in 0..20 {
        for i in 0..monkeys.len() {
            let success_monkey_index: usize = monkeys[i].success;
            let failure_monkey_index: usize = monkeys[i].failure;

            for item_i in 0..monkeys[i].items.len() {
                let new_level: usize = perform_operation(monkeys[i].operation, monkeys[i].items[item_i]) / 3;
                monkeys[i].inspected_an_item += 1;
                if new_level % monkeys[i].test == 0 {
                    monkeys[success_monkey_index].items.push(new_level);
                } else {
                    monkeys[failure_monkey_index].items.push(new_level);
                }
            }
            monkeys[i].items = vec![];

            if monkeys[i].inspected_an_item > largest {
                smaller = largest;
                largest = monkeys[i].inspected_an_item;
            } else if monkeys[i].inspected_an_item > smaller {
                smaller = monkeys[i].inspected_an_item;
            }
        }
    }

    println!("Monkey 0 {:?}", monkeys[0]);
    println!("Monkey 1 {:?}", monkeys[1]);
    println!("Monkey 2 {:?}", monkeys[2]);
    println!("Monkey 3 {:?}", monkeys[3]);
    largest * smaller
}

pub fn solution() {
    println!("First time back to this after advent");
    println!("Result is: {}", get_monkey_business());
}