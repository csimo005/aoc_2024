use std::fs::read_to_string;
use std::collections::HashMap;

fn main() {
    let (antennas, dim) = parse_input("data/input_08.txt");

    let mut map: Vec<i32> = vec![0; (dim.0 * dim.1).try_into().unwrap()];
    for (_key, val) in antennas.iter() {
        for i in 0..val.len() {
            for j in (i+1)..val.len() {
                let an = calc_antinode(val[i], val[j]);
                if in_bounds(an, dim) {
                    map[(an.0 * dim.1 + an.1) as usize] = 1;
                }

                let an = calc_antinode(val[j], val[i]);
                if in_bounds(an, dim) {
                    map[(an.0 * dim.1 + an.1) as usize] = 1;
                }
            }
        }
    }

    let mut total: i32 = 0;
    for i in 0..map.len() {
        total += map[i];
    }
    println!("Part 1: {:?}", total);
   
    let mut map: Vec<i32> = vec![0; (dim.0 * dim.1).try_into().unwrap()];
    for (_key, val) in antennas.iter() {
        for i in 0..val.len() {
            for j in (i+1)..val.len() {
                let ans = calc_antinodes(val[i], val[j], dim);
                for an in ans {
                    map[(an.0 * dim.1 + an.1) as usize] = 1;
                }

                let ans = calc_antinodes(val[j], val[i], dim);
                for an in ans {
                    map[(an.0 * dim.1 + an.1) as usize] = 1;
                }

                map[(val[i].0 * dim.1 + val[i].1) as usize] = 1;
                map[(val[j].0 * dim.1 + val[j].1) as usize] = 1;
            }
        }
    }

    let mut total: i32 = 0;
    for i in 0..map.len() {
        total += map[i];
    }
    println!("Part 2: {:?}", total);
}

fn parse_input(fname: &str) -> (HashMap<char, Vec<(i32, i32)>>, (i32, i32)) {
    let mut rows: i32 = 0;
    let mut cols: i32 = 0;
    
    for line in read_to_string(fname).unwrap().lines() {
        rows += 1;
        cols = line.len() as i32;
    }
    let dim = (rows, cols);

    let mut antennas: HashMap<char, Vec<(i32, i32)>> = HashMap::new();
    for (i, line) in read_to_string(fname).unwrap().lines().enumerate() {
        for (j, c) in line.chars().enumerate() {
            if c != '.' {
                if antennas.contains_key(&c) {
                    antennas.get_mut(&c).unwrap().push((i as i32, j as i32));
                } else {
                    antennas.insert(c, vec![(i as i32, j as i32)]);
                }
            }
        }
    }

    return (antennas, dim)
}

fn calc_antinode(n1: (i32, i32), n2: (i32, i32)) -> (i32, i32) {
    (2 * n2.0 - n1.0, 2 * n2.1 - n1.1)
}

fn calc_antinodes(n1: (i32, i32), n2: (i32, i32), dim: (i32, i32)) -> Vec<(i32, i32)> {
    let mut ans: Vec<(i32, i32)> = Vec::<(i32, i32)>::new();
    let mut i: i32 = 1;

    while in_bounds(((i + 1) * n2.0 - i * n1.0, (i + 1) * n2.1 - i * n1.1), dim) {
        ans.push(((i + 1) * n2.0 - i * n1.0, (i + 1) * n2.1 - i * n1.1));
        i += 1;
    }

    return ans;
}

fn in_bounds(p: (i32, i32), dim: (i32, i32)) -> bool {
    p.0 >= 0 && p.0 < dim.0 && p.1 >= 0 && p.1 < dim.1
}
