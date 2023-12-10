fn get_last_nums(seq: &Vec<i32>, mut last_nums: Vec<i32>, mut first_nums: Vec<i32>) -> (Vec<i32>, Vec<i32>) {
    if !seq.iter().find(|&&a| a != 0).is_some() {
        return (last_nums, first_nums);
    }
    let mut new_seq: Vec<i32> = Vec::new();

    // Build new sequence by getting differences
    for i in 0..seq.len() - 1 {
        new_seq.push(seq[i + 1] - seq[i]);
    }

    last_nums.push(*new_seq.last().unwrap());
    first_nums.push(*new_seq.first().unwrap());

    return get_last_nums(&new_seq, last_nums, first_nums);
}

fn get_those_sequences() -> (i32, i32) {
    let mut sum_1 = 0;
    let mut sum_2 = 0;
    let data = include_str!("../../input/day09/input.txt").lines().map(|a| a.split(" ").map(|b| b.parse::<i32>().unwrap()).collect::<Vec<i32>>()).collect::<Vec<Vec<i32>>>();

    // Part one:
    // 68 = 45 + 23 -> 68 = 45 + (15 + 8) -> 68 = 45 + (15 + (6 + 2)) -> 68 = 45 + (15 + (6 + (2 + 0)))
    // 68 = 45 + 15 + 6 + 2 (just last numbers of all of these)
    
    // Part two:
    // 5 = 10 - (3 - (0 - (2) ) )
    // 5 = a0 - (b0 - (c0 - (d0) ) )
    // 5 = a0 - (b0 - c0 + d0)
    // 5 = a0 - b0 + c0 - d0
    
    // 5 = 10 - 3 + 0 - 2

    for seq in data {
        let (last_nums, first_nums) = get_last_nums(&seq, vec![], vec![]);

        sum_1 += last_nums.iter().sum::<i32>() + seq.last().unwrap();

        sum_2 += seq.first().unwrap();
        for i in 0..first_nums.len() {
            if i % 2 != 0 {
                sum_2 += first_nums[i];
            } else {
                sum_2 -= first_nums[i];
            }
        }
    }

    (sum_1, sum_2)
}

pub fn solution() {
    println!("After coming back home, need to try to catch up again");
    let (a, b) = get_those_sequences();
    println!("Hmm, I don't really have an optimal solution idea {}", a);
    println!("Actually was way easier than I thought {}", b);
}
