use std::fs::read_to_string;
use std::collections::HashMap;
use std::cell::RefCell;
use std::rc::Rc;
use std::fmt;
use std::cmp::Ordering;

use regex::Regex;

#[derive(Debug,Copy,Clone,Eq,PartialEq)]
enum Operation {
    And,
    Xor,
    Or,
    ConstTrue,
    ConstFalse,
    Dummy,
}

#[derive(Clone)]
struct Gate {
    name: String,
    op: Operation,
    input_a: Option<Rc<RefCell<Gate>>>,
    input_b: Option<Rc<RefCell<Gate>>>,
    value: Option<bool>,
}

impl Gate {
    fn print(&self) {
        match self.op {
            Operation::ConstTrue => println!("{}: 1", self.name),
            Operation::ConstFalse => println!("{}: 0", self.name),
            Operation::And => {
                let Some(ref input_a) = self.input_a else { panic!("") };
                let Some(ref input_b) = self.input_b else { panic!("") };
                println!("{} AND {} -> {}", input_a.borrow().name, input_b.borrow().name, self.name);
            },
            Operation::Or => {
                let Some(ref input_a) = self.input_a else { panic!("") };
                let Some(ref input_b) = self.input_b else { panic!("") };
                println!("{}  OR {} -> {}", input_a.borrow().name, input_b.borrow().name, self.name);
            },
            Operation::Xor => {
                let Some(ref input_a) = self.input_a else { panic!("") };
                let Some(ref input_b) = self.input_b else { panic!("") };
                println!("{} XOR {} -> {}", input_a.borrow().name, input_b.borrow().name, self.name);
            },
            _ => println!("{:?}", self),
        };
    }
}

impl fmt::Debug for Gate {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Gate(Name: {:?}, Op: {:?}", self.name, self.op)?;
        if self.input_a.is_none() {
            write!(f, ", Input A: N/A")?;
        } else {
            write!(f, ", Input A: {:?}({:?})", self.input_a.clone().unwrap().borrow().name, self.input_a.clone().unwrap().borrow().op)?;
        }
        if self.input_b.is_none() {
            write!(f, ", Input B: N/A")?;
        } else {
            write!(f, ", Input B: {:?}({:?})", self.input_b.clone().unwrap().borrow().name, self.input_b.clone().unwrap().borrow().op)?;
        }
        if self.value.is_none() {
            write!(f, ", Value: N/A)")
        } else {
            write!(f, ", Value: {:?})", self.value.unwrap())
        }
    }
}


#[derive(Debug)]
struct Circuit {
    gates: HashMap<String, Rc<RefCell<Gate>>>,
}

impl Circuit {
    fn new() -> Self {
        Self{gates: HashMap::<String, Rc<RefCell<Gate>>>::new()}
    }

    fn add_gate(&mut self, name: &str, op: Operation, input_a: Option<&str>, input_b: Option<&str>) {
        let gate_a = match input_a {
            None => None,
            Some(s) => match self.gates.get(s) {
                Some(g) => Some(g.clone()),
                None => Some(Rc::new(RefCell::new(
                    Gate {
                        name: s.to_string(),
                        op: Operation::Dummy,
                        input_a: None,
                        input_b: None,
                        value: None,
                    }
                ))),
            },
        };
        
        let gate_b = match input_b {
            None => None,
            Some(s) => match self.gates.get(s) {
                Some(g) => Some(g.clone()),
                None => Some(Rc::new(RefCell::new(
                    Gate {
                        name: s.to_string(),
                        op: Operation::Dummy,
                        input_a: None,
                        input_b: None,
                        value: None,
                    }
                ))),
            },
        };

        let g: Rc<RefCell<Gate>>;
        if input_a.cmp(&input_b) == Ordering::Less {
            g = Rc::new(RefCell::new(
                Gate {
                    name: name.to_string(),
                    op,
                    input_a: gate_a,
                    input_b: gate_b,
                    value: None
                }
            ));
        } else {
            g = Rc::new(RefCell::new(
                Gate {
                    name: name.to_string(),
                    op,
                    input_a: gate_b,
                    input_b: gate_a,
                    value: None
                }
            ));

        }
        self.gates.insert(name.to_string(), g.clone());

        for (_key, gate) in self.gates.iter() {
            if gate.borrow().input_a.is_some() {
                if gate.borrow().input_a.clone().expect("Ooppps").borrow().name == name {
                    gate.borrow_mut().input_a = Some(g.clone());
                }
            }

            if gate.borrow().input_b.is_some() {
                if gate.borrow().input_b.clone().expect("Ooppps").borrow().name == name {
                    gate.borrow_mut().input_b = Some(g.clone());
                }
            }
        }
    }

