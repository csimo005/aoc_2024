use std::fs::read_to_string;

struct Matrix {
    rows: usize,
    cols: usize,
    data: Vec<i32>
}

impl Matrix {
    pub fn new(rows: usize, cols: usize) -> Self {
            Matrix { rows: rows, cols: cols, data: vec![0; rows * cols] }
    }

    pub fn get(&self, r: usize, c: usize) -> Option<&i32> {
        if self.in_bounds(r, c) {
            Some(&self.data[r * self.cols + c])
        } else {
            None
        }
    }
    
    pub fn get_mut(&mut self, r: usize, c: usize) -> Option<&mut i32> {
        if self.in_bounds(r, c) {
            Some(&mut self.data[r * self.cols + c])
        } else {
            None
        }
    }

    pub fn in_bounds(&self, r: usize, c: usize) -> bool {
        r < self.rows && c < self.cols
    }

    pub fn get_neighbors(&self, r: usize, c: usize) -> Vec<(usize, usize)> {
        let mut n: Vec<(usize, usize)> = Vec::<(usize, usize)>::new();

        if r > 0 {
            n.push((r-1, c));
        }
        if r+1 < self.rows {
            n.push((r+1, c));
        }
        if c > 0 {
            n.push((r, c-1));
        }
        if c+1 < self.cols {
            n.push((r, c+1));
        }

        return n;
    }
}

fn main() {
    let data = parse_input("data/input_12.txt");
    let mut visited = Matrix::new(data.rows, data.cols);
    let mut total = 0;

    for r in 0..data.rows {
        for c in 0..data.cols {
            if *visited.get(r, c).unwrap() == 0 {
                let (area, perimeter, _) = flood_fill(r, c, &data, &mut visited);
                let plant: char = char::from_u32(*data.get(r, c).unwrap() as u32 + 'A' as u32).unwrap();
                println!("A region of {} plants with price {:?} * {:?} = {:?}.", plant, area, perimeter, area * perimeter);
                total += area * perimeter;
            }
        }
    }
    println!("Part 1: {:?}", total);
    
    let mut visited = Matrix::new(data.rows, data.cols);
    let mut total = 0;

    for r in 0..data.rows {
        for c in 0..data.cols {
            if *visited.get(r, c).unwrap() == 0 {
                let (area, _, corners) = flood_fill(r, c, &data, &mut visited);
                let plant: char = char::from_u32(*data.get(r, c).unwrap() as u32 + 'A' as u32).unwrap();
                println!("A region of {} plants with price {:?} * {:?} = {:?}.", plant, area, corners, area * corners);
                total += area * corners;
            }
        }
    }
    println!("Part 1: {:?}", total);
}

fn parse_input(fname: &str) -> Matrix {
    let mut rows: usize = 0;
    let mut cols: usize = 0;

    for line in read_to_string(fname).unwrap().lines() {
        rows += 1;
        cols = line.len();
    }

    let mut data: Matrix = Matrix::new(rows, cols);
    for (r, line) in read_to_string(fname).unwrap().lines().enumerate() {
        for (c, val) in line.chars().enumerate() {
            *data.get_mut(r, c).unwrap() = (val as u32 - 'A' as u32) as i32;
        }
    }

    return data;
}

fn print_pretty(m: &Matrix) {
    for r in 0..m.rows {
        for c in 0..m.cols {
            print!("{}", char::from_u32(*m.get(r, c).unwrap() as u32 + 'A' as u32).unwrap());
        }
        print!("\n");
    }
}

fn flood_fill(r: usize, c: usize, m: &Matrix, v: &mut Matrix) -> (u32, u32, u32) {
    let mut area: u32 = 0;
    let mut perimeter: u32 = 0;
    let mut corners: u32 = 0;

    let mut q: VecDeque<(usize, usize)> = VecDeque::<(usize, usize)>::new();
    q.push_front((r, c));

    while q.len() > 0 {
        let p = q.pop_back().unwrap();

        if *v.get(p.0, p.1).unwrap() == 0 {
            *v.get_mut(p.0, p.1).unwrap() = 1;
            area += 1;
            perimeter += 4;
            corners += count_corners(p, m);
   
            for n in m.get_neighbors(p.0, p.1) {
                if *m.get(r, c).unwrap() == *m.get(n.0, n.1).unwrap() {
                    if *v.get(n.0, n.1).unwrap() == 0 {
                        q.push_front((n.0, n.1));
                    } else {
                        perimeter -= 2;
                    }
                }
            }
        }
    }

    return (area, perimeter, corners);
}

fn count_corners(p: (usize, usize), m: &Matrix) -> u32 {
    let mut corners: u32 = 0;
    let k = *m.get(p.0, p.1).unwrap();
    let dirs: Vec<(i32, i32)> = vec![(-1, -1), (-1, 1), (1, -1), (1, 1)];
   
    for d in dirs {
        let mut n: (usize, usize) = (0, 0);
        if (p.0 as i32) + d.0 < 0 {
            n.0 = m.rows;
        } else {
            n.0 = ((p.0 as i32) + d.0) as usize;
        }
        if (p.1 as i32) + d.1 < 0 {
            n.1 = m.cols;
        } else {
            n.1 = ((p.1 as i32) + d.1) as usize;
        }

        if m.in_bounds(n.0, n.1) {
            if *m.get(n.0, p.1).unwrap() == k && *m.get(p.0, n.1).unwrap() == k && *m.get(n.0, n.1).unwrap() != k {
                corners += 1;
            } else if *m.get(n.0, p.1).unwrap() != k && *m.get(p.0, n.1).unwrap() != k {
                corners += 1;
            }
        } else if m.in_bounds(n.0, p.1) {
            if *m.get(n.0, p.1).unwrap() != k {
                corners += 1;
            }
        } else if m.in_bounds(p.0, n.1) {
            if *m.get(p.0, n.1).unwrap() != k {
                corners += 1; 
            }

        } else {
            corners += 1;
        }
    }

    return corners;
}
