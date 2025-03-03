use std::fs::read_to_string;
use std::cmp;

pub struct List<T> {
    head: Link<T>,
}

type Link<T> = Option<Box<Node<T>>>;

struct Node<T> {
    elem: T,
    next: Link<T>,
}

impl<T> List<T> {
    pub fn new() -> Self {
        List { head: None }
    }

    pub fn push(&mut self, elem: T) {
        let new_node = Box::new(Node {
            elem: elem,
            next: self.head.take(),
        });

        self.head = Some(new_node);
    }

    pub fn pop(&mut self) -> Option<T> {
        self.head.take().map(|node| {
            self.head = node.next;
            node.elem
        })
    }

    pub fn peek(&self) -> Option<&T> {
        self.head.as_ref().map(|node| {
            &node.elem
        })
    }

    pub fn peek_mut(&mut self) -> Option<&mut T> {
        self.head.as_mut().map(|node| {
            &mut node.elem
        })
    }

    pub fn into_iter(self) -> IntoIter<T> {
        IntoIter(self)
    }

    pub fn iter(&self) -> Iter<'_, T> {
        Iter { next: self.head.as_deref() }
    }

    pub fn iter_mut(&mut self) -> IterMut<'_, T> {
        IterMut { next: self.head.as_deref_mut() }
    }
}

impl<T> Drop for List<T> {
    fn drop(&mut self) {
        let mut cur_link = self.head.take();
        while let Some(mut boxed_node) = cur_link {
            cur_link = boxed_node.next.take();
        }
    }
}

pub struct IntoIter<T>(List<T>);

impl<T> Iterator for IntoIter<T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        // access fields of a tuple struct numerically
        self.0.pop()
    }
}

pub struct Iter<'a, T> {
    next: Option<&'a Node<T>>,
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;
    fn next(&mut self) -> Option<Self::Item> {
        self.next.map(|node| {
            self.next = node.next.as_deref();
            &node.elem
        })
    }
}

pub struct IterMut<'a, T> {
    next: Option<&'a mut Node<T>>,
}

impl<'a, T> Iterator for IterMut<'a, T> {
    type Item = &'a mut T;

    fn next(&mut self) -> Option<Self::Item> {
        self.next.take().map(|node| {
            self.next = node.next.as_deref_mut();
            &mut node.elem
        })
    }
}

fn main() {
    let mut compressed = parse_input("data/input_09.txt");
    let mut disk = expand_disk(&compressed);

    let mut head: usize = 0;
    let mut tail: usize = disk.len() - 1;


    while head < tail {
        while tail > 0 && disk[tail] == -1 {
            tail -= 1;
        }
    
        while head < disk.len() && disk[head] != -1 {
            head += 1;
        }

        if tail <= head {
            break;
        }

        disk.swap(head, tail);
    }

    println!("Part 1: {:?}\n", check_sum(&disk));

    let max_id = cmp::max(compressed[compressed.len() - 1].0, compressed[compressed.len() - 2].0);
    for id in (0..max_id+1).rev() {
        let mut size: i32 = 0;
        for i in 0..compressed.len() {
            if compressed[i].0 == id {
                size = compressed[i].1;
            }
        }

        for i in 0..compressed.len() {
            if compressed[i].0 == -1 && size <= compressed[i].1 {
                compressed[i].1 -= size;
                compressed.insert(i, (id, size));
                compressed.insert(i, (-1, 0));
                for j in (0..compressed.len()).rev() {
                    if compressed[j].0 == id {
                        compressed[j].0 = -1;
                        break;
                    }
                }

                break;
            }
        }
    }

    println!("Part 2: {:?}\n", check_sum(&expand_disk(&compressed)));
}

fn parse_input(fname: &str) -> Vec<(i32, i32)> {
    let data = read_to_string(fname).unwrap();

    let mut compressed: Vec<(i32, i32)> = Vec::<(i32, i32)>::new();
    let mut id = 0;

    for (i, c) in data.chars().enumerate() {
        if c != '\n' {
            let size = c.to_string().parse::<i32>().unwrap();
            if i % 2 == 0 {
                compressed.push((id, size));
                id += 1;
            } else {
                compressed.push((-1, size));
            }
        }
    }

    return compressed
}

fn expand_disk(compressed: &Vec<(i32, i32)>) -> Vec<i32> {
    let mut disk: Vec<i32> = Vec::<i32>::new();
    for (id, size) in compressed.iter() {
        for _ in 0..*size {
            disk.push(*id);
        }
    }

    return disk;

}

fn print_disk(disk: &Vec<i32>) {
    for i in 0..disk.len() {
        if disk[i] > -1 {
            print!("{:?}", disk[i]);
        } else {
            print!(".");
        }
    }
    print!("\n");
}

fn check_sum(disk: &Vec<i32>) -> i64 {
    let mut val: i64 = 0;
    for i in 0..disk.len() {
        if disk[i] > -1 {
            val += (i as i64) * (disk[i] as i64)
        }
    }

    return val;
}
