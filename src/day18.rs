use std::fs::read_to_string;
use std::collections::BinaryHeap;

#[derive(Debug, Eq, Ord, PartialEq, PartialOrd)]
struct MatInd {
    row: usize,
    col: usize,
}

#[derive(Debug)]
struct Matrix<T> {
    rows: usize,
    cols: usize,
    data: Vec<T>,
}

impl<T: std::clone::Clone> Matrix<T> {
    fn new(rows: usize, cols: usize, fill: T) -> Matrix<T> {
        Matrix{rows, cols, data: vec![fill; rows * cols]}
    }

    fn get(&self, ind: &MatInd) -> &T {
        &self.data[ind.row * self.cols + ind.col]
    }

    fn get_mut(&mut self, ind: &MatInd) -> &mut T {
        &mut self.data[ind.row * self.cols + ind.col]
    }
}

impl Matrix<i32> {
    fn print(&self) {
        for r in 0..self.rows {
            for c in 0..self.cols {
                match self.data[r * self.cols + c] {
                    0 => print!("."),
                    1 => print!("#"),
                    _ => print!("X"),
                };
            }
            println!("");
        }
    }
}

fn main() {
    let inds = parse_input("data/input_18.txt");
    let mut m = Matrix::<i32>::new(71, 71, 0);

    for i in 0..1024 {
        *m.get_mut(&inds[i]) = 1;
    }

    let costs: Matrix<i32> = path_length(&m);
    println!("Path Length: {:?}", -1 * (*costs.get(&MatInd{row: m.rows-1, col: m.cols-1})));
    
    let mut m = Matrix::<i32>::new(71, 71, 0);
    let mut costs: Matrix<i32> = path_length(&m);
    let mut i: usize = 0;

    while *costs.get(&MatInd{row: m.rows-1, col: m.cols-1}) != 0 {
        *m.get_mut(&inds[i]) = 1;
        i += 1;
        costs = path_length(&m);
    }

    println!("Failure After: {:?},{:?}", inds[i-1].col, inds[i-1].row);

}

fn parse_input(fname: &str) -> Vec<MatInd> {
    let mut data: Vec<MatInd> = Vec::<MatInd>::new();

    for line in read_to_string(fname).unwrap().lines() {
        let fields: Vec<&str> = line.split(",").collect();
        let col = fields[0].parse::<usize>().unwrap();
        let row = fields[1].parse::<usize>().unwrap();

        data.push(MatInd{row, col});
    }

    return data;
}

fn path_length(occup: &Matrix<i32>) -> Matrix<i32> {
    let mut cost: Matrix<i32> = Matrix::<i32>::new(occup.rows, occup.cols, 0);
    let mut v: Matrix<bool> = Matrix::<bool>::new(occup.rows, occup.cols, false);
    let mut h: BinaryHeap<(i32, MatInd)> = BinaryHeap::<(i32, MatInd)>::new();
    h.push((0, MatInd{row: 0, col: 0}));

    while let Some((c, ind)) = h.pop() {
        if *v.get(&ind) == false{
            *v.get_mut(&ind) = true;
            *cost.get_mut(&ind) = c;

            if ind.col > 0 {
                let next: MatInd = MatInd{row: ind.row, col: ind.col-1};
                if *occup.get(&next) == 0 {
                    h.push((c-1, next));
                }
            }
            if ind.col+1 < occup.cols {
                let next: MatInd = MatInd{row: ind.row, col: ind.col+1};
                if *occup.get(&next) == 0 {
                    h.push((c-1, next));
                }
            }
            if ind.row > 0 {
                let next: MatInd = MatInd{row: ind.row-1, col: ind.col};
                if *occup.get(&next) == 0 {
                    h.push((c-1, next));
                }
            }
            if ind.row+1 < occup.rows {
                let next: MatInd = MatInd{row: ind.row+1, col: ind.col};
                if *occup.get(&next) == 0 {
                    h.push((c-1, next));
                }
            }
        }
    }
    return cost;
}
