use std::fs::read_to_string;
use std::fmt;
use std::collections::VecDeque;
use std::collections::HashMap;

#[derive(Debug,Copy,Clone,PartialEq,Eq)]
enum Action {
    Up,
    Down,
    Left,
    Right,
    Press,
}

#[derive(Debug,Copy,Clone,PartialEq,Eq)]
struct Position {
    row: usize,
    col: usize,
}

#[derive(Debug,Clone,PartialEq,Eq)]
struct State {
    pos: Vec<Position>,
    seq: String,
}

struct Keypad {
    rows: usize,
    cols: usize,
    keys: Vec<Option<char>>,
    partials: Vec<Option<Vec<String>>>,
    cache: Option<HashMap<(usize, String), usize>>,
}

impl Keypad {
    fn from_str(keys: &str) -> Keypad {
        let mut rows: usize = 0;
        let mut cols: usize = 0;

        for line in keys.split(";") {
            rows += 1;
            cols = line.len();
        }

        let mut keypad = Keypad{rows, cols, keys: vec![None; rows*cols], partials: vec![None; rows*rows*cols*cols], cache: None};
        for (r, line) in keys.split(";").enumerate() {
            for (c, key) in line.chars().enumerate() {
                match key {
                    ' ' => keypad.keys[r * cols + c] = None,
                     _  => keypad.keys[r * cols + c] = Some(key),
                };
            }
        }
        keypad.calculate_partials();

        return keypad;
    }

    fn get_key(&self, pos: &Position) -> Option<char> {
        self.keys[pos.row * self.cols + pos.col]
    }

    fn get_ind(&self, pos: &Position) -> usize {
        pos.row * self.cols + pos.col
    }

    fn find_key(&self, key: char) -> Option<Position> {
        for r in 0..self.rows {
            for c in 0..self.cols {
                if let Some(k) = self.keys[r * self.cols + c] {
                    if k == key {
                        return Some(Position{row: r, col: c});
                    }
                }
            }
        }

        return None;
    }

    fn calculate_partials(&mut self) {
        for i in 0..self.keys.len() {
            let r1: usize = i / self.cols;
            let c1: usize = i % self.cols;
            for j in 0..self.keys.len() {
                let r2: usize = j / self.cols;
                let c2: usize = j % self.cols;
                
                if self.keys[i].is_some() && self.keys[j].is_some() {
                    let dc: String;
                    if c1 < c2 {
                        dc = ">".repeat(c2-c1);
                    } else if c2 < c1 {
                        dc = "<".repeat(c1-c2);
                    } else {
                        dc = "".to_string();
                    }
                    
                    let dr: String;
                    if r1 < r2 {
                        dr = "v".repeat(r2-r1);
                    } else if r2 < r1 {
                        dr = "^".repeat(r1-r2);
                    } else {
                        dr = "".to_string();
                    }

                    if r1 == r2 && c1 == c2 {
                        self.partials[i * self.keys.len() + j] = Some(vec!["".to_string()]);
                    } else if r1 == r2 {
                        self.partials[i * self.keys.len() + j] = Some(
                            vec![dc] 
                        );
                    } else if c1 == c2 {
                        self.partials[i * self.keys.len() + j] = Some(
                            vec![dr] 
                        );
                    } else {
                        let mut sols: Vec<String> = Vec::<String>::new();
                        if self.keys[r1 * self.cols + c2].is_some() {
                            sols.push(dc.clone() + &dr);            
                        }
                    
                        if self.keys[r2 * self.cols + c1].is_some() {
                            sols.push(dr.clone() + &dc);            
                        }
                        self.partials[i * self.keys.len() + j] = Some(sols);
                    }
                }
            }
        }
    }
    
