use std::time::Instant;
use std::fs::read_to_string;
use std::collections::VecDeque;

fn main() {
    let (map, dim) = parse_input("data/input_10.txt");
    
    let before = Instant::now();
    let mut total: i32 = 0;
    for i in 0..map.len() {
        if map[i] == 0 {
            total += score_trailhead(i, &map, dim);
        }
    }
    println!("Part 1: {:?}, Time: {:?}", total, before.elapsed());
    
    let before = Instant::now();
    let mut total: i32 = 0;
    for i in 0..map.len() {
        if map[i] == 0 {
            total += count_paths(i, &map, dim);
        }
    }
    println!("Part 2: {:?}, Time: {:?}", total, before.elapsed());
}

fn parse_input(fname: &str) -> (Vec<i32>, (usize, usize)) {
    let mut rows: usize = 0;
    let mut cols: usize = 0;
    
    for line in read_to_string(fname).unwrap().lines() {
        rows += 1;
        cols = line.len();
    }
    let dim = (rows, cols);

    let mut map: Vec<i32> = vec![0; (dim.0 * dim.1) as usize];
    for (i, line) in read_to_string(fname).unwrap().lines().enumerate() {
        for (j, c) in line.chars().enumerate() {
            map[i * (dim.1 as usize) + j] = c.to_string().parse::<i32>().unwrap();
        }
    }

    return (map, dim)
}

fn score_trailhead(p: usize, map: &Vec<i32>, dim: (usize, usize)) -> i32 {
    let mut score = 0;
    let mut visited: Vec<i32> = vec![0; map.len()];


    let mut q: VecDeque<usize> = VecDeque::<usize>::new();
    q.push_front(p);

    while q.len() > 0 {
        let curr = q.pop_back().unwrap();
        for n in neighbors(curr, dim) {
            if map[n] - map[curr] == 1 && visited[n] == 0 {
                visited[n] = 1;
                if map[n] == 9 {
                    score += 1;
                } else {
                    q.push_front(n);
                }
            }
        }
    }

    return score;
}

fn count_paths(src: usize, map: &Vec<i32>, dim: (usize, usize)) -> i32 {
    let mut paths: i32 = 0;

    let mut q: VecDeque<Vec<usize>> = VecDeque::<Vec<usize>>::new();
    q.push_front(vec![src]);

    while q.len() > 0 {
        let curr = q.pop_back().unwrap();
        for n in neighbors(curr[curr.len() - 1], dim) {
            if map[n] == map[curr[curr.len() - 1]] + 1 {
                if map[n] == 9 {
                    paths += 1;
                } else {
                    let mut next = curr.clone();
                    next.push(n);
                    q.push_front(next);
                }
            }
        }
    }
     
    return paths;
}

fn neighbors(ind: usize, dim: (usize, usize)) -> Vec<usize> {
    let mut n: Vec<usize> = Vec::<usize>::new();

    if ind + dim.1 < dim.0 * dim.1 {
        n.push(ind + dim.1);
    }
    
    if ind >= dim.1 {
        n.push(ind - dim.1);
    }

    if (ind % dim.1) + 1 < dim.1 {
        n.push(ind + 1);
    }
    
    if (ind % dim.1) > 0 {
        n.push(ind - 1);
    }

    return n;
}

fn print_trails(map: &Vec<i32>, visited: &Vec<i32>, dim: (usize, usize)) {
    for i in 0..map.len() {
        if visited[i] == 1 {
            print!("{:?}", map[i]);
        } else {
            print!(".");
        }

        if (i + 1) % dim.1 == 0 {
            print!("\n");
        }
    }

    for _i in 0..dim.1 {
        print!("-");
    }
    print!("\n");
}
