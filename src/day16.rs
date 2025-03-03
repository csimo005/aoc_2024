use std::fs::read_to_string;
use std::collections::BinaryHeap;
use std::collections::VecDeque;
use std::cmp::Ordering;

struct Maze {
    rows: usize,
    cols: usize,
    map: Vec<char>,
    start: usize,
    goal: usize,
}

#[derive(Debug)]
struct State {
    idx: usize,
    cost: i32,
    dir: Direction,
    act: Action,
}

#[derive(Clone,Copy,Debug)]
enum Direction {
    Up = 0,
    Right = 1,
    Down = 2,
    Left = 3,
}

impl Direction {
    pub fn iterator() -> impl Iterator<Item = Self> {
        [Self::Up, Self::Right, Self::Down, Self::Left].iter().copied()
    }
}

#[derive(Clone,Copy,Debug)]
enum Action {
    TurnLeft = 0,
    Forward = 1,
    TurnRight = 2,
}

impl Action {
    pub fn iterator() -> impl Iterator<Item = Self> {
        [Self::TurnLeft, Self::Forward, Self::TurnRight].iter().copied()
    }
}

impl Maze {
    fn new(rows: usize, cols: usize) -> Self {
        Self {
            rows, cols,
            map: vec!['.'; rows * cols],
            start: 0,
            goal: 0,
        }
    }

    fn step(&self, partial: &State, act: Action) -> State {
        match act {
            Action::TurnLeft => {State{
                 idx: partial.idx,
                cost: partial.cost - 1000,
                 dir: match partial.dir {
                    Direction::Up    => Direction::Left,
                    Direction::Right => Direction::Up,
                    Direction::Down  => Direction::Right,
                    Direction::Left  => Direction::Down,
                },
                 act: act
            }},
            Action::Forward => {State{
                 idx: match partial.dir {
                    Direction::Up    => partial.idx - self.cols,
                    Direction::Right => partial.idx + 1,
                    Direction::Down  => partial.idx + self.cols,
                    Direction::Left  => partial.idx - 1,
                },
                cost: partial.cost - 1,
                 dir: partial.dir,
                 act: act,
            }},
            Action::TurnRight => {State{
                 idx: partial.idx,
                cost: partial.cost - 1000,
                 dir: match partial.dir {
                    Direction::Up    => Direction::Right,
                    Direction::Right => Direction::Down,
                    Direction::Down  => Direction::Left,
                    Direction::Left  => Direction::Up,
                },
                 act: act,
            }},
        }
    }
    
    fn step_inv(&self, partial: &State, act: Action) -> State {
        match act {
            Action::TurnLeft => {State{
                 idx: partial.idx,
                cost: partial.cost + 1000,
                 dir: match partial.dir {
                    Direction::Up    => Direction::Right,
                    Direction::Right => Direction::Down,
                    Direction::Down  => Direction::Left,
                    Direction::Left  => Direction::Up,
                },
                 act: act,
            }},
            Action::Forward => {State{
                 idx: match partial.dir {
                    Direction::Up    => partial.idx + self.cols,
                    Direction::Right => partial.idx - 1,
                    Direction::Down  => partial.idx - self.cols,
                    Direction::Left  => partial.idx + 1,
                },
                cost: partial.cost + 1,
                 dir: partial.dir,
                 act: act,
            }},
            Action::TurnRight => {State{
                 idx: partial.idx,
                cost: partial.cost + 1000,
                 dir: match partial.dir {
                    Direction::Up    => Direction::Left,
                    Direction::Right => Direction::Up,
                    Direction::Down  => Direction::Right,
                    Direction::Left  => Direction::Down,
                },
                 act: act,
            }},
        }
    }

