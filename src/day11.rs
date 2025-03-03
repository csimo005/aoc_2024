use std::time::Instant;
use std::fs::read_to_string;
use std::collections::HashMap;
use std::cmp::max;

fn main() {
    let mut stones: HashMap<i64, i64> = parse_input("data/input_11.txt");

    let before = Instant::now();
    for _ in 0..25 {
        stones = step(&stones);
    }
    println!("Part 1: {:?}, Time: {:?}", count_stones(&stones), before.elapsed());
    
    let mut stones: HashMap<i64, i64> = parse_input("data/input_11.txt");

    let before = Instant::now();
    for _ in 0..75 {
        stones = step(&stones);
    }
    println!("Part 2: {:?}, Time: {:?}", count_stones(&stones), before.elapsed());
}

fn parse_input(fname: &str) -> HashMap<i64, i64> {
    let mut data: HashMap<i64, i64> = HashMap::<i64, i64>::new();
    for line in read_to_string(fname).unwrap().lines() {
        for field in line.split_whitespace() {
            let key: i64 = field.parse::<i64>().unwrap();

            if let Some(x) = data.get_mut(&key) {
                *x = *x + 1;
            } else {
                data.insert(key, 1);
            }
        }
    }

    return data;
}

fn step(stones: &HashMap<i64, i64>) -> HashMap<i64, i64>{
    let mut new_stones: HashMap<i64, i64> = HashMap::<i64, i64>::new();
    for (key, val) in stones.iter(){
        let pow = ((*key as f32).log(10.0).floor() + 1.0)as u32;
        if *key == 0 {
            if let Some(x) = new_stones.get_mut(&1) {
                *x = *x + *val;
            } else {
                new_stones.insert(1, *val);
            }
        } else if pow % 2 == 0 {
            let k1 =  *key / 10_i64.pow(pow/2);
            if let Some(x) = new_stones.get_mut(&k1) {
                *x = *x + *val;
            } else {
                new_stones.insert(k1, *val);
            }

            let k2 = *key % 10_i64.pow(pow/2);
            if let Some(x) = new_stones.get_mut(&k2) {
                *x = *x + *val;
            } else {
                new_stones.insert(k2, *val);
            }
        } else {
            let k = *key * 2024;
            if let Some(x) = new_stones.get_mut(&k) {
                *x = *x + *val;
            } else {
                new_stones.insert(k, *val);
            }
        }
    }

    return new_stones;
}

fn count_stones(stones: &HashMap<i64, i64>) -> i64 {
    let mut total: i64 = 0;
    for (_key, val) in stones.iter() {
        total += *val;
    }

    return total;
}

fn print_stones(stones: &HashMap<i64, i64>) {
    let mut keys: Vec<i64> = Vec::<i64>::new();
    let mut max_pow_k: u32 = 0;
    let mut max_pow_v: u32 = 0;
    for k in stones.keys() {
        keys.push(*k);
        max_pow_k = max(max_pow_k, ((*k as f32).log(10.0).floor() + 1.0) as u32);
        max_pow_v = max(max_pow_v, ((*stones.get(k).unwrap() as f32).log(10.0).floor() + 1.0) as u32);
    }
    keys.sort();

    for k in keys.iter() {
        let mut pad = max_pow_k - ((*k as f32).log(10.0).floor() + 1.0) as u32;
        if *k == 0 {
            pad -= 1;
        }
        let p1 = (0..pad).map(|_| " ").collect::<String>();

        let pad = max_pow_v - ((*stones.get(k).unwrap() as f32).log(10.0).floor() + 1.0) as u32;
        let p2 = (0..pad).map(|_| " ").collect::<String>();

        println!("{}{:?}: {}{:?}", p1, k, p2, stones.get(k).unwrap());
    }
}
