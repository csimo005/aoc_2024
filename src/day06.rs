use std::fs::read_to_string;

fn main() {
    let (map, dims, start) = parse_input("data/input_06.txt");

    let visited = simulate_path(&map, dims, start);
    let mut total = 0;
    for i in 0..visited.len() {
        if visited[i] != 0 {
            total += 1;
        }
    }
    println!("Part 1: {:?}", total);

    total = 0;
    for i in 0..dims.0 {
        for j in 0..dims.1 {
            if visited[i * dims.1 + j] != 0 {
                let mut new_map = map.clone();
                new_map[i * dims.1 + j] = 1;

                if detect_loop(&new_map, dims, start) {
                    total += 1;
                }
            }
        }
    }
    println!("Part 2: {:?}", total);
}

fn parse_input(fname: &str) -> (Vec::<i32>, (usize, usize), (usize, usize)) {
    let mut rows: usize = 0;
    let mut cols: usize = 0;
    
    for line in read_to_string(fname).unwrap().lines() {
        rows += 1;
        cols = line.len();
    }
    let dims = (rows, cols);

    let mut map: Vec<i32> = vec![0; rows * cols];
    let mut pos: (usize, usize) = (0, 0);
    for (i, line) in read_to_string(fname).unwrap().lines().enumerate() {
        for (j, c) in line.chars().enumerate() {
            if c == '#' {
                map[i * cols + j] = 1;
            } else if c == '^' {
                pos = (i, j);
            }
        }
    }

    return (map, dims, pos);
}

fn simulate_path(map: &Vec<i32>, dims: (usize, usize), start: (usize, usize)) -> Vec<i32> {
    let mut pos = start;
    let mut d: i32 = 1;
    let mut visited: Vec<i32> = vec![0; map.len()];

    while visited[pos.0 * dims.1 + pos.1] & d == 0 {
        visited[pos.0 * dims.1 + pos.1] += d;

        while step_in_bounds(pos, d, dims) && !collision_free(step(pos, d), &map, dims) {
            d = d * 2;
            if d == 16 {
                d = 1;
            }
        }

        if !step_in_bounds(pos, d, dims) {
            break;
        }

        pos = step(pos, d);
    }

    return visited;
}

fn detect_loop(map: &Vec<i32>, dims: (usize, usize), start: (usize, usize)) -> bool {
    let mut pos = start;
    let mut d: i32 = 1;
    let mut visited: Vec<i32> = vec![0; map.len()];

    while visited[pos.0 * dims.1 + pos.1] & d == 0 {
        visited[pos.0 * dims.1 + pos.1] += d;

        while step_in_bounds(pos, d, dims) && !collision_free(step(pos, d), &map, dims) {
            d = d * 2;
            if d == 16 {
                d = 1;
            }
        }

        if !step_in_bounds(pos, d, dims) {
            break;
        }

        pos = step(pos, d);
    }

    step_in_bounds(pos, d, dims) && visited[pos.0 * dims.1 + pos.1] & d != 0
}

fn step_in_bounds(pos: (usize, usize), dir: i32, dims: (usize, usize)) -> bool {
    if dir == 1 {
        return pos.0 >= 1;
    } else if dir == 2 {
        return pos.1 + 1 < dims.1;
    } else if dir == 4 {
        return pos.0 + 1 < dims.0;
    } else {
        return pos.1 >= 1;
    }
}

fn step(pose: (usize, usize), dir: i32) -> (usize, usize) {
    if dir == 1 {
        return (pose.0 - 1, pose.1);
    } else if dir == 2 {
        return (pose.0, pose.1 + 1);
    } else if dir == 4 {
        return (pose.0 + 1, pose.1);
    } else {
        return (pose.0, pose.1 - 1);
    }

}

fn collision_free(pose: (usize, usize), map: &Vec<i32>, dims: (usize, usize)) -> bool {
    map[pose.0 * dims.1 + pose.1] == 0
}

fn visualize_path(map: &Vec<i32>, path: &Vec<i32>, dims: &(usize, usize)) {
    for i in 0..dims.0 {
        for j in 0..dims.1 {
            if map[i * dims.1 + j] == 1 {
                print!("{}", '#');
            } else if path[i * dims.1 + j] != 0 {
                print!("{}", 'X');
            } else {
                print!("{}", '.');
            }
        }
        print!("{}", '\n');
    }
    print!("{}", '\n');
}
