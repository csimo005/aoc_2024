use std::fs::read_to_string;
use itertools::Itertools;

fn main() {
    let (rules, updates) = parse_input("data/input_05.txt");

    let mut total: usize = 0;
    for update in updates.iter() {
        if valid_update(&update, &rules) {
            total += update[update.len() / 2];
        }
    }
    println!("Part 1: {:?}", total);
    
    total = 0;

    for update in updates.iter() {
        if !valid_update(&update, &rules) {
            let ordering = fix_update(&update, &rules);
            total += update[ordering[update.len() / 2]];
        }
    }
    println!("Part 2: {:?}", total);
}

fn parse_input(fname: &str) -> (Vec::<i32>, Vec<Vec::<usize>>) {
    let mut rules: Vec<i32> = vec![0; 10000];
    let mut updates: Vec<Vec<usize>> = Vec::new();
    let mut reading_rules: bool = true;
    
    for line in read_to_string(fname).unwrap().lines() {
        if reading_rules {
            if line == "" {
                reading_rules = false;
            } else {
                let (i, j) = line.split("|").map(|c| c.parse::<usize>().unwrap()).collect_tuple().unwrap();
                rules[100 * i + j] =  1;
                rules[100 * j + i] = -1;
            }
        } else {
            updates.push(line.split(",").map(|c| c.parse::<usize>().unwrap()).collect::<Vec<usize>>()); 
        }
    }

    (rules, updates)
}

fn valid_update(update: &Vec<usize>, rules: &Vec::<i32>) -> bool {
    for i in 0..update.len() {
        for j in (i+1)..update.len() {
            if rules[update[j] * 100 + update[i]] == 1 {
                return false;
            }
        }
    }

    return true;
}

fn fix_update(update: &Vec<usize>, rules: &Vec::<i32>) -> Vec<usize> {
    let condensed = condense_rules(update, rules);
    let mut index: Vec<usize> = vec![0; update.len()];
    for i in 0..update.len() {
        index[i] = i;
    }
    
    for i in 0..update.len() {
        for j in 0..update.len() {
            if condensed[index[i] * update.len() + index[j]] != -1 {
                index.swap(i, j);
            }
        }
    }

    return index;
}

fn condense_rules(update: &Vec<usize>, rules: &Vec::<i32>) -> Vec<i32> {
    let N = update.len();
    let mut new_rules: Vec::<i32> = vec![0; N*N];
    for i in 0..N {
        for j in 0..N {
            new_rules[i * N + j] = rules[update[i] * 100 + update[j]];
        }
    }

    for _ in 0..N {
        for i in 0..N {
            for j in 0..N {
                for k in 0..N {
                    if new_rules[k * N + i] == 0 { 
                        if new_rules[j * N + i] ==  1 && new_rules[k * N + j] ==  1 { // j < i && k < j -> k < i
                            new_rules[k * N + i] =  1;
                            new_rules[i * N + k] = -1;
                        }
                        
                        if new_rules[j * N + i] == -1 && new_rules[k * N + j] == -1 { // j > i && k > j -> k > i
                            new_rules[k * N + i] = -1;
                            new_rules[i * N + k] =  1;
                        }
//                        
//                        if new_rules[j * 100 + i] == -1 && new_rules[j * 100 + k] ==  1 {
//                            new_rules[i * 100 + k] =  1;
//                            new_rules[k * 100 + i] = -1;
//                        }
//                        
//                        if new_rules[j * 100 + i] ==  1 && new_rules[j * 100 + k] == -1 {
//                            new_rules[i * 100 + k] = -1;
//                            new_rules[k * 100 + i] =  1;
//                        }
                    }
                }
            }
        }
    }

    return new_rules;
}
