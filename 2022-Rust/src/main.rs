mod days;
mod helpers;

use days::day05::solution;

use crate::helpers::Stack;

fn main() {
    println!("Oh, Rusty boi");
    solution();
    let mut r = Stack::new();
    r.push('a');
    println!("{:?}", r.peek());
    println!("{:?}", r.pop());
}
