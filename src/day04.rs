use std::fs;

fn main() {
    let data = parse_input("./data/input_04.txt");
    let mut total = 0;

    for i in 0..data.len() {
        for j in 0..data[i].len() {
            if check_left(&data, i, j) {
                total += 1;
            }
            
            if check_right(&data, i, j) {
                total += 1;
            }
            
            if check_up(&data, i, j) {
                total += 1;
            }
            
            if check_down(&data, i, j) {
                total += 1;
            }
            
            if check_ul(&data, i, j) {
                total += 1;
            }
            
            if check_ur(&data, i, j) {
                total += 1;
            }
            
            if check_dl(&data, i, j) {
                total += 1;
            }
            
            if check_dr(&data, i, j) {
                total += 1;
            }
        }
    }

    println!("Part 1: {:?}", total);
    
    let mut total = 0;

    for i in 0..data.len() {
        for j in 0..data[i].len() {
            if check_xmas(&data, i, j) {
                total += 1;
            }
        }
    }
    
    println!("Part 2: {:?}", total);
}

fn parse_input(fname: &str) -> Vec<String> {
    let mut data: Vec<String> = Vec::<String>::new();

    for line in fs::read_to_string(fname).unwrap().lines() {
        data.push(line.to_string());
    }

    return data;
}

fn check_left(matrix: &Vec<String>, i: usize, j: usize) -> bool{
    if j < 3 {
        return false;
    }
    matrix[i].chars().nth(j).unwrap() == 'X' && matrix[i].chars().nth(j-1).unwrap() == 'M' && matrix[i].chars().nth(j-2).unwrap() == 'A' && matrix[i].chars().nth(j-3).unwrap() == 'S'
}

fn check_right(matrix: &Vec<String>, i: usize, j: usize) -> bool{
    if j+3 >= matrix[i].len() {
        return false;
    }
    matrix[i].chars().nth(j).unwrap() == 'X' && matrix[i].chars().nth(j+1).unwrap() == 'M' && matrix[i].chars().nth(j+2).unwrap() == 'A' && matrix[i].chars().nth(j+3).unwrap() == 'S'
}

fn check_up(matrix: &Vec<String>, i: usize, j: usize) -> bool{
    if i < 3 {
        return false;
    }
    matrix[i].chars().nth(j).unwrap() == 'X' && matrix[i-1].chars().nth(j).unwrap() == 'M' && matrix[i-2].chars().nth(j).unwrap() == 'A' && matrix[i-3].chars().nth(j).unwrap() == 'S'
}

fn check_down(matrix: &Vec<String>, i: usize, j: usize) -> bool{
    if i+3 >= matrix.len() {
        return false;
    }
    matrix[i].chars().nth(j).unwrap() == 'X' && matrix[i+1].chars().nth(j).unwrap() == 'M' && matrix[i+2].chars().nth(j).unwrap() == 'A' && matrix[i+3].chars().nth(j).unwrap() == 'S'
}

fn check_ul(matrix: &Vec<String>, i: usize, j: usize) -> bool{
    if j < 3 || i < 3{
        return false;
    }
    matrix[i].chars().nth(j).unwrap() == 'X' && matrix[i-1].chars().nth(j-1).unwrap() == 'M' && matrix[i-2].chars().nth(j-2).unwrap() == 'A' && matrix[i-3].chars().nth(j-3).unwrap() == 'S'
}

fn check_ur(matrix: &Vec<String>, i: usize, j: usize) -> bool{
    if j+3 >= matrix[i].len() || i < 3{
        return false;
    }
    matrix[i].chars().nth(j).unwrap() == 'X' && matrix[i-1].chars().nth(j+1).unwrap() == 'M' && matrix[i-2].chars().nth(j+2).unwrap() == 'A' && matrix[i-3].chars().nth(j+3).unwrap() == 'S'
}

fn check_dl(matrix: &Vec<String>, i: usize, j: usize) -> bool{
    if j < 3 || i+3 >= matrix.len(){
        return false;
    }
    matrix[i].chars().nth(j).unwrap() == 'X' && matrix[i+1].chars().nth(j-1).unwrap() == 'M' && matrix[i+2].chars().nth(j-2).unwrap() == 'A' && matrix[i+3].chars().nth(j-3).unwrap() == 'S'
}

fn check_dr(matrix: &Vec<String>, i: usize, j: usize) -> bool{
    if j+3 >= matrix[i].len() || i+3 >= matrix.len(){
        return false;
    }
    matrix[i].chars().nth(j).unwrap() == 'X' && matrix[i+1].chars().nth(j+1).unwrap() == 'M' && matrix[i+2].chars().nth(j+2).unwrap() == 'A' && matrix[i+3].chars().nth(j+3).unwrap() == 'S'
}

fn check_xmas(matrix: &Vec<String>, i: usize, j: usize) -> bool {
    if i < 1 || j < 1 || j+1 >= matrix[i].len() || i+1 >= matrix.len(){
        return false;
    }

    let mut valid = matrix[i].chars().nth(j).unwrap() == 'A';
    valid = valid && ((matrix[i-1].chars().nth(j-1).unwrap() == 'M' && matrix[i+1].chars().nth(j+1).unwrap() == 'S') || (matrix[i-1].chars().nth(j-1).unwrap() == 'S' && matrix[i+1].chars().nth(j+1).unwrap() == 'M'));
    valid && ((matrix[i-1].chars().nth(j+1).unwrap() == 'M' && matrix[i+1].chars().nth(j-1).unwrap() == 'S') || (matrix[i-1].chars().nth(j+1).unwrap() == 'S' && matrix[i+1].chars().nth(j-1).unwrap() == 'M'))
}
