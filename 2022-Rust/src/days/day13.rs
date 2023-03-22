use std::{vec, cmp::Ordering};

// Checking if string type is array (for example: "[2,3]")
fn type_is_arr(item: &String) -> bool {
    item.chars().nth(0) == Some('[')
}

// Converting item to the vector if it's just a number
// "[2,3]" -> "[2, 3]" but "4" -> "[4]"
fn convert_to_vec(item: &String) -> String {
    if type_is_arr(item) { return item.to_string() }
    format!("[{item}]")
}

// If number stack is not empty, adding concatenated result and clearing the stack
fn add_to_number(number_stack: &mut Vec<char>, result: &mut Vec<String>) {
    if !number_stack.is_empty() {
        result.push(number_stack.iter().collect());
        number_stack.clear();
    }
}

// Parses items from string to array of items
// "[1,[2,3],5]" -> [1, "[2,3]", 5]
fn parser(_input: String) -> Vec<String> {
    let input = &_input[1.._input.len()-1];
    let mut result: Vec<String> = Vec::new();
    let mut brackets: Vec<usize> = Vec::new(); // Stack
    let mut number: Vec<char> = Vec::new();    // Stack (this allows numbers of more than one digit)

    // Going through the string one by one and counting brackets and number digits
    for (i, item) in input.chars().enumerate() {
        match item {
            '[' => { brackets.push(i) },
            ',' => { add_to_number(&mut number, &mut result) },
            ']' => {
                add_to_number(&mut number, &mut result);
                let first_bracket = brackets.pop().unwrap();
                if brackets.is_empty() {
                    // Adding 1 since need to add the ending ']'
                    result.push(input[first_bracket..=i].to_string());
                }
            },
            _ => {
                if brackets.is_empty() {
                    number.push(item);
                }
            }
        }
    }
    // Flushing the rest if there is
    // Note that into_iter() will consume the original stack
    if !number.is_empty() {
        result.push(number.into_iter().collect());
    }

    result
}

// Returns whether the items are in the right order
// Less -> correct order
// Equal -> undetermined
// Greater -> wrong order
fn compare(left: Vec<String>, right: Vec<String>) -> Ordering {
    for (i, item_a) in left.iter().enumerate() {
        // Right ran out of items before left, wrong order
        if i + 1 > right.len() { return Ordering::Greater }

        let item_b = &right[i];
        // Both are integers
        if !type_is_arr(item_a) && !type_is_arr(item_b){
            if item_a.parse::<i32>().unwrap() < item_b.parse::<i32>().unwrap() { return Ordering::Less }
            if item_a.parse::<i32>().unwrap() > item_b.parse::<i32>().unwrap() { return Ordering::Greater }
        }
         // Both are lists
        else if type_is_arr(item_a) && type_is_arr(item_b) {
            let result = compare(parser(item_a.clone()), parser(item_b.clone()));
            if result.is_ne() { return result }
        }
        else {
            let result = compare(parser(convert_to_vec(item_a)), parser(convert_to_vec(item_b)));
            if result.is_ne() { return result }
        }
    }
    // Left ran out of items, if right is longer then it's the correct order, otherwise undetermined
    if left.len() < right.len() { return Ordering::Less }
    Ordering::Equal
}

// Part one
fn get_indices() -> usize {
    let mut result = 0;
    let data: Vec<&str> = include_str!("../../input/day13/input.txt").split("\n\n").collect();
    for (i, pair) in data.iter().enumerate() {
        let l: Vec<&str> = pair.split('\n').collect();
        if compare(vec![l[0].to_string()], vec![l[1].to_string()]).is_lt() { result += i + 1 }
    }
    result
}

// Part two
fn get_sorted_indices() -> usize {
    let divider_1 = "[[2]]"; let divider_2 = "[[6]]";
    let mut first_divider = 0;

    let mut data: Vec<String> = include_str!("../../input/day13/input.txt").split("\n\n").fold(vec![],|acc, pair| {
        [acc, pair.lines().map(|item| { item.to_string()}).collect() ].concat()
    });
    data.push(divider_1.to_string());
    data.push(divider_2.to_string());

    data.sort_unstable_by(|a, b| compare(vec![a.to_string()], vec![b.to_string()]));
    for (i, pair) in data.iter().enumerate() {
        if pair == divider_1 { first_divider = i + 1 }
        else if pair == divider_2 { return first_divider * (i + 1) }
    }
    0
}

pub fn solution() {
    println!("Started to go way slower but I guess still going?");
    println!("The right order indices sum {}", get_indices());
    println!("The sorted indices decoder key: {}", get_sorted_indices());
}