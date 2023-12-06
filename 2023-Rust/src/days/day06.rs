fn get_times(data: &Vec<&str>, is_time: bool, is_part_two: bool) -> Vec<i64> {
    let i = if is_time { 0 } else { 1 };
    if is_part_two {
        return vec![data[i][10..].split_whitespace().collect::<Vec<&str>>().join("").parse::<i64>().unwrap_or(0)];
    }
    return data[i][10..].split_whitespace().map(|a| a.parse::<i64>().unwrap()).collect();
}

// Using bruteforce + math. Could've also used just math since we know a and b, so we can find
// x1, x2 and then it's just solving quadratic inequality or simply x2 - x1 and
fn determine_error(is_part_two: bool) -> i32 {
    let mut sum = 1;
    let data: Vec<&str> = include_str!("../../input/day06/input.txt").lines().collect();
    let times = get_times(&data, true, is_part_two);
    let distances = get_times(&data, false, is_part_two);

    // The distance is calculated by S = Time * (held_time) - (held_time)^2, where held time <= Time
    // So the optimal solution is S` = 0, hence held_time = Time / 2
    // Then we iterate over held_time - 1, -2 , -3 etc. until we are no longer faster than the record
    for i in 0..times.len() {
        let mut num_of_viable_solutions = 0;
        let time = times[i];
        let mut optimal_time = time / 2;
        while time * optimal_time - optimal_time * optimal_time > distances[i] {
            optimal_time -= 1;
            num_of_viable_solutions += 1;
        }

        // Since we went only in one direction, there's twice as many solutions unless it's divisible by 2
        if time % 2 == 0 {
            num_of_viable_solutions = num_of_viable_solutions * 2 - 1;
        } else {
            num_of_viable_solutions *= 2;
        }
        sum *= num_of_viable_solutions;
    }

    sum

}

pub fn solution() {
    println!("Seems awfully like a math question here");
    println!("Maybe a quick part 1? {}", determine_error(false));
    println!("And no change part 2? {}", determine_error(true));
}