fn available_list(node: (usize, usize), map: &Vec<Vec<char>>) -> Vec<(usize, usize)> {
    let mut result: Vec<(usize, usize)> = Vec::new();
    let current_height: char = map[node.0][node.1];
    let c: (i32, i32) = (node.0 as i32, node.1 as i32);
    for (y, x) in [(c.0 - 1, c.1), (c.0 + 1, c.1), (c.0, c.1 - 1), (c.0, c.1 + 1)] {
        if  0 <= y && y < map.len() as i32 && 0 <= x && x < map[0].len() as i32 {
            let cc: (usize, usize) = (y as usize, x as usize);
            if map[cc.0][cc.1] != 'X' && current_height as usize + 1 >= map[cc.0][cc.1] as usize {
                // Reachable neighbour
                result.push((cc.0, cc.1))
            }
        }
    }
    result
}

fn create_graph() -> (Vec<(usize, usize)>, (usize, usize), Vec<Vec<i32>>, Vec<Vec<char>>) {
    let mut starting_positions: Vec<(usize, usize)> = Vec::new();
    let mut finish_position: (usize, usize) = (0, 0);
    let topology_map: Vec<Vec<char>> = include_str!("../../input/day12/input.txt").lines().enumerate().map(|(i, res)| {
        res.trim().chars().enumerate().map(|(j,item)| {
            if item == 'S' {
                starting_positions.insert(0, (i, j));
                'a'
            } else if item == 'E' {
                finish_position = (i, j);
                'z'
            } else if item == 'a' {
                starting_positions.push((i, j));
                'a'
            } else { item }
        }).collect()
    }).collect();
    // 1. To create an unvisited set with distances equal to infinity
    let unvisited_set: Vec<Vec<i32>> = vec![vec![i32::MAX; topology_map[0].len()]; topology_map.len()];
    (starting_positions, finish_position, unvisited_set, topology_map)
}

// The steps are from here: https://en.wikipedia.org/wiki/Dijkstra%27s_algorithm#Algorithm
// This is actually a really slow and inefficient implementation. Would need to use a priority queue for memory and speed optimization
fn dijkstra(i: usize, mut current: (usize, usize), finish_position: (usize, usize), unvisited: &[Vec<i32>], map: &[Vec<char>]) -> i32 {
    let mut unv: Vec<Vec<i32>> = unvisited.to_vec();
    let mut m: Vec<Vec<char>> = map.to_vec();
    //  2. Set 0 for the initial node
    unv[current.0][current.1] = 0;
    // If current node is X then we there is no path
    while current != finish_position && m[current.0][current.1] != 'X' {
        let current_distance: i32 = unv[current.0][current.1];
        // 3. Need to go through available neighbors to set their distances if smaller than already set
        for (y, x) in available_list(current, &m) {
            if current_distance + 1 > unvisited[y][x] { continue }
            unv[y][x] = current_distance + 1;   // All nodes have the length 1
        }
        // 4. Need to remove the current node from the unvisited set
        m[current.0][current.1] = 'X';

        // 6. Set the current node as the smallest unvisited
        let mut smallest = i32::MAX;
        for (y, line) in unv.iter().enumerate() {
            for (x, item) in line.iter().enumerate() {
                if m[y][x] != 'X' && smallest > *item {
                    smallest = *item;
                    current = (y, x);
                }
            }
        }
        // Functional way to find the smallest but way less readable
        // current = unv.iter().enumerate().fold(finish_position, |acc, (y, line)| {
        //     line.iter().enumerate().fold(acc, |smallest_pos, (x, item)| {
        //         if m[y][x] != 'X' && unv[smallest_pos.0][smallest_pos.1] > *item { (y, x) } else { smallest_pos }
        //     })
        // });
    }
    println!("Checked {}", i + 1);
    unv[finish_position.0][finish_position.1]
}

pub fn solution() {
    println!("It's getting tough a bit, oof");
    let (starting_points, finish_position, unvisited, map) = create_graph();
    // Passing vectors as slices since Vec has no Copy so basically if you pass simply a vector, then after the call, the values are going to changes
    // Because it's gonna be passed as reference? Maybe not as reference but the ownership is passed
    let mut smallest = dijkstra(0, starting_points[0], finish_position, unvisited.as_slice(), map.as_slice());
    // Part one
    println!("The shortest path has length of: {smallest}");
    println!("Need to check {}", starting_points.len());
    // Part two
    smallest = starting_points[1..].iter().enumerate().map(|(i, item)| dijkstra(i, *item, finish_position, unvisited.as_slice(), map.as_slice())).min().unwrap();
    println!("Overall smallest is: {smallest}");
}