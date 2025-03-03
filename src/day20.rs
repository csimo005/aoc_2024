use std::fs::read_to_string;
use std::collections::VecDeque;

#[derive(Debug,Copy,Clone,PartialEq,Eq)]
struct Position {
    row: usize,
    col: usize,
}

#[derive(Debug,Copy,Clone,PartialEq,Eq)]
struct State {
    pos: Position,
    cost: usize,
}

#[derive(Debug,Clone)]
struct Track {
    rows: usize,
    cols: usize,
    occup: Vec<bool>,
    start: Position,
    end: Position,
}

impl Track {
    fn new(rows: usize, cols: usize) -> Self {
        Self {
            rows,
            cols,
            occup: vec![false; rows * cols],
            start: Position{row: 0, col: 0},
            end: Position{row: 0, col: 0},
        }
    }

    fn get_ind(&self, p: &Position) -> usize {
        p.row * self.cols + p.col
    }

    fn print(&self) {
        for r in 0..self.rows {
            for c in 0..self.cols {
                if self.occup[r * self.cols + c] {
                    print!("#");
                } else {
                    let p: Position = Position{row: r, col: c};
                    if p == self.start {
                        print!("S");
                    } else if p == self.end {
                        print!("E");
                    } else {
                        print!(".");
                    }
                }
            }
            println!("");
        }
    }

    fn neighbors(&self, p: &Position) -> Vec<Position> {
        let mut n: Vec<Position> = Vec::<Position>::new();

        if p.row > 0 {
            n.push(Position{row: p.row-1, col: p.col});
        }
        if p.col > 0 {
            n.push(Position{row: p.row, col: p.col-1});
        }
        if p.row < self.rows-1 {
            n.push(Position{row: p.row+1, col: p.col});
        }
        if p.col < self.cols-1 {
            n.push(Position{row: p.row, col: p.col+1});
        }

        return n;
    }

    fn extended_neighbors(&self, p: &Position, iter: usize) -> Vec<State> {
        let mut n: Vec<State> = Vec::<State>::new();
/*        let mut v: Vec<bool> = vec![false; self.occup.len()];
        let mut q: VecDeque<State> = VecDeque::<State>::new();
        q.push_front(State{pos: *p, cost: 0});

        while let Some(curr) = q.pop_back() {
            if !v[self.get_ind(&curr.pos)] {
                v[self.get_ind(&curr.pos)] = true;
                n.push(curr.pos);
                if curr.cost + 1 <= iter {
                
                    for n in self.neighbors(&curr.pos) {
                        q.push_front(State{pos: n, cost: curr.cost+1});
                    }
                }
            }
        }
        */

        for dr in -(iter as i32)..=(iter as i32) {
            if dr < 0 && dr.abs() as usize > p.row {
                continue; 
            } else if dr > 0 && (p.row + dr as usize) >= self.rows {
                continue; 
            }
            for dc in -(iter as i32)..=(iter as i32) {
                if dc < 0 && dc.abs() as usize > p.col {
                    continue; 
                } else if dc > 0 && (p.col + dc as usize) >= self.cols {
                    continue; 
                }

                let dist: usize = (dr.abs() + dc.abs()) as usize;
                if dist == 0 || dist > iter{
                    continue;
                }
                n.push(State{
                    pos: Position{row: ((p.row as i32) + dr) as usize, col: ((p.col as i32) + dc) as usize},
                    cost: dist,
                });
            }
        }

        return n;
    }
    
    fn count_costs(&self, cheats: usize) -> Vec<usize> {
        let mut dist: Vec<usize> = vec![0; self.occup.len()];
        let mut q: VecDeque<Position> = VecDeque::<Position>::new();
        q.push_front(self.start);

        while let Some(curr) = q.pop_back() {
            for n in self.neighbors(&curr) {
                if !self.occup[self.get_ind(&n)] && dist[self.get_ind(&n)] == 0 {
                    dist[self.get_ind(&n)] = dist[self.get_ind(&curr)] + 1;
                    q.push_front(n);
                }
            }
        }
        dist[self.get_ind(&self.start)] = 0;

        let mut counts: Vec<usize> = vec![0; dist[self.get_ind(&self.end)] + 1];
        let mut curr: Position = self.start.clone();
        
        while curr != self.end {
            for s in self.extended_neighbors(&curr, cheats) {
                if !self.occup[self.get_ind(&s.pos)] && dist[self.get_ind(&s.pos)] > dist[self.get_ind(&curr)] + s.cost {
                    let cost: usize = (dist[self.get_ind(&self.end)] - dist[self.get_ind(&s.pos)]) + dist[self.get_ind(&curr)] + s.cost + 1;
                    counts[cost] += 1;
                }
            }

            for n in self.neighbors(&curr) {
                if (dist[self.get_ind(&curr)] + 1) == dist[self.get_ind(&n)] {
                    curr.row = n.row;
                    curr.col = n.col;
                    break;
                }
            }

            if curr == self.start {
                break;
            }
        }
        return counts;
    }

}

fn main() {
    let track = parse_input("data/input_20.txt");

    let counts: Vec<usize> = track.count_costs(2);
    let mut total: usize = 0;
    for (i, c) in counts.iter().enumerate() {
        if *c > 0 {
            if (counts.len() - i) >= 100 {
                total += c;
            }
        }
    }
    println!("There are {} cheats that save at least 100 picoseconds", total);

    let counts: Vec<usize> = track.count_costs(20);
    let mut total: usize = 0;
    for (i, c) in counts.iter().enumerate() {
        if *c > 0 {
            if (counts.len() - i) >= 100 {
                total += c;
            }
        }
    }
    println!("There are {} cheats that save at least 100 picoseconds", total);
}

fn parse_input(fname: &str) -> Track {
    let mut rows: usize = 0;
    let mut cols: usize = 0;

    for line in read_to_string(fname).unwrap().lines() {
        rows += 1;
        cols = line.len();
    }

    let mut track: Track = Track::new(rows, cols);
    for (i, line) in read_to_string(fname).unwrap().lines().enumerate() {
        for (j, c) in line.chars().enumerate() {
            match c {
                '#' => track.occup[i * cols + j] = true,
                '.' => (), // Track cells are unoccupied by default, no need to set
                'S' => track.start = Position{row: i, col: j},
                'E' => track.end = Position{row: i, col: j},
                 _  => panic!("Unexpected character {c} in input file {fname}"),
            }
        }
    }

    return track
}
