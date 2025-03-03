use std::fs::read_to_string;
use regex::Regex;

#[derive(Debug, Clone)]
struct Matrix {
    rows: usize,
    cols: usize,
    data: Vec<f64>
}

impl Matrix {
    pub fn new(rows: usize, cols: usize) -> Self {
            Matrix { rows: rows, cols: cols, data: vec![0.0; rows * cols] }
    }

    pub fn get(&self, r: usize, c: usize) -> Option<&f64> {
        if self.in_bounds(r, c) {
            Some(&self.data[r * self.cols + c])
        } else {
            None
        }
    }
    
    pub fn get_mut(&mut self, r: usize, c: usize) -> Option<&mut f64> {
        if self.in_bounds(r, c) {
            Some(&mut self.data[r * self.cols + c])
        } else {
            None
        }
    }

    pub fn in_bounds(&self, r: usize, c: usize) -> bool {
        r < self.rows && c < self.cols
    }

    pub fn det(&self) -> f64 {
        assert!(self.rows == 2);
        assert!(self.cols == 2);

        (self.data[0] * self.data[3]) - (self.data[1] * self.data[2])
    }

    pub fn inv(&self) -> Matrix {
        assert!(self.rows == 2);
        assert!(self.cols == 2);

        let mut inverted: Matrix = Matrix::new(2, 2);
        let d = self.det();

        inverted.data[0] = self.data[3] / d;
        inverted.data[1] = -1.0 * (self.data[1] / d);
        inverted.data[2] = -1.0 * (self.data[2] / d);
        inverted.data[3] = self.data[0] / d;

        return inverted;
    }
}

impl std::ops::Mul<Matrix> for Matrix {
    type Output = Matrix;

    fn mul(self, rhs: Matrix) -> Matrix {
        assert!(self.cols == rhs.rows);
        let mut res: Matrix = Matrix::new(self.rows, rhs.cols);

        for i in 0..self.rows {
            for j in 0..rhs.cols {
                for k in 0..self.cols {
                    *res.get_mut(i, j).unwrap() += *self.get(i, k).unwrap() * rhs.get(k, j).unwrap();
                }
            }
        }

        return res;
    }
}

impl std::ops::Mul<&Matrix> for Matrix {
    type Output = Matrix;

    fn mul(self, rhs: &Matrix) -> Matrix {
        assert!(self.cols == rhs.rows);
        let mut res: Matrix = Matrix::new(self.rows, rhs.cols);

        for i in 0..self.rows {
            for j in 0..rhs.cols {
                for k in 0..self.cols {
                    *res.get_mut(i, j).unwrap() += *self.get(i, k).unwrap() * rhs.get(k, j).unwrap();
                }
            }
        }

        return res;
    }
}

impl std::ops::Add<Matrix> for Matrix {
    type Output = Matrix;

    fn add(self, rhs: Matrix) -> Matrix {
        assert!(self.rows == rhs.rows);
        assert!(self.cols == rhs.cols);
        let mut res: Matrix = Matrix::new(self.rows, rhs.cols);
        for i in 0..res.data.len() {
            res.data[i] = self.data[i] + res.data[i];
        }

        return res;
    }
}

impl std::ops::Add<&Matrix> for Matrix {
    type Output = Matrix;

    fn add(self, rhs: &Matrix) -> Matrix {
        assert!(self.rows == rhs.rows);
        assert!(self.cols == rhs.cols);
        let mut res: Matrix = Matrix::new(self.rows, rhs.cols);
        for i in 0..res.data.len() {
            res.data[i] = self.data[i] + res.data[i];
        }

        return res;
    }
}

/*impl std::ops::Add<Matrix> for &Matrix {
    type Output = Matrix;

    fn add(&self, rhs: Matrix) -> Matrix {
        assert!(self.rows == rhs.rows);
        assert!(self.cols == rhs.cols);
        let mut res: Matrix = Matrix::new(self.rows, rhs.cols);
        for i in 0..res.data.len() {
            res.data[i] = self.data[i] + res.data[i];
        }

        return res;
    }
}

impl std::ops::Add<&Matrix> for &Matrix {
    type Output = Matrix;

    fn add(&self, rhs: &Matrix) -> Matrix {
        assert!(self.rows == rhs.rows);
        assert!(self.cols == rhs.cols);
        let mut res: Matrix = Matrix::new(self.rows, rhs.cols);
        for i in 0..res.data.len() {
            res.data[i] = self.data[i] + res.data[i];
        }

        return res;
    }
}*/

