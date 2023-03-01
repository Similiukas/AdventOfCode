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

fn create_graph() -> ((usize, usize), (usize, usize), Vec<Vec<i32>>, Vec<Vec<char>>) {
    let mut starting_position: (usize, usize) = (0, 0);
    let mut finish_position: (usize, usize) = (0, 0);
    let topology_map: Vec<Vec<char>> = include_str!("../../input/day12/input.txt").lines().enumerate().map(|(i, res)| {
        res.trim().chars().enumerate().map(|(j,item)| {
            if item == 'S' {
                starting_position = (i, j);
                'a'
            } else if item == 'E' {
                finish_position = (i, j);
                'z'
            } else { item }
        }).collect()
    }).collect();
    // 1. and 2. To create an unvisited set with distances equal to infinity and 0 for the initial node
    let mut unvisited_set: Vec<Vec<i32>> = vec![vec![i32::MAX; topology_map[0].len()]; topology_map.len()];
    unvisited_set[starting_position.0][starting_position.1] = 0;
    (starting_position, finish_position, unvisited_set, topology_map)
}

// The steps are from here: https://en.wikipedia.org/wiki/Dijkstra%27s_algorithm#Algorithm
fn dijkstra() -> i32 {
    let (mut current, finish_position, mut unvisited, mut map) = create_graph();
    while current != finish_position {
        let current_distance: i32 = unvisited[current.0][current.1];
        // 3. Need to go through available neighbors to set their distances if smaller than already set
        for (y, x) in available_list(current, &map) {
            if current_distance + 1 > unvisited[y][x] { continue }
            unvisited[y][x] = current_distance + 1;   // All nodes have the length 1
        }
        // 4. Need to remove the current node from the unvisited set
        map[current.0][current.1] = 'X';

        // 6. Set the current node as the smallest unvisited
        let mut smallest = i32::MAX;
        for (y, line) in unvisited.iter().enumerate() {
            for (x, item) in line.iter().enumerate() {
                if map[y][x] != 'X' && smallest > *item {
                    smallest = *item;
                    current = (y, x);
                }
            }
        }
        // Functional way to find the smallest but way less readable
        // current = unvisited.iter().enumerate().fold(finish_position, |acc, (y, line)| {
        //     line.iter().enumerate().fold(acc, |smallest_pos, (x, item)| {
        //         if map[y][x] != 'X' && unvisited[smallest_pos.0][smallest_pos.1] > *item { (y, x) } else { smallest_pos }
        //     })
        // });
    }
    for y in map {
        for x in y {
            print!("{x}");
        }
        println!();
    }
    unvisited[finish_position.0][finish_position.1]
}

pub fn solution() {
    println!("It's getting tough a bit, oof");
    println!("The shortest path has length of: {}", dijkstra());
}