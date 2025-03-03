use std::fs::read_to_string;
use std::fmt;
use std::time::Instant;
use std::collections::VecDeque;

#[derive(Clone)]
struct Test {
    result: i64,
    operands: Vec<i64>,
}

#[derive(Debug)]
struct PartialSol {
    result: i64,
    idx: usize,
}

impl Test {
/*
    fn check_ops1(&self, key: i64) -> bool {
        let mut result = self.operands[0];

        for i in 1..self.operands.len() {
            if result > self.result {
                return false;
            }

            let op = key & (1<<(i-1));
            if op == 0 {
                result = result + self.operands[i];   
            } else {
                result = result * self.operands[i];   
            }
        }

        return result == self.result;
    }

    fn check_pt1(&self) -> bool {
        for i in 0..2_i64.pow((self.operands.len()-1) as i64) {
            if self.check_ops1(i) {
                return true;
            }
        }

        return false;
    }

    fn check_ops2(&self, key: i64) -> bool {
        let mut result = self.operands[0];

        for i in 1..self.operands.len() {
            if result > self.result {
                return false;
            }

            let op = (key / 3_i64.pow((i-1) as i64)) % 3;
            if op == 0 {
                result = result + self.operands[i];   
            } else if op == 1 {
                result = result * self.operands[i];   
            } else {
                let mut pow = 10;
                while pow <= self.operands[i] {
                    pow *= 10;
                }
                result = result * pow + self.operands[i];
            }
        }

        return result == self.result;
    }

    fn check_pt2(&self) -> bool {
        for i in 0..3_i64.pow((self.operands.len()-1) as i64) {
            if self.check_ops2(i) {
                return true;
            }
        }

        return false;
    }
*/
    
    fn check_pt1(&self) -> bool {
        let mut q: VecDeque<PartialSol> = VecDeque::<PartialSol>::new();
        q.push_front(PartialSol{result: self.operands[0], idx: 1});

        while let Some(s) = q.pop_back() {
            if s.idx == self.operands.len() {
                if s.result == self.result {
                    return true;
                } else {
                    continue;
                }
            }

            if s.result + self.operands[s.idx] <= self.result {
                q.push_front(PartialSol{result: s.result + self.operands[s.idx], idx: s.idx + 1});
            }
            
            if s.result * self.operands[s.idx] <= self.result {
                q.push_front(PartialSol{result: s.result * self.operands[s.idx], idx: s.idx + 1});
            }
        }

        return false;
    }
    
    fn check_pt2(&self) -> bool {
        let mut q: VecDeque<PartialSol> = VecDeque::<PartialSol>::new();
        q.push_front(PartialSol{result: self.operands[0], idx: 1});

        while let Some(s) = q.pop_back() {
            if s.idx == self.operands.len() {
                if s.result == self.result {
                    return true;
                } else {
                    continue;
                }
            }

            if s.result + self.operands[s.idx] <= self.result {
                q.push_front(PartialSol{result: s.result + self.operands[s.idx], idx: s.idx + 1});
            }
            
            if s.result * self.operands[s.idx] <= self.result {
                q.push_front(PartialSol{result: s.result * self.operands[s.idx], idx: s.idx + 1});
            }

            let v = Self::int_cat(s.result, self.operands[s.idx]);
            if v <= self.result {
                q.push_front(PartialSol{result: v, idx: s.idx + 1});
            }
        }

        return false;
    }

    fn int_cat(lhs: i64, rhs: i64) -> i64 {
        let mut pow = 10;
        while pow <= rhs {
            pow *= 10;
        }
        lhs * pow + rhs
    }
}

impl fmt::Debug for Test {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}: {:?}", self.result, self.operands)
    }
}

fn main() {
    let tests = parse_input("data/input_07.txt");

    let before = Instant::now();
    let mut total = 0;
    let mut result = 0;
    for test in tests.iter() {
        if test.check_pt1() {
            total += 1;
            result += test.result;
        }
    }
    println!("Time: {:.2?}, Passed: {:?}/{:?}, Result: {:?}", before.elapsed(), total, tests.len(), result);
    
    let before = Instant::now();
    let mut total = 0;
    let mut result = 0;
    for test in tests.iter() {
        if test.check_pt2() {
            total += 1;
            result += test.result;
        }
    }
    println!("Time: {:.2?}, Passed: {:?}/{:?}, Result: {:?}", before.elapsed(), total, tests.len(), result);
}

fn parse_input(fname: &str) -> Vec<Test> {
    let mut tests: Vec<Test> = Vec::<Test>::new();
    for line in read_to_string(fname).unwrap().lines() {
        println!("{:?}", line);
        let fields: Vec<&str> = line.split(": ").collect();
        let result: i64 = fields[0].parse::<i64>().unwrap();
        let operands: Vec<i64> = fields[1].split(" ").map(|c| c.parse::<i64>().unwrap()).collect();

        tests.push(Test{result: result, operands: operands});
    }

    return tests;
}
