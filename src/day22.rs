use std::fs::read_to_string;
use std::collections::HashMap;
use std::cmp::max;

fn main() {
    let nums = parse_input("data/input_22.txt");

    let mut total: i64 = 0;
    for n in nums.iter() {
        let mut s = *n;
        for _ in 0..2000 {
            s = step(s);
        }
        total += s;
    }
    println!("Total: {:?}", total);

    let mut map: HashMap<Vec<i64>, Vec<i64>> = HashMap::<Vec<i64>, Vec<i64>>::new();
    for (i, n) in nums.iter().enumerate() {
        let mut deltas: Vec<i64> = Vec::<i64>::new();
        
        let mut s = *n;
        for j in 0..2000 {
            //deltas.insert(0, (step(s) % 10) - (s % 10));
            deltas.push((step(s) % 10) - (s % 10));
            if deltas.len() == 5 {
                deltas.remove(0);
            }

            s = step(s);
            if deltas.len() == 4 {
                if let Some(v) = map.get_mut(&deltas) {
                    if v[i] == -1 {
                        v[i] = s % 10;
                    }
                } else {
                    map.insert(deltas.clone(), vec![-1; nums.len()]);
                    let  v: &mut Vec<i64> = map.get_mut(&deltas).unwrap();
                    v[i] = s % 10;
                }
            }
        }
    }
    
    let mut best_score: i64 = 0;
    for (key, val) in map.iter() {
        let mut total: i64 = 0;
        for v in val.iter() {
            if *v != -1 {
                total += v;
            }
        }
        best_score = max(total, best_score);
    }
    println!("Best Score: {:?}", best_score);
}

fn parse_input(fname: &str) -> Vec<i64> {
    let mut data: Vec<i64> = Vec::<i64>::new();
    for line in read_to_string(fname).unwrap().lines() {
        data.push(line.parse::<i64>().unwrap());
    }

    return data
}

fn step(secret: i64) -> i64 {
    let mut result: i64 = (secret ^ (secret << 6)) % 16777216_i64;
    result = (result ^ (result >> 5)) % 16777216_i64;
    result = (result ^ (result << 11 )) % 16777216_i64;

    return result;
}