    fn simulate(&mut self) {
        let mut done = false;
        while !done {
            done = true;
        
            for (_key, gate) in self.gates.iter() {
                let mut gate = gate.borrow_mut();
                match gate.op {
                    Operation::ConstTrue => {
                        if gate.value.is_none() {
                            gate.value = Some(true);
                            done = false;
                        }
                    },
                    Operation::ConstFalse => {
                        if gate.value.is_none() {
                            gate.value = Some(false);
                            done = false;
                        }
                    },
                    Operation::And => {
                        if gate.value.is_none() {
                            let Some(ref input_a) = gate.input_a else { panic!("") };
                            let Some(ref input_b) = gate.input_b else { panic!("") };
                            if input_a.borrow().value.is_some() && input_b.borrow().value.is_some() {
                                if input_a.borrow().value.unwrap() && input_b.borrow().value.unwrap() {
                                    gate.value = Some(true);
                                } else {
                                    gate.value = Some(false);
                                }
                            }
                            done = false;
                        }
                    },
                    Operation::Or => {
                        if gate.value.is_none() {
                            let Some(ref input_a) = gate.input_a else { panic!("") };
                            let Some(ref input_b) = gate.input_b else { panic!("") };
                            if input_a.borrow().value.is_some() && input_b.borrow().value.is_some() {
                                if input_a.borrow().value.unwrap() || input_b.borrow().value.unwrap() {
                                    gate.value = Some(true);
                                } else {
                                    gate.value = Some(false);
                                }
                            }
                            done = false;
                        }
                    },
                    Operation::Xor => {
                        if gate.value.is_none() {
                            let Some(ref input_a) = gate.input_a else { panic!("") };
                            let Some(ref input_b) = gate.input_b else { panic!("") };
                            if input_a.borrow().value.is_some() && input_b.borrow().value.is_some() {
                                if input_a.borrow().value.unwrap() ^ input_b.borrow().value.unwrap() {
                                    gate.value = Some(true);
                                } else {
                                    gate.value = Some(false);
                                }
                            }
                            done = false;
                        }
                    },
                    Operation::Dummy => panic!("Unexpected Dummy Gate!"),
                };
            }
        }
    }
}

fn main() {
    let mut circuit = parse_input("data/input_24.txt");
    circuit.simulate();

    let re1 = Regex::new(r"z([0-9]+)").unwrap();
    let mut result: u64 = 0;
    for (key, gate) in circuit.gates.iter() {
        if let Some(cap) = re1.captures(key) {
            if gate.borrow().value.unwrap() {
                let pow: u64 = cap[1].parse::<u64>().unwrap();
                result += 1 << pow;
            }
        }
    }

    println!("Result: {:?}", result);

    let mut keys: Vec<_> = circuit.gates.keys().collect();
    keys.retain(|&k| circuit.gates.get(k).unwrap().borrow().op == Operation::ConstTrue || circuit.gates.get(k).unwrap().borrow().op == Operation::ConstFalse);
    keys.sort();

    for k in keys.iter() {
        if let Some(gate) = circuit.gates.get(*k) {
            gate.borrow().print();
        }
    }
    
    let mut keys: Vec<_> = circuit.gates.keys().collect();
    keys.retain(|&k| circuit.gates.get(k).unwrap().borrow().op == Operation::And);
    keys.sort_by(|&a, &b| circuit.gates.get(a).unwrap().borrow().input_a.as_ref().name.cmp(&circuit.gates.get(b).unwrap().borrow().input_a.unwrap().borrow().name));

    for k in keys.iter() {
        if let Some(gate) = circuit.gates.get(*k) {
            gate.borrow().print();
        }
    }
    
    let mut keys: Vec<_> = circuit.gates.keys().collect();
    keys.retain(|&k| circuit.gates.get(k).unwrap().borrow().op == Operation::Xor);
    keys.sort();

    for k in keys.iter() {
        if let Some(gate) = circuit.gates.get(*k) {
            gate.borrow().print();
        }
    }
    
    let mut keys: Vec<_> = circuit.gates.keys().collect();
    keys.retain(|&k| circuit.gates.get(k).unwrap().borrow().op == Operation::Or);
    keys.sort();

    for k in keys.iter() {
        if let Some(gate) = circuit.gates.get(*k) {
            gate.borrow().print();
        }
    }
}

fn parse_input(fname: &str) -> Circuit {
    let mut circ = Circuit::new();

    let re1 = Regex::new(r"([a-z0-9]+): ([01])").unwrap();
    let re2 = Regex::new(r"([a-z0-9]+)\s([A-Z]+)\s([a-z0-9]+)\s->\s([a-z0-9]+)").unwrap();
    for (i, line) in read_to_string(fname).unwrap().lines().enumerate() {
        if let Some(cap) = re1.captures(line) {
            let op: Operation = match cap[2].chars().nth(0).unwrap() {
                '0' => Operation::ConstFalse,
                '1' => Operation::ConstTrue,
                 _  => panic!("Unexpected value {} in constant value definition", cap[2].to_string()),
            };
            circ.add_gate(&cap[1].to_string(), op, None, None);
        } else if let Some(cap) = re2.captures(line) {
            let op: Operation = match cap[2].chars().nth(0).unwrap() {
                'A' => Operation::And,
                'O' => Operation::Or,
                'X' => Operation::Xor,
                 _  => panic!("Unexpected value {} in constant value definition", cap[2].to_string()),
            };
            circ.add_gate(
                &cap[4].to_string(),
                op,
                Some(&cap[1].to_string()),
                Some(&cap[3].to_string()),
            );
        } else if line == "" {
            continue;
        } else {
            println!("{:?}: {:?}, Unhandled input {:?}", fname, i, line);
        }
    }

    return circ;
}
