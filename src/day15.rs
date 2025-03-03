use std::fs::read_to_string;
use std::collections::VecDeque;

struct World {
    width: usize,
    height: usize,
    map: Vec<char>,
    robot: (usize, usize),
}

impl World {
    fn new(width: usize, height: usize) -> Self {
        let mut w = World {
            width, height,
            map: vec!['.'; width * height],
            robot: (0, 0),
        };

        return w;
    }

    fn cmd_robot(&mut self, cmd: u8) {
        if let Some(src) = self.step(self.robot.0 * self.width + self.robot.1, cmd) {
            if let Some(crates) = self.find_crates(src, cmd) {
                let mut chars: Vec<char> = vec!['.'; crates.len()];
                for i in 0..crates.len() {
                    chars[i] = self.map[crates[i]];
                    self.map[crates[i]] = '.';
                }

                for i in 0..crates.len() {
                    let n = self.step(crates[i], cmd).unwrap();
                    self.map[n] = chars[i];
                }

                self.map[self.robot.0 * self.width + self.robot.1] = '.';
                self.robot.0 = src / self.width;
                self.robot.1 = src % self.width;
                self.map[self.robot.0 * self.width + self.robot.1] = '@';
            }
        }
   }
    

    fn step(&self, idx: usize, cmd: u8) -> Option<usize> {
        match cmd {
            0 => Some(idx - self.width),
            1 => Some(idx + 1),
            2 => Some(idx + self.width),
            3 => Some(idx - 1),
            _ => None,
        }
    }

    fn find_crates(&self, idx: usize, cmd: u8) -> Option<Vec<usize>> {
        let mut crates: Vec<usize> = Vec::<usize>::new();
        let mut v: Vec<bool> = vec![false; self.map.len()];
        let mut q: VecDeque<usize> = VecDeque::<usize>::new();
        q.push_front(idx);

        while let Some(i) = q.pop_back() {
            if !v[i] {
                v[i] = true;
                match self.map[i] {
                    'O' => {
                        crates.push(i);
                        q.push_front(self.step(i, cmd)?);
                    },
                    '[' => {
                        crates.push(i);
                        q.push_front(self.step(i, cmd)?);
                        q.push_front(self.step(i, 1)?);
                    },
                    ']' => {
                        crates.push(i);
                        q.push_front(self.step(i, cmd)?);
                        q.push_front(self.step(i, 3)?);
                    },
                    '.' => (),
                    '#' => return None,
                     _  => (),
                };
            }
        }
    
        return Some(crates);
    }

    fn stretch(&self) -> Self {
        let mut stretched: Self = Self::new(self.width * 2, self.height);
        for i in 0..self.height {
            for j in 0..self.width {
                let idx: usize = i * self.width + j;
                match self.map[idx] {
                    '.' | '#' => {
                        stretched.map[2 * idx] = self.map[idx];
                        stretched.map[2 * idx + 1] = self.map[idx];
                    },
                    'O' => {
                        stretched.map[2 * idx] = '[';
                        stretched.map[2 * idx + 1] = ']';
                    },
                    '@' => {
                        stretched.map[2 * idx] = '@';
                        stretched.map[2 * idx + 1] = '.';
                    },
                     _  => (),
                }
            }
        }
        stretched.robot.0 = self.robot.0;
        stretched.robot.1 = 2 * self.robot.1;

        return stretched;
    }
    
    fn print_map(&self) {
        for i in 0..self.height {
            for j in 0..self.width {
                print!("{}", self.map[i * self.width + j]);
            }
            println!("");
        }
        println!("Robot Pose: {:?}, {:?}, Score {:?}", self.robot.0, self.robot.1, self.score_map());
        println!("");
    }

    fn score_map(&self) -> i64 {
        let mut total: i64 = 0;

        for i in 0..self.height {
            for j in 0..self.width {
                total += match self.map[i * self.width + j] {
                    'O' | '[' => (100 * i + j) as i64,
                     _  => 0,
                }
            }
        }

        return total;
    }
}

fn main() {
    let (mut w, m) = parse_input("data/input_15.txt");
    println!("Initial State:");
    w.print_map();

    for cmd in m {
        w.cmd_robot(cmd);
        println!("Move {}:", match cmd {
            0 => "^",
            1 => ">",
            2 => "v",
            3 => "<",
            _ => "#",
        });
        w.print_map();
    }
    
    let (w, m) = parse_input("data/input_15.txt");
    let mut w = w.stretch();
    println!("Initial State:");
    w.print_map();

    for cmd in m {
        w.cmd_robot(cmd);
        println!("Move {}:", match cmd {
            0 => "^",
            1 => ">",
            2 => "v",
            3 => "<",
            _ => "#",
        });
        w.print_map();
    }
}

fn parse_input(fname: &str) -> (World, Vec<u8>) {
    let mut rows: usize = 0;
    let mut cols: usize = 0;

    for line in read_to_string(fname).unwrap().lines() {
        if line.len() > 0 {
            rows += 1;
            cols = line.len();
        } else {
            break;
        }
    }

    let mut world = World::new(cols, rows);
    let mut moves = Vec::<u8>::new();

    for (i, line) in read_to_string(fname).unwrap().lines().enumerate() {
        if let Some(f) = line.chars().nth(0) {
            match f {
                '#' => {
                    for (j, c) in line.chars().enumerate() {
                        match c {
                            '#' | 'O' => world.map[i * world.width + j] = c,
                            '@' => {
                                world.map[i * world.width + j] = c;
                                world.robot.0 = i;
                                world.robot.1 = j;
                            },
                            _ => (),
                        };
                    }
                },
                '^' | '>' | 'v' | '<' => {
                    for m in line.chars() {
                        match m {
                            '^' => moves.push(0),
                            '>' => moves.push(1),
                            'v' => moves.push(2),
                            '<' => moves.push(3),
                            _ => (),
                        }
                    }
                },
                _ => (),
            }
        }
    }

    return (world, moves);
}
