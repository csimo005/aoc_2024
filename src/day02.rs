use std::fs;

fn main() {
    let data = parse_input("./data/input_02.txt");

    let mut total: i32 = 0;
    for report in data.iter() {
        if is_safe(report) {
            total += 1;
        }
    }
    println!("Part 1: {:?}", total);
    
    let data = parse_input("./data/input_02.txt");

    let mut total: i32 = 0;
    for report in data.iter() {
        if is_safe(report) {
            total += 1;
            continue;
        }
        for i in 0..report.len() {
            let mut r: Vec<i32> = report.to_vec();
            r.remove(i);
            if is_safe(&r) {
                total += 1;
                break;
            }
        }
    }
    println!("Part 2: {:?}", total);
}

fn parse_input(fname: &str) -> Vec<Vec<i32>> {
    let mut data: Vec<Vec<i32>> = Vec::<Vec::<i32>>::new();

    for line in fs::read_to_string(fname).unwrap().lines() {
        data.push(Vec::<i32>::new());
        let i = data.len() - 1;
        for field in line.split_whitespace() {
            data[i].push(field.parse::<i32>().unwrap());
        }
    }

    return data;
}

fn is_safe(v: &Vec<i32>) -> bool {
    let sign: i32 = (v[1] - v[0]).signum();
    for i in 1..v.len() {
        let diff: i32 = v[i] - v[i-1];
        if diff.signum() != sign || diff.abs() < 1 || diff.abs() > 3 {
            return false;
        }
    }

    return true;
}
