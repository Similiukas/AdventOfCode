fn transpose(original: Vec<Vec<i64>>) -> Vec<Vec<i64>> {
    // From mxn create nxm matrix
    let mut transposed = (0..original[0].len()).map(|_| vec![]).collect::<Vec<_>>();

    // Go through row
    for original_row in original {
        // Go through element of original row and add them to the now column
        for (item, transposed_row) in original_row.into_iter().zip(&mut transposed) {
            // Add item to new row
            transposed_row.push(item);
        }
    }
    transposed
}

// Bruteforce part 1 which is failed because of memory issues.
// The idea is to build a hashmap of all values and then simply use get()
// or use the same number if get returned None
// fn build_map(matrix: Vec<Vec<i64>>) -> HashMap<i64, i64> {
//     let source_starts = &matrix[0]; let destination_starts = &matrix[1]; let ranges = &matrix[2];
//     let mut result: HashMap<i64, i64> = Default::default();
//     for (i, &source_start) in source_starts.iter().enumerate() {
//         for (j, map) in (source_start..source_start + ranges[i]).enumerate() {
//             result.insert(destination_starts[i] + j as i64, map);
//         }
//     }
//     result
// }

fn get_mapped_result(matrix: &Vec<Vec<i64>>, num: i64) -> i64 {
    // Since the hashmap would just repeat for ranges, we can just calculate what is the value
    let source_starts = &matrix[1];
    for (i, source_start) in source_starts.iter().enumerate() {
        // If num is in the range, find the value
        if num >= *source_start && num < source_start + matrix[2][i] {
            return matrix[0][i] + (num - source_start);
        }
    }
    // Value is not in the hashmap
    num
}

// Go through the ranges and depending on where the seed is, adjust it and maybe split into two
// and then through that new one recursively.
fn get_split_arrays(seeds: &(i64, i64), numbers: &Vec<(i64, i64, i64)>) -> Vec<(i64, i64)> {
    for row in numbers {
        // row_start <= seed_start <= row_end
        if seeds.0 >= row.0 && seeds.0 <= row.1 {
            // whole seed is within row
            // row_start <= seed_start seed_end <= row_end
            return if seeds.1 <= row.1 {
                // Do not split, just modify the whole seed range
                vec![(seeds.0 + row.2, seeds.1 + row.2)]
            // row_start <= seed_start <= row_end < seed_end
            } else {
                // The end is over the row range -> split right
                // Additional seed range is from row_end + 1 to seed_end
                let mut additional = get_split_arrays(&(row.1 + 1, seeds.1), numbers);
                // New seed range is from seed_start to row_end
                additional.push((seeds.0 + row.2, row.1 + row.2));
                additional
            }
        // seed_start < row_start <= seed_end <= row_end
        } else if seeds.1 >= row.0 && seeds.1 <= row.1 {
            // Additional range is from seed_start to row_end - 1
            let mut additional = get_split_arrays(&(seeds.0, row.0 - 1), numbers);
            // New seed range is from row_start to seed_end
            additional.push((row.0 + row.2, seeds.1 + row.2));
            return additional;
        }
    }

    // If nothing changed, return the same seed
    return vec![(seeds.0, seeds.1)];
}

// Part one
fn lowest_location() -> i64 {
    let data: Vec<&str> = include_str!("../../input/day05/input.txt").split("\n\n").collect();
    let mut initial_seeds: Vec<i64> = (&data[0][7..]).split(" ").map(|a| a.parse::<i64>().unwrap_or(0)).collect();
    for map in &data[1..] {
        let lines = map.split("\n").filter(|a| !a.is_empty()).collect::<Vec<&str>>();
        let numbers: Vec<Vec<i64>> = transpose(lines[1..].iter().map(|a | {
            let nums = a.split(" ").map(|b| b.parse::<i64>().unwrap()).collect::<Vec<i64>>();
            vec![nums[0], nums[1], nums[2]]
        }).collect());

        // Part one failed
        // let map = build_map(numbers);
        // for i in 0..initial_seeds.len() {
        //     initial_seeds[i] = *map.get(&initial_seeds[i]).unwrap_or(&initial_seeds[i]);
        // }

        for i in 0..initial_seeds.len() {
            initial_seeds[i] = get_mapped_result(&numbers, initial_seeds[i]);
        }
    }

    initial_seeds.iter().fold(i64::MAX, |acc, elem| if *elem < acc { *elem } else { acc })
}

// Part two
fn lowest_range() -> i64 {
    let data: Vec<&str> = include_str!("../../input/day05/input.txt").split("\n\n").collect();
    let initial_seeds: Vec<i64> = (&data[0][7..]).split(" ").map(|a| a.parse::<i64>().unwrap_or(0)).collect();
    let mut seeds: Vec<(i64, i64)> = vec![];

    // Create seeds as ranges [(79 - 92), (55 - 67)]
    for i in (0..initial_seeds.len()).step_by(2) {
        let num = initial_seeds[i];
        seeds.push((num, num + initial_seeds[i+1] - 1));
    }

    for map in &data[1..] {
        let lines = map.split("\n").filter(|a| !a.is_empty()).collect::<Vec<&str>>();
        // Get as ranges, where third number is the map function (either add or subtract numbers of that range)
        // [(98 - 99, +2), (50 - 57, -48)]
        let numbers: Vec<(i64, i64, i64)> = lines[1..].iter().map(|a | {
            let nums = a.split(" ").map(|b| b.parse::<i64>().unwrap()).collect::<Vec<i64>>();
            (nums[1], nums[1] + nums[2] - 1, nums[0] - nums[1])
        }).collect();

        let mut new_seeds: Vec<(i64, i64)> = vec![];
        for seed in &seeds {
            // Go through the seeds and then adjust the new seed ranges and maybe split them
            new_seeds.append(&mut get_split_arrays(seed, &numbers));
        }
        seeds = new_seeds.to_vec();
    }
    seeds.iter().fold(i64::MAX, |acc, elem| if elem.0 < acc { elem.0 } else { acc })
}

pub fn solution() {
    println!("Finally early start of the day");
    println!("Seems not too hard so far {}", lowest_location());
    println!("Oh no, this is not gonna work {}", lowest_range());
}