    fn find_path(&self) -> Result<(i32, i32), String> {
        let mut costs: Vec<i32> = vec![-1; 4 * self.map.len()];
        let mut acts: Vec<i32> = vec![0; 4 * self.map.len()];
        let mut v: Vec<bool> = vec![false; 4 * self.map.len()];
        let mut heap = BinaryHeap::<State>::new();
        heap.push(State{idx: self.start, cost: 0, dir: Direction::Right, act: Action::Forward});

        while let Some(p) = heap.pop() {
            if !v[4 * p.idx + (p.dir as usize)] {
                v[4 * p.idx + (p.dir as usize)] = true;
                costs[4 * p.idx + (p.dir as usize)] = -p.cost;
                acts[4 * p.idx + (p.dir as usize)] = 2_i32.pow(p.act as u32);

                for act in Action::iterator() {
                    let next = self.step(&p, act);
                    if self.map[next.idx] == '.' {
                        heap.push(next);
                    }
                }
            } else if costs[4 * p.idx + (p.dir as usize)] == -p.cost {
                acts[4 * p.idx + (p.dir as usize)] += 2_i32.pow(p.act as u32);
            }
        }
               
        let mut min_cost: Direction = Direction::Up;
        for dir in Direction::iterator() {
            if costs[4 * self.goal + (dir as usize)] != -1 {
                if costs[4 * self.goal + (min_cost as usize)] == -1 {
                    min_cost = dir;
                } else if costs[4 * self.goal + (dir as usize)] < costs[4 * self.goal + (min_cost as usize)] {
                    min_cost = dir;
                }
            }
        }

        if costs[4 * self.goal + (min_cost as usize)] > -1 {
            let mut v: Vec<bool> = vec![false; 4 * self.map.len()];
            let mut q: VecDeque<State> = VecDeque::<State>::new();
            q.push_front(State{idx: self.goal, cost: 0, dir: min_cost, act: Action::Forward});

            while let Some(curr) = q.pop_back() {
                if !v[4 * curr.idx + (curr.dir as usize)] {
                    v[4 * curr.idx + (curr.dir as usize)] = true;
    
                    for act in Action::iterator() {
                        if acts[4 * curr.idx + (curr.dir as usize)] & 2_i32.pow(act as u32) > 0 { 
                            q.push_front(self.step_inv(&curr, act));
                        }
                    }
                }
            }
            self.print_visited(&v);

            let mut tiles: i32 = 0;
            for r in 0..self.rows {
                for c in 0..self.cols {
                    for dir in Direction::iterator() {
                        if v[4 * (r * self.cols + c) + (dir as usize)] {
                            tiles += 1;
                            break;
                        }
                    }
                }
            }
            tiles -= 1;

            return Ok((costs[4 * self.goal + (min_cost as usize)], tiles));
        } else {
            return Err("Failed to find path".to_string());
        }
    }

    fn print_map(&self) {
        for r in 0..self.rows {
            for c in 0..self.cols {
                print!("{}", self.map[r * self.cols + c]);
            }
            println!("");
        }
        println!("Start: {:?}, Goal: {:?}", self.start, self.goal);
    }

    fn print_visited(&self, v: &Vec<bool>) {
        for r in 0..self.rows {
            for c in 0..self.cols {
                match self.map[r * self.cols + c] {
                    'S'|'.'|'E' => {
                        let mut total: usize = 0;
                        for i in 0..4 {
                            if v[4 * (r * self.cols + c) + i] {
                                total += 1;
                            }
                        }
                        print!("{}", total);

                    },
                    _ => print!("{}", self.map[r * self.cols + c]),
                };
            }
            println!("");
        }
        println!("Start: {:?}, Goal: {:?}", self.start, self.goal);

    }
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.cost.cmp(&other.cost)
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for State {
    fn eq(&self, other: &Self) -> bool {
        self.cost == other.cost
    }
}

impl Eq for State {}

fn main() {
    let m = parse_input("data/debug_16.txt");
    if let Ok((cost, tiles)) = m.find_path() {
        println!("Debug 1: Min Distance: {:?}, Total Tiles: {:?}", cost, tiles);
    } else {
        println!("Failed to find path");
    }
    
    let m = parse_input("data/debug_16_2.txt");
    if let Ok((cost, tiles)) = m.find_path() {
        println!("Debug 2: Min Distance: {:?}, Total Tiles: {:?}", cost, tiles);
    } else {
        println!("Failed to find path");
    }

    let m = parse_input("data/input_16.txt");
    if let Ok((cost, tiles)) = m.find_path() {
        println!("Input: Min Distance: {:?}, Total Tiles: {:?}", cost, tiles);
    } else {
        println!("Failed to find path");
    }
}

fn parse_input(fname: &str) -> Maze{
    let mut rows = 0;
    let mut cols = 0;
    for line in read_to_string(fname).unwrap().lines() {
        rows += 1;
        cols = line.len();
    }

    let mut m: Maze = Maze::new(rows, cols);
    for (r, line) in read_to_string(fname).unwrap().lines().enumerate() {
        for (c, symb) in line.chars().enumerate() {
            match symb {
                '.' | '#' => m.map[r * m.cols + c] = symb,
                'S' => {
                    m.map[r * m.cols + c] = '.';
                    m.start = r * m.cols + c;
                },
                'E' => {
                    m.map[r * m.cols + c] = '.';
                    m.goal = r * m.cols + c;
                },
                 _  => (),
            }
        }
    }

    return m;
}
