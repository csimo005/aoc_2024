use std::fs::read_to_string;
use std::hash::{Hash, Hasher};
use std::collections::HashMap;
use regex::Regex;

#[derive(Debug,Copy,Clone,PartialEq,Eq,Hash)]
enum Color {
    White=0,
    Blue=1,
    Black=2,
    Red=3,
    Green=4,
}

#[derive(Debug,Eq,Clone)]
struct Pattern {
    stripes: Vec<Color>,
}

impl PartialEq for Pattern {
    fn eq(&self, other: &Self) -> bool {
        if self.stripes.len() == other.stripes.len() {
            for i in 0..self.stripes.len() {
                if self.stripes[i] != other.stripes[i] {
                    return false;
                }
            }
            return true;
        }
        return false;
    }
}

impl Hash for Pattern {
    fn hash<H: Hasher>(&self, state: &mut H) {
        for (i, c) in self.stripes.iter().enumerate() {
            c.hash(state);
            state.write_u32(i as u32);
        }
    }
}

impl Pattern {
    fn new() -> Self {
        Pattern{stripes: Vec::<Color>::new()}
    }

    fn len(&self) -> usize {
        self.stripes.len()
    }

    fn remove_prefix(&self, prefix: &Self) -> Option<Self> {
        if self.len() < prefix.len() {
            return None;
        } else {
            for i in 0..prefix.len() {
                if self.stripes[i] != prefix.stripes[i] {
                    return None;
                }
            }

            let mut ret: Self = Self{stripes: vec![Color::White; self.len() - prefix.len()]};
            for i in 0..(self.len() - prefix.len()) {
                ret.stripes[i] = self.stripes[prefix.len() + i];
            }

            return Some(ret);
        }
    }
}

fn main() {
    let (towels, goals) = parse_input("data/input_19.txt");
    let mut partial_sol = HashMap::<Pattern, u64>::new();

    let mut total_possible: u64 = 0;
    let mut total_arrangments: u64 = 0;
    for g in goals.iter() {
        if count_solutions(&g, &towels, &mut partial_sol) > 0 {
            total_possible += 1;
            total_arrangments += count_solutions(&g, &towels, &mut partial_sol);
        }
    }

    println!("Total Possible: {:?}, Total Arrangements: {:?}", total_possible, total_arrangments);
}

fn count_solutions(goal: &Pattern, towels: &Vec<Pattern>, partials: &mut HashMap<Pattern, u64>) -> u64 {
    if partials.contains_key(goal) {
        return *partials.get(goal).unwrap();
    } else {
        let mut count: u64 = 0;
        for prefix in towels.iter() {
            if let Some(postfix) = goal.remove_prefix(prefix) {
                if postfix.len() == 0 {
                    count += 1;
                } else {
                    count += count_solutions(&postfix, &towels, partials);
                }
            }
        }

        partials.insert(goal.clone(), count);
        return count;
    }
}

fn parse_input(fname: &str) -> (Vec<Pattern>, Vec<Pattern>) {
    let mut towels: Vec<Pattern> = Vec::<Pattern>::new();
    let mut goals: Vec<Pattern> = Vec::<Pattern>::new();

    for (i, line) in read_to_string(fname).unwrap().lines().enumerate() {
        match i {
            0 => {
                let re = Regex::new("(([wubrg]+)(, )?)").unwrap();
                for cap in re.captures_iter(&line) {
                    let mut pat = Pattern::new();
                    for c in cap[2].chars() {
                        pat.stripes.push(match c {
                            'w' => Color::White,
                            'u' => Color::Blue,
                            'b' => Color::Black,
                            'r' => Color::Red,
                            'g' => Color::Green,
                            _ => panic!("Unknown color code: {:?}", c),
                        });
                    }
                    towels.push(pat);
                }
            },
            1 => (),
            _ => {
                let mut pat = Pattern::new();
                for c in line.chars() {
                    pat.stripes.push(match c {
                        'w' => Color::White,
                        'u' => Color::Blue,
                        'b' => Color::Black,
                        'r' => Color::Red,
                        'g' => Color::Green,
                        _ => panic!("Unknown color code: {:?}", c),
                    });
                }
                goals.push(pat);
            }
        };
    }

    return (towels, goals);
}
