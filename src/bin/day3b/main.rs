use std::cell::RefCell;
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;
use std::rc::Rc;
use crate::PartOrGear::PART;

fn main() {
    let input = Path::new(file!()).parent().unwrap().parent().unwrap().join("day3a/input.txt");
    let input = BufReader::new(File::open(input).expect("Input file missing"));

    let mut index: HashMap<(i32, i32), PartOrGear> = HashMap::new();
    let mut gears: Vec<Rc<RefCell<HashSet<Rc<PartNum>>>>> = Vec::new();
    for (i, line) in input.lines().enumerate() {
        let i = i as i32;
        let line = line.expect("IO error");
        let line = line.as_bytes();
        let mut j = 0i32;
        while j < (line.len() as i32) {
            match line[j as usize] {
                b'0'..=b'9' => {
                    let (num, num_end) = parse_part_num(j as usize, line);
                    let num = Rc::new(PartNum::new(num));
                    for j_1 in j..(num_end as i32) {
                        index.insert((i, j_1), PART(num.clone()));
                    }
                    for j_2 in (j - 1)..=(num_end as i32) {
                        if let Some(pog) = index.get(&((i - 1), j_2))
                        {
                            if let PartOrGear::GEAR(parts) = pog {
                                parts.borrow_mut().insert(num.clone());
                            }
                        }
                    }
                    if let Some(pog) = index.get(&(i, (j - 1)))
                    {
                        if let PartOrGear::GEAR(parts) = pog {
                            parts.borrow_mut().insert(num.clone());
                        }
                    }
                    j = num_end as i32 - 1;
                }
                b'*' => {
                    let parts = Rc::new(RefCell::new(HashSet::new()));
                    gears.push(parts.clone());
                    index.insert((i, j), PartOrGear::GEAR(parts.clone()));
                    for j_1 in (j - 1)..=(j + 1) {
                        if let Some(pog) = index.get(&((i - 1), j_1))
                        {
                            if let PART(part) = pog {
                                parts.borrow_mut().insert(part.clone());
                            }
                        }
                    }
                    if let Some(pog) = index.get(&(i, (j - 1)))
                    {
                        if let PART(part) = pog {
                            parts.borrow_mut().insert(part.clone());
                        }
                    }
                }
                _ => {}
            };
            j += 1;
        }
    }

   let sum: u64 = gears.iter()
       .filter(|parts| parts.borrow().len() == 2)
       .map(|parts| parts.borrow().iter()
           .map(|p| p.num)
           .product::<u64>())
       .sum();
    println!("Gear sum {}", sum);
}

fn parse_part_num(mut idx: usize, line: &[u8]) -> (u64, usize) {
    let mut num = 0u64;
    while idx < line.len() && line[idx].is_ascii_digit() {
        num *= 10;
        num += (line[idx] as char).to_digit(10).expect("base 10 digit") as u64;
        idx += 1;
    }
    return (num, idx);
}

#[derive(Debug)]
enum PartOrGear {
    PART(Rc<PartNum>),
    GEAR(Rc<RefCell<HashSet<Rc<PartNum>>>>),
}

#[derive(Debug, Hash, PartialEq, Eq)]
struct PartNum {
    num: u64,
}

impl PartNum {
    pub(crate) fn new(num: u64) -> PartNum {
        PartNum {
            num,
        }
    }
}
