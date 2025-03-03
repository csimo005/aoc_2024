use std::fs;
use regex::Regex;
use itertools::Itertools;

fn main() {
    let data = parse_p1("./data/input_03.txt");

    let mut total: i32 = 0;
    for op in data {
        total += op.0 * op.1;
    }
    println!("Part 1: {:?}", total);
    
    let data = parse_p2("./data/input_03.txt");

    let mut total: i32 = 0;
    for op in data {
        total += op.0 * op.1;
    }
    println!("Part 2: {:?}", total);
}

fn parse_p1(fname: &str) -> Vec<(i32, i32)> {
    let re = Regex::new(r"mul\(([0-9]+),([0-9]+)\)").unwrap();
    let mut data: Vec<(i32, i32)> = Vec::<(i32, i32)>::new();

    for line in fs::read_to_string(fname).unwrap().lines() {
        for (_, [a, b]) in re.captures_iter(line).map(|c| c.extract()) {
            data.push((a.parse::<i32>().unwrap(), b.parse::<i32>().unwrap()));
        }
    }

    return data;
}

fn parse_p2(fname: &str) -> Vec<(i32, i32)> {
    let re1 = Regex::new(r"mul\([0-9]+,[0-9]+\)|don't\(\)|do\(\)").unwrap();
    let re2 = Regex::new(r"[0-9]+").unwrap();

    let mut enabled: bool = true;
    let mut data: Vec<(i32, i32)> = Vec::<(i32, i32)>::new();

    for line in fs::read_to_string(fname).unwrap().lines() {
        for m in re1.find_iter(line).map(|c| c.as_str()) {
            if &m[0..3] == "mul" {
                if enabled {
                    data.push(re2.find_iter(m).map(|c| c.as_str().parse::<i32>().unwrap()).collect_tuple().unwrap());
                }
            } else if &m[0..3] == "don" {
                enabled = false;
            } else if &m[0..3] == "do(" {
                enabled = true;
            } else {
                println!("{:?}", m);
            }
        }
    }

    return data;
}
