fn get_empty_columns(board: &Vec<Vec<char>>) -> Vec<usize> {
    let mut result = Vec::new();
    for i in 0..board[0].len() {
        if board.iter().find(|column| column[i] == '#').is_none() {
            result.push(i);
        }
    }
    result
}

fn taxicab_geometry() -> usize {
    let mut sum = 0;
    let mut empty_rows: Vec<usize> = Vec::new();
    let mut data: Vec<Vec<char>> = include_str!("../../input/day11/input.txt").lines().enumerate().map(|(i, a)| {
        if !a.contains("#") {
            empty_rows.push(i);
        }
        a.chars().enumerate().map(|(j, b)| {
            // if b == '#' { galaxies.push((i, j)) }
            b
        }).collect()
    }).collect();

    let empty_columns = get_empty_columns(&data);

    // Expand rows
    for (i, row) in empty_rows.iter().enumerate() {
        data.insert(row + i, vec!['.'; data[0].len()])
    }
    // Expand columns
    for (i, column) in empty_columns.iter().enumerate() {
        for j in 0..data.len() {
            data[j].insert(column + i, '.')
        }
    }
    // Get new galaxy positions
    let mut galaxies: Vec<(usize, usize)> = Vec::new();
    for (i, row) in data.iter().enumerate() {
        for (j, &elem) in row.iter().enumerate() {
            if elem == '#' {
                galaxies.push((i, j));
            }
        }
    }

    // Calculate taxicab geometry distances for all pairs (a + b) / 2 (n_C_2)
    for i in 0..galaxies.len() {
        for j in i..galaxies.len() {
            sum += galaxies[j].0.abs_diff(galaxies[i].0) + galaxies[j].1.abs_diff(galaxies[i].1);
        }
    }
    sum
}

// Euclidean geometry
// sqrt((6-1)^2 + (6-2)^2) = sqrt(25 + 16) = sqrt(41) -> 6.4

// Taxicab geometry (diagonal not allowed) however not unique anymore. The idea is that we can just
// go fully vertical and then fully horizontal. Basically in L shape instead of diagonal.
// 6 - 1 + 6 - 2 = 9

// Initial
// ..#.
// ....
// #...
// ....
// (0, 2), (2, 0) -> 4

// 1 expansion
// ...#..
// ......
// ......
// #.....
// ......
// ......
// (0, 3), (3, 0) -> 6

// 2 expansion
// ....#...
// ........
// ........
// ........
// #.......
// ........
// ........
// ........
// (0, 4), (4, 0) -> 8
// So basically, need to recalculate where the galaxies end up after expansion. If there is a column
// which is empty before the galaxy, add expansion to x coordinate for every empty column. Same with rows
fn taxicab_smarter() -> usize {
    let mut sum = 0;
    let mut galaxies: Vec<(usize, usize)> = Vec::new();
    let mut empty_rows: Vec<usize> = Vec::new();
    let data: Vec<Vec<char>> = include_str!("../../input/day11/input.txt").lines().enumerate().map(|(i, a)| {
        if !a.contains("#") {
            empty_rows.push(i);
        }
        a.chars().enumerate().map(|(j, b)| {
            if b == '#' { galaxies.push((i, j)) }
            b
        }).collect()
    }).collect();

    let empty_columns = get_empty_columns(&data);

    // Recalculate galaxy new position
    for i in 0..galaxies.len() {
        let mut new_galaxy = galaxies[i];
        for &row in &empty_rows {
            // If empty row is before galaxy, need to add expansion to it
            if galaxies[i].0 > row {
                new_galaxy.0 += 999_999;
            }
        }

        for &column in &empty_columns {
            if galaxies[i].1 > column {
                new_galaxy.1 += 999_999;
            }
        }

        galaxies[i] = new_galaxy;
    }

    // Calculate taxicab geometry
    for i in 0..galaxies.len() {
        for j in i..galaxies.len() {
            sum += galaxies[j].0.abs_diff(galaxies[i].0) + galaxies[j].1.abs_diff(galaxies[i].1);
        }
    }
    sum
}

pub fn solution() {
    println!("Okay, double digit days are hard");
    println!("Would be easy if not for expansion {}", taxicab_geometry());
    println!("Now let's be smart {}", taxicab_smarter());
}