    fn build_sequences(&self, seq: &str) -> Vec<String> {
        let mut q: VecDeque<String> = VecDeque::<String>::new();
        q.push_front("".to_string());

        let mut prev: usize = self.get_ind(&self.find_key('A').unwrap());
        for c in seq.chars() {
            let curr: usize = self.get_ind(&self.find_key(c).unwrap());
            let i = q.len();

            for _ in 0..i {
                if let Some(s) = q.pop_back() {
                    if let Some(partial) = &self.partials[prev * self.keys.len() + curr] {
                        for n in partial.iter() {
                            q.push_front(s.clone() + &n + &"A".to_string());
                        }
                    } else {
                        println!("No partial sol: {prev}->{curr}");
                    }
                } else {
                    println!("queue is empty");
                }
            }

            prev = curr;
        }

        Vec::from(q)
    }

    fn shortest_sequence(&mut self, seq: &str, depth: usize) -> usize {
//        println!("D{depth}: {seq}");
        if self.cache.is_none() {
            self.cache = Some(HashMap::<(usize, String), usize>::new());
        }


        if depth == 0 {
            return seq.len();
        }

        if self.cache.as_ref().expect("WAAAAAAA").contains_key(&(depth, seq.to_string())) {
            return *self.cache.as_ref().expect("WAAAAAAA").get(&(depth, seq.to_string())).unwrap();
        }

        let mut total: usize = 0;
        for field in (seq.to_string() + &"_".to_string()).split("A") {
            if field == "_" {
                continue;
            }
//            println!("  F{}", field.to_owned() + &"A".to_string());
            let mut min = 0;
            for (i, sol) in self.build_sequences(&(field.to_owned() + &"A".to_string())).into_iter().enumerate() {
                let l = self.shortest_sequence(&sol, depth-1);
//                println!("  S{}: {}", sol, l);
                if i == 0 || l < min {
                    min = l;
                }
            }

            total += min;
        }

        self.cache.as_mut().expect("WAAAAAAAAAAA").insert((depth, seq.to_string()), total);
        return total;
    }
}

impl fmt::Debug for Keypad {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Keys:\n")?;
        for r in 0..self.rows {
            for c in 0..self.cols {
                if self.keys[r * self.cols + c].is_some() {
                    write!(f, "{}", self.keys[r * self.cols + c].unwrap())?;
                } else {
                    write!(f, " ")?;
                }
            }
            write!(f, "\n")?;
        }

        write!(f, "Partials:\n")?;
        for i in 0..self.keys.len() {
            let r1: usize = i / self.cols;
            let c1: usize = i % self.cols;
            for j in 0..self.keys.len() {
                let r2: usize = j / self.cols;
                let c2: usize = j % self.cols;

                if self.keys[i].is_some() && self.keys[j].is_some() {
                    write!(f, "{:?}->{:?}: {:?}\n", self.keys[i].unwrap(), self.keys[j].unwrap(), self.partials[i * self.keys.len() + j])?;
                }
            }
        }
        Ok(())
    }
}

fn main() {
    let sequences = parse_input("data/input_21.txt");
    let num_pad = Keypad::from_str("789;456;123; 0A");
    let mut dir_pad = Keypad::from_str(" ^A;<v>");

    let mut total: usize = 0;
    for seq in sequences.iter() {
        let mut min: usize = 0;
        for (i, s) in num_pad.build_sequences(&seq).into_iter().enumerate() {
            let l = dir_pad.shortest_sequence(&s, 2);
            if i == 0 || l < min {
                min = l;
            }
        }
        let code = seq[0..3].parse::<usize>().unwrap();
        println!("{seq}: {min}*{code} = {}", min*code);
        total += min * code;
    }
    println!("Total: {total}");
   
    println!("");
    let mut total: usize = 0;
    for seq in sequences.iter() {
        let mut min: usize = 0;
        for (i, s) in num_pad.build_sequences(&seq).into_iter().enumerate() {
            let l = dir_pad.shortest_sequence(&s, 25);
            if i == 0 || l < min {
                min = l;
            }
        }
        let code = seq[0..3].parse::<usize>().unwrap();
        println!("{seq}: {min}*{code} = {}", min*code);
        total += min * code;
    }
    println!("Total: {total}");
}

fn parse_input(fname: &str) -> Vec<String> {
    let mut seq: Vec<String> = Vec::<String>::new();
    for line in read_to_string(fname).unwrap().lines() {
        seq.push(line.to_string());
    }

    return seq;
}
