use std::fs::read_to_string;
use std::collections::VecDeque;
use fancy_regex::Regex;

#[derive(Copy,Clone,Debug,PartialEq)]
enum Operation {
    ADV = 0,
    BXL = 1,
    BST = 2,
    JNZ = 3,
    BXC = 4,
    OUT = 5,
    BDV = 6,
    CDV = 7,
}

impl From<u8> for Operation {
    fn from(v: u8) -> Self {
        match v {
            0 => Self::ADV,
            1 => Self::BXL,
            2 => Self::BST,
            3 => Self::JNZ,
            4 => Self::BXC,
            5 => Self::OUT,
            6 => Self::BDV,
            7 => Self::CDV,
            _ => panic!("Unknown value: {}", v),
        }
    }
}

#[derive(Debug)]
struct Computer {
    reg_a: u64,
    reg_b: u64,
    reg_c: u64,
}

impl Computer {
    fn new(reg_a: u64, reg_b: u64, reg_c: u64) -> Self {
        Computer{reg_a, reg_b, reg_c}
    }

    fn exec(&mut self, prog: &Vec<Operation>) {
        let mut ip: usize = 0;
        let mut first_print = true;
        while ip < prog.len() {
            match prog[ip] {
                Operation::ADV => {
                    let operand = self.parse_combo(prog[ip+1]);
                    self.reg_a = self.reg_a / 2_u64.pow(operand as u32);
                    ip += 2;
                },
                Operation::BXL => {
                    let operand = self.parse_literal(prog[ip+1]);
                    self.reg_b = self.reg_b ^ operand;
                    ip += 2;
                },
                Operation::BST => {
                    let operand = self.parse_combo(prog[ip+1]);
                    self.reg_b = operand % 8;
                    ip += 2;
                },
                Operation::JNZ => {
                    if self.reg_a != 0 {
                        let operand = self.parse_literal(prog[ip+1]);
                        ip = operand as usize;
                    } else {
                        ip += 2;
                    }
                },
                Operation::BXC => {
                    self.reg_b = self.reg_b ^ self.reg_c;
                    ip += 2;
                },
                Operation::OUT => {
                    let operand = self.parse_combo(prog[ip+1]);
                    if first_print {
                        print!("{:?}", (operand % 8));
                        first_print = false;
                    } else {
                        print!(",{:?}", (operand % 8));
                    }
                    ip += 2;
                },
                Operation::BDV => {
                    let operand = self.parse_combo(prog[ip+1]);
                    self.reg_b = self.reg_a / 2_u64.pow(operand as u32);
                    ip += 2;
                },
                Operation::CDV => {
                    let operand = self.parse_combo(prog[ip+1]);
                    self.reg_c = self.reg_a / 2_u64.pow(operand as u32);
                    ip += 2;
                },
            }
        }
        println!("");
    }

    fn parse_literal(&self, op: Operation) -> u64 {
        op as u64
    }
    
    fn parse_combo(&self, op: Operation) -> u64 {
        match op {
            Operation::ADV => 0,
            Operation::BXL => 1,
            Operation::BST => 2,
            Operation::JNZ => 3,
            Operation::BXC => self.reg_a,
            Operation::OUT => self.reg_b,
            Operation::BDV => self.reg_c,
            Operation::CDV => panic!("{:?} is not valid value for combo operand", op),
        }
    }

    fn print_prog(&self, prog: &Vec<Operation>) {
        let mut ip: usize = 0;
        while ip < prog.len() {
            println!("{:?}: {}", ip / 2, match prog[ip] {
                Operation::ADV => format!("regA = regA / 2.pow({})", self.combo_str(prog[ip + 1])).to_string(),
                Operation::BXL => format!("regB = regB ^ {}", prog[ip + 1] as u32).to_string(),
                Operation::BST => format!("regB = {} % 8", self.combo_str(prog[ip + 1])).to_string(),
                Operation::JNZ => format!("JNZ -> {}", prog[ip + 1] as u32).to_string(),
                Operation::BXC => "regB = regB ^ regC".to_string(),
                Operation::OUT => format!("print({} % 8)", self.combo_str(prog[ip + 1])).to_string(),
                Operation::BDV => format!("regB = regA / 2.pow({})", self.combo_str(prog[ip + 1])).to_string(),
                Operation::CDV => format!("regC = regA / 2.pow({})", self.combo_str(prog[ip + 1])).to_string(),
            });
            ip += 2;
        }
    }
    
    fn combo_str(&self, op: Operation) -> String {
        match op {
            Operation::ADV => "0",
            Operation::BXL => "1",
            Operation::BST => "2",
            Operation::JNZ => "3",
            Operation::BXC => "regA",
            Operation::OUT => "regB",
            Operation::BDV => "regC",
            Operation::CDV => "HCF",
        }.to_string()
    }
}

fn main() {
    let (mut comp, prog) = parse_input("data/input_17.txt");
    comp.print_prog(&prog);
    comp.exec(&prog);

    if let Some(reg_a) = invert_prog(&prog) {
        comp.reg_a = reg_a;
        comp.reg_b = 0;
        comp.reg_c = 0;
        println!("Register A: {:?}", comp.reg_a);
        comp.exec(&prog);
    } else {
        println!("Failed to invert program");
    }

}

fn parse_input(fname: &str) -> (Computer, Vec<Operation>){
    let input = read_to_string(fname).unwrap();

    let re = Regex::new(r"(?<=Register A: )[0-9]+").unwrap();
    let reg_a = re.captures(&input).unwrap().unwrap()[0].parse::<u64>().unwrap();
    
    let re = Regex::new(r"(?<=Register B: )[0-9]+").unwrap();
    let reg_b = re.captures(&input).unwrap().unwrap()[0].parse::<u64>().unwrap();
    
    let re = Regex::new(r"(?<=Register C: )[0-9]+").unwrap();
    let reg_c = re.captures(&input).unwrap().unwrap()[0].parse::<u64>().unwrap();

    let comp = Computer::new(reg_a, reg_b, reg_c);
    
    let mut prog = Vec::<Operation>::new();
    let re = Regex::new(r"(?<=Program: )[0-7,]+").unwrap();
    for op in re.captures(&input).unwrap().unwrap()[0].split(",") {
        prog.push(Operation::from(op.parse::<u8>().unwrap()));
    }

    return (comp, prog);
}

fn invert_prog(prog: &Vec<Operation>) -> Option<u64> {
    let mut q: VecDeque<(u64, usize)> = VecDeque::<(u64, usize)>::new();
    q.push_front((0, prog.len()-1));

    while let Some((reg_a, idx)) = q.pop_back() {
        for i in 0..8 {
            let next: u64 = 8 * reg_a + i;
            if predict(next) == prog[idx] {
                if idx == 0 {
                    return Some(next);
                } else {
                    q.push_front((next, idx-1));
                }
            }
        }
    }

    return None;
}

fn predict(reg_a: u64) -> Operation {
    let mut reg_b = (reg_a % 8) ^ 3;
    let reg_c = reg_a / 2_u64.pow(reg_b as u32);
    reg_b = (reg_b ^ 5 ^ reg_c) % 8;
    Operation::from(reg_b as u8)
}
