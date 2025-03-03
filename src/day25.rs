use std::fs::read_to_string;
use regex::Regex;

#[derive(Debug)]
struct Lock {
    pins: Vec<i32>
}

impl Lock {
    fn new() -> Self {
        Self { pins: vec![6; 5] }
    }

    fn can_open(&self, k: &Key) -> bool {
        for i in 0..5 {
            if self.pins[i] + k.pins[i] > 5 {
                return false
            }
        }

        return true;
    }
}

#[derive(Debug)]
struct Key {
    pins: Vec<i32>
}

impl Key {
    fn new() -> Self {
        Self { pins: vec![5; 5] }
    }
}

fn main() {
    let (locks, keys) = parse_input("data/input_25.txt");
    
    let mut total: usize = 0;
    for lock in locks.iter() {
        for key in keys.iter() {
            if lock.can_open(&key) {
                total += 1;
            }
        }
    }
    println!("Result: {:?}", total);
}

fn parse_input(fname: &str) -> (Vec<Lock>, Vec<Key>) {
    let mut locks = Vec::<Lock>::new();
    let mut keys = Vec::<Key>::new();

    let mut is_ready: bool = true;
    let mut is_key: bool = true;
    let start_key = Regex::new(r"\.\.\.\.\.").unwrap();
    let start_lock = Regex::new(r"#####").unwrap();
    for line in read_to_string(fname).unwrap().lines() {
        if is_ready {
            if start_key.is_match(line) {
                is_key = true;
                is_ready = false;
                keys.push(Key::new());
            } else if start_lock.is_match(line) {
                is_key = false;
                is_ready = false;
                locks.push(Lock::new());
            }
        } else {
            if line != "" {
                for (i, c) in line.chars().enumerate() {
                    if is_key {
                        if c == '.' {
                            let k = keys.len();
                            keys[k - 1].pins[i] -= 1;
                        }
                    } else {
                        if c == '.' {
                            let l = locks.len();
                            locks[l - 1].pins[i] -= 1;
                        }
                    }
                }
            } else {
                is_ready = true;
            }
        }
    }

    return (locks, keys);
}
