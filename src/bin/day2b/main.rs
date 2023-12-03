use std::cmp::max;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

fn main() {
    let input = Path::new(file!()).parent().unwrap().parent().unwrap().join("day2a/input.txt");
    let input = BufReader::new(File::open(input).expect("Input file missing"));

    let mut sum = 0;
    for game in input.lines() {
        let game = game.expect("IO reading line");
        let (_, cube_count) = game.split_once(":").expect("input colon formatting off");
        let mut min_green = 0;
        let mut min_blue = 0;
        let mut min_red = 0;

        for handful in cube_count.split(";") {
            for count in handful.split(",") {
                let (count, color) = count.trim().split_once(" ").expect("no space");
                let count: i32 = count.trim().parse().unwrap();
                match color {
                    "blue" => min_blue = max(min_blue, count),
                    "red" => min_red = max(min_red, count),
                    "green" => min_green = max(min_green, count),
                    &_ => unimplemented!()
                };
            }
        }
        sum += min_green * min_blue * min_red;
    }
    println!("Power set sum {}", sum)
}