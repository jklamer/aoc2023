use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

fn main() {
    let input = Path::new(file!()).parent().unwrap().parent().unwrap().join("day1a/input.txt");
    let input = BufReader::new(File::open(dbg!(input)).expect("Input file missing"));

    let mut sum = 0u64;
    for line in input.lines() {
        let line = line.expect("Io error reading line");
        if line.is_empty() {
            continue;
        }
        let mut first_found = false;
        let mut last_found = 0;
        let line = line + "xxxx";
        let line = line.as_bytes();
        for window in line.windows(5) {
            let digit = match window {
                [b'o', b'n', b'e', ..] => {
                    Some(1u64)
                }
                [b't', b'w', b'o', ..] => {
                    Some(2u64)
                }
                [b't', b'h', b'r', b'e', b'e'] => {
                    Some(3u64)
                }
                [b'f', b'o', b'u', b'r', ..] => {
                    Some(4u64)
                }
                [b'f', b'i', b'v', b'e', ..] => {
                    Some(5u64)
                }
                [b's', b'i', b'x', ..] => {
                    Some(6u64)
                }
                [b's', b'e', b'v', b'e', b'n'] => {
                    Some(7u64)
                }
                [b'e', b'i', b'g', b'h', b't'] => {
                    Some(8u64)
                }
                [b'n', b'i', b'n', b'e', ..] => {
                    Some(9u64)
                }
                _ => {
                    if window[0].is_ascii_digit() {
                        Some((window[0]as char).to_digit(10).unwrap() as u64)
                    } else {
                        None
                    }
                }
            };
            if let Some(digit) = digit {
                if !first_found {
                    sum += digit * 10;
                    first_found = true;
                }
                last_found = digit
            }
        }
        sum += last_found;
    }
    println!("Sum is {}", sum)
}