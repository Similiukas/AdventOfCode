// use crate::helpers::Stack;

fn format_stacks(input: &str) {
    let data = input.split("   ")
        .map(|cargo| {
            println!("Crate? [{}]", cargo.trim());
            cargo.trim().split(' ').map(|oth| {
                println!("ta pati row {oth}");
                oth
            }).collect::<Vec<&str>>()
        }).collect::<Vec<Vec<&str>>>();
    println!("{:?}", data);
}

// fn move_crates() {

// }

pub fn solution() {
    println!("Well how am I gonna read this input?");
    format_stacks("    [D]    
    [N] [C]    
    [Z] [M] [P]
     1   2   3 ");
}