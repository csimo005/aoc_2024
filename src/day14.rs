use std::fs::read_to_string;
use std::collections::VecDeque;
use regex::Regex;

#[derive(Debug)]
struct Robot {
    px: i32,
    py: i32,
    vx: i32,
    vy: i32,
}

#[derive(Debug)]
struct World {
    height: usize,
    width: usize,
    robots: Vec<Robot>,
}

impl World {
    pub fn step(&mut self) {
        for r in self.robots.iter_mut() {
            r.px = (r.px + r.vx) % (self.width as i32);
            if r.px < 0 {
                r.px = (self.width as i32) + r.px;
            }

            r.py = (r.py + r.vy) % (self.height as i32);
            if r.py < 0 {
                r.py = (self.height as i32) + r.py;
            }
        }
    }

    pub fn print_map(&self) {
        let mut counts: Vec<u32> = vec![0; self.width * self.height];
        for r in self.robots.iter() {
            counts[(r.py as usize) * self.width + (r.px as usize)] += 1;
        }

        for i in 0..self.height {
            for j in 0..self.width {
                if counts[i * self.width + j] == 0 {
                    print!(".");
                } else {
                    print!("{}", counts[i * self.width + j]);
                }
            }
            if i+1 < self.height {
                print!("\n");
            }
        }
        print!("\n\n");
    }

    pub fn sum_quadrants(&self) -> Vec<u32>{
        let mut res: Vec<u32> = vec![0, 0, 0, 0];

        for r in self.robots.iter() {
            if r.px < ((self.width / 2) as i32) {
                if r.py < ((self.height / 2) as i32) {
                    res[0] += 1;
                } else if r.py > ((self.height / 2) as i32) {
                    res[1] += 1;
                }
            } else if r.px > ((self.width / 2) as i32) {
                if r.py < ((self.height / 2) as i32) {
                    res[2] += 1;
                } else if r.py > ((self.height / 2) as i32) {
                    res[3] += 1;
                }

            }
        }

        return res;
    }

    pub fn is_treelike(&self) -> bool {
        let mut counts: Vec<u32> = vec![0; self.width * self.height];
        for r in self.robots.iter() {
            counts[(r.py as usize) * self.width + (r.px as usize)] += 1;
            if counts[(r.py as usize) * self.width + (r.px as usize)] > 1 {
                return false;
            }
        }

        let mut v: Vec<u32> = vec![0; counts.len()];
        let mut connected = 0;

        for i in 0..counts.len() {
            if counts[i] == 1 && v[i] == 0 {
                connected += 1;

                let mut q: VecDeque<usize> = VecDeque::<usize>::new();
                q.push_front(i);

                while q.len() > 0 {
                    let c = q.pop_back().unwrap();

                    if v[c] == 0 {
                         v[c] = 1;
     
                         if c > self.width {
                             q.push_front(c - self.width);
                         }
                         
                         if c+self.width > counts.len() {
                             q.push_front(c + self.width);
                         }

                         if c % self.width > 1 {
                             q.push_front(c - 1);
                         }
                         
                         if c % self.width < self.width - 1 {
                             q.push_front(c + 1);
                         }

                    }
                }
            }
        }

        if connected > 1 {
            println!("Found {:?} connected components", connected);
        }
        return connected <= 1;
    }
}

fn main() {
    let robots = parse_input("data/input_14.txt");
    let mut world: World = World{
        height: 103,
        width: 101,
        robots: robots,
    };

    world.print_map();
    for _ in 0..100 {
        world.step();
    }
    world.print_map();
   
    let mut prod: u32 = 1;
    for v in world.sum_quadrants() {
        prod *= v;
    }
    println!("Quadrants: {:?}, Safety Factor: {:?}", world.sum_quadrants(), prod);
    
    let robots = parse_input("data/input_14.txt");
    let mut world: World = World{
        height: 103,
        width: 101,
        robots: robots,
    };

    for i in 0..10000 {
        println!("Steps {:?}:", i);
        world.print_map();
        world.step();
    }

}

fn parse_input(fname: &str) -> Vec<Robot> {
    let re1 = Regex::new("p=([0-9]+),([0-9]+)").unwrap();
    let re2 = Regex::new("v=(-?[0-9]+),(-?[0-9]+)").unwrap();
    let mut robots: Vec<Robot> = Vec::<Robot>::new();

    for line in read_to_string(fname).unwrap().lines() {
        let caps = re1.captures(line).unwrap();
        let px = caps[1].parse::<i32>().unwrap();
        let py = caps[2].parse::<i32>().unwrap();
        
        let caps = re2.captures(line).unwrap();
        let vx = caps[1].parse::<i32>().unwrap();
        let vy = caps[2].parse::<i32>().unwrap();

        robots.push(Robot{px, py, vx, vy});
    }

    return robots;
}
