use std::cell::RefCell;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;
use std::rc::Rc;

use crate::PartOrSymbol::PART;

fn main() {
    let input = Path::new(file!()).parent().unwrap().join("input.txt").canonicalize().expect("Cannot locate file");
    let input = BufReader::new(File::open(input).expect("Input file missing"));
    let mut index: HashMap<(i32, i32), PartOrSymbol> = HashMap::new();
    let mut nums: Vec<Rc<PartNum>> = Vec::new();

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
                    nums.push(num.clone());
                    for j_1 in j..(num_end as i32) {
                        index.insert((i, j_1), PART(num.clone()));
                    }
                    for j_2 in (j - 1)..=(num_end as i32) {
                        if let Some(p) = index.get(&((i - 1), j_2))
                        {
                            if let PartOrSymbol::SYMBOL = p {
                                num.mark_valid()
                            }
                        }
                    }
                    if let Some(p_or_s) = index.get(&(i, (j - 1)))
                    {
                        if let PartOrSymbol::SYMBOL = p_or_s {
                            num.mark_valid();
                        }
                    }
                    j = num_end as i32 - 1;
                }
                b'.' => {}
                _ => {
                    index.insert((i, j), PartOrSymbol::SYMBOL);
                    for j_1 in (j - 1)..=(j + 1) {
                        if let Some(p_or_s) = index.get(&((i - 1), j_1))
                        {
                            p_or_s.mark_valid();
                        }
                    }
                    if let Some(p_or_s) = index.get(&(i, (j - 1)))
                    {
                        p_or_s.mark_valid();
                    }
                }
            };
            j += 1;
        }
    }

    let sum: u64 = nums.into_iter().map(|p| p.get_valid_num()).flatten().sum();
    println!("Part num sum {}", sum);
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
enum PartOrSymbol {
    PART(Rc<PartNum>),
    SYMBOL,
}

#[derive(Debug)]
struct PartNum {
    is_valid: RefCell<bool>,
    num: u64,
}

impl PartOrSymbol {
    fn mark_valid(&self) {
        if let PART(num) = self {
            num.mark_valid();
        }
    }
}

impl PartNum {
    pub(crate) fn new(num: u64) -> PartNum {
        PartNum {
            is_valid: RefCell::new(false),
            num,
        }
    }

    pub(crate) fn mark_valid(&self) {
        self.is_valid.replace(true);
    }

    fn get_valid_num(&self) -> Option<u64> {
        if *self.is_valid.borrow() {
            Some(self.num)
        } else {
            None
        }
    }
}

