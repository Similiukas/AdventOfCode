fn draw_pixel(screen: &mut [[char; 40]; 6], register: i32, y: i32, x: i32) {
    if (register - x).abs() < 2 {
        screen[y as usize][x as usize] = '#';
    }
}

// Part one
fn run_cycles() -> i32 {
    let mut cycle = 0; let mut register = 1; let mut result = 0; let mut ignore_next: bool;
    let data = include_str!("../../input/day10/input.txt").lines();
    for command in data {
        if cycle > 220 { break; }
        ignore_next = false;
    
        if command.starts_with("n") { cycle += 1 }
        else {
            if (cycle - 19) % 40 == 0 {
                result += register * (cycle + 1);
            // This is the 20th, 60th, etc cycle for which the addition has not ended.
            // Thus register is still without addx value
            } else if (cycle - 18) % 40 == 0 {
                result += register * (cycle + 2);
                ignore_next = true;
            }
            cycle += 2;
            register += command[5..].parse().unwrap_or(0);
        }

        if (cycle - 20) % 40 == 0 && !ignore_next {
            result += register * cycle;
        }
    }
    result
}

// Part two
fn draw_screen() {
    let mut cycle: i32 = 0; let mut register: i32 = 1;
    let mut crt: [[char; 40]; 6] = [['.'; 40]; 6];
    
    include_str!("../../input/day10/input.txt").lines().for_each(|command| {
        // Drawing the pixel
        draw_pixel(&mut crt, register, cycle / 40, cycle % 40);
        // Ending the cycle
        if command.starts_with("n") { cycle += 1; } else {
            // Drawing the pixel again for the next cycle since it's going to be skipped by cycle += 2
            draw_pixel(&mut crt, register, (cycle + 1) / 40, (cycle + 1) % 40);
            register += command[5..].parse().unwrap_or(0);
            cycle += 2;
        }
    });

    println!("The result is:");
    crt.iter().for_each(|line| {
        println!("{}", line.iter().collect::<String>());
    })
}

pub fn solution() {
    println!("Might as well do more today");
    println!("Get all the registers: {}", run_cycles());
    draw_screen();
}