fn main() {
    let mut data = parse_input("data/input_13.txt");

    let mut total = 0;
    for (i, (x, y)) in data.iter().enumerate() {
        let sol = x.inv() * y;

        let A: f64 = *sol.get(0, 0).unwrap();
        let B: f64 = *sol.get(1, 0).unwrap();

        if (A - A.round()).abs() > 0.01 {
            println!("Machine {:?}: Can't push A fractional amount ({:?})", i, A);
            continue;
        }
        
        if (B - B.round()).abs() > 0.01 {
            println!("Machine {:?}: Can't push B fractional amount ({:?})", i, B);
            continue;
        }

        println!("Machine {:?}: A: {:?}, B: {:?}, Cost: {:?}", i, A.round() as i32, B.round() as i32, (3.0 * A.round() + B.round()) as i32);
        total += 3 * (A.round() as i32) + B.round() as i32;
    }
    println!("Part 1: {:?}", total);
    
    let mut total = 0;
    for i in 0..data.len() {
        *data[i].1.get_mut(0, 0).unwrap() += 10000000000000.0;
        *data[i].1.get_mut(1, 0).unwrap() += 10000000000000.0;
        let sol = data[i].0.inv() * data[i].1.clone();
        let A: f64 = *sol.get(0, 0).unwrap();
        let B: f64 = *sol.get(1, 0).unwrap();

        if (A - A.round()).abs() > 0.01 {
            println!("Machine {:?}: Can't push A fractional amount ({:?})", i, A);
            continue;
        }
        
        if (B - B.round()).abs() > 0.01 {
            println!("Machine {:?}: Can't push B fractional amount ({:?})", i, B);
            continue;
        }

        println!("Machine {:?}: A: {:?}, B: {:?}, Cost: {:?}", i, A.round() as i64, B.round() as i64, (3.0 * A.round() + B.round()) as i64);
        total += 3 * (A.round() as i64) + B.round() as i64;
    }
    println!("Part 2: {:?}", total);
}

fn parse_input(fname: &str) -> Vec<(Matrix, Matrix)> {
    let mut data: Vec<(Matrix, Matrix)> = Vec::<(Matrix, Matrix)>::new();

    let c1 = Regex::new(r"^Button A: X\+([0-9]+), Y\+([0-9]+)").unwrap();
    let c2 = Regex::new(r"^Button B: X\+([0-9]+), Y\+([0-9]+)").unwrap();
    let c3 = Regex::new(r"^Prize: X=([0-9]+), Y=([0-9]+)").unwrap();

    data.push((Matrix::new(2, 2), Matrix::new(2, 1)));
    let mut n: usize = 0;
    for line in read_to_string(fname).unwrap().lines() {
        if let Some(caps) = c1.captures(line) {
            *data[n].0.get_mut(0, 0).unwrap() = caps[1].parse::<f64>().unwrap();
            *data[n].0.get_mut(1, 0).unwrap() = caps[2].parse::<f64>().unwrap();
        } else if let Some(caps) = c2.captures(line) {
            *data[n].0.get_mut(0, 1).unwrap() = caps[1].parse::<f64>().unwrap();
            *data[n].0.get_mut(1, 1).unwrap() = caps[2].parse::<f64>().unwrap();
        } else if let Some(caps) = c3.captures(line) {
            *data[n].1.get_mut(0, 0).unwrap() = caps[1].parse::<f64>().unwrap();
            *data[n].1.get_mut(1, 0).unwrap() = caps[2].parse::<f64>().unwrap();
        } else {
            data.push((Matrix::new(2, 2), Matrix::new(2, 1)));
            n += 1;
        }
    }

    return data;
}
