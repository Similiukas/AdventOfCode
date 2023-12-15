fn solve_row(row: &[u8], kazkas: Option<usize>, remaining_groups: &[usize]) -> usize {
    if row.is_empty() {
        return match (kazkas, remaining_groups.len()) {
            (None, 0) => 1,
            (Some(x), 1) if x == remaining_groups[0] => 1,
            _ => 0
        }
    }
    if kazkas.is_some() && remaining_groups.is_empty() {
        return 0;
    }

    let ways = match (row[0], kazkas) {
        (b'.', Some(x)) if x != remaining_groups[0] => 0,
        (b'.', Some(_)) => solve_row(&row[1..], None, &remaining_groups[1..]),
        (b'.', None)    => solve_row(&row[1..], None, remaining_groups),
        (b'#', Some(_)) => solve_row(&row[1..], kazkas.map(|a| a + 1), remaining_groups),
        (b'#', None)    => solve_row(&row[1..], Some(1), remaining_groups),
        (b'?', Some(x)) => {
            let mut ans = solve_row(&row[1..], kazkas.map(|a| a + 1), remaining_groups);
            if x == remaining_groups[0] {
                ans += solve_row(&row[1..], None, &remaining_groups[1..]);
            }
            ans
        },
        (b'?', None)    => solve_row(&row[1..], Some(1), remaining_groups) + solve_row(&row[1..], None, remaining_groups),
        _ => unreachable!()
    };

    ways
}

fn nonogram() -> usize {
    let mut sum = 0;
    let data: Vec<(&str, Vec<usize>)> = include_str!("../../input/day12/input.txt").lines().map(|a| {
        let (left, right) = a.split_once(" ").unwrap();
        let c: Vec<usize> = right.split(",").map(|d| d.parse::<usize>().unwrap()).collect();
        (left, c)
    }).collect();

    println!("data {:?}", data);

    for row in data {
        let k = solve_row(row.0.as_bytes(), None, &row.1);
        println!("nu? {k}");
        sum += k;
    }

    sum
}

pub fn solution() {
    println!("Looks like another math problem");
    println!("Let's try this probability {}", nonogram());
}

// https://github.com/AxlLind/AdventOfCode2023/blob/main/src/bin/12.rs
// https://github.com/shemetz/advent_of_code_2023/blob/main/day12.py