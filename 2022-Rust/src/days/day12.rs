fn available_list(current_height: char, node_position: (i32, i32), map: &Vec<Vec<char>>) -> Vec<(usize, usize)> {
    let mut result: Vec<(usize, usize)> = Vec::new();
    println!("node: {:?} height: {}", node_position, current_height);
    for (y, x) in [(node_position.0 - 1, node_position.1), (node_position.0 + 1, node_position.1), (node_position.0, node_position.1 - 1), (node_position.0, node_position.1 + 1)] {
        // println!("{y} {}", y as usize);
        if  0 <= y && y < map.len() as i32 && 0 <= x && x < map[0].len() as i32 {
            if map[y as usize][x as usize] != 'X' && current_height as usize + 1 >= map[y as usize][x as usize] as usize {
                // Cia jau tas pasiekiamas gaunasi
                result.push((y as usize, x as usize))
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

    println!("topology: {:?} start: {:?} finish: {:?} unvisited: {:?}", topology_map, starting_position, finish_position, unvisited_set);
    (starting_position, finish_position, unvisited_set, topology_map)
}

// The steps are from here: https://en.wikipedia.org/wiki/Dijkstra%27s_algorithm#Algorithm
fn dijkstra() {
    let (mut current, finish_position, mut unvisited, mut map) = create_graph();
    while current != finish_position {
        let current_distance: i32 = unvisited[current.0][current.1];
        let current_height: char = map[current.0][current.1];
        // 4. Need to remove the current node from the unvisited set
        map[current.0][current.1] = 'X';
        // 3. Need to go through available neighbors to set their distances
        for (y, x) in available_list(current_height, (current.0 as i32, current.1 as i32), &map) {
            if current_distance + 1 > unvisited[y][x] { continue }
            unvisited[y][x] = current_distance + 1;   // All nodes have the length 1
        }

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
    }
    println!("So we found a path? {}", unvisited[finish_position.0][finish_position.1]);
    for y in map {
        for x in y {
            print!("{x}");
        }
        println!();
    }
}

pub fn solution() {
    println!("It's getting tough a bit, oof");
    dijkstra();
}