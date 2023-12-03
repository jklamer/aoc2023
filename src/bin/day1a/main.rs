use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

fn main() {
    let input = Path::new(file!()).parent().unwrap().join("input.txt").canonicalize().expect("Cannot locate file");
    let input = BufReader::new(File::open(input).expect("Input file missing"));

    let mut sum = 0u64;
    for line in input.lines() {
        let line = line.expect("Io error reading line");
        if line.is_empty() {
            continue;
        }
        let mut first_found = false;
        let mut last_found= 0;
        for char in line.chars() {
            if char.is_ascii_digit() {
                let digit  = char.to_digit(10).unwrap();
                if !first_found {
                    sum += digit as u64 * 10;
                    first_found = true;
                }
                last_found = digit
            }
        }
        sum += last_found as u64;
    }
    println!("Sum is {}", sum)
}