use std::fs;

fn main() {
    let (mut l1, mut l2) = parse_input("./data/input_day1.txt");
    l1.sort();
    l2.sort();

    let mut d1: i32 = 0;
    for i in 0..l1.len() {
        d1 += (l1[i] - l2[i]).abs();
    }

    println!("Part 1: {:?}", d1);
    
    let (l1, l2) = parse_input("./data/input_day1.txt");
    let mut d2: i32 = 0;
    for i in 0..l1.len() {
        for j in 0..l2.len() {
            if l1[i] == l2[j] {
                d2 += l1[i];
            }
        }
    }

    println!("Part 2: {:?}", d2);
}

fn parse_input(fname: &str) -> (Vec<i32>, Vec<i32>) {
    let mut l1: Vec<i32> = Vec::<i32>::new();
    let mut l2: Vec<i32> = Vec::<i32>::new();

    for line in fs::read_to_string(fname).unwrap().lines() {
        let mut fields = line.split_whitespace();
        l1.push(fields.next().unwrap().parse::<i32>().unwrap());
        l2.push(fields.next().unwrap().parse::<i32>().unwrap());
    }

    return (l1, l2)
}
