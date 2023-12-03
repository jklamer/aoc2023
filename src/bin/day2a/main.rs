use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

fn main() {
    let input = Path::new(file!()).parent().unwrap().join("input.txt").canonicalize().expect("Cannot locate file");
    let input = BufReader::new(File::open(input).expect("Input file missing"));

    let greens_allowed = 13;
    let reds_allowed = 12;
    let blues_allowed = 14;
    let mut sum = 0;
    for game in input.lines() {
        let game = game.expect("Io reading line");
        let (game_num, cube_count) = game.split_once(":").expect("input colon formatting off");
        let game_num: i32 = game_num.trim_start_matches("Game ").trim().parse().expect("num expected");
        let mut possible_game = true;
        'game: for handful in cube_count.split(";") {
            for count in handful.split(",") {
                let (count, color) = count.trim().split_once(" ").expect("no space");
                let count: i32 = count.trim().parse().unwrap();
                possible_game &= match color {
                    "blue" => count <= blues_allowed,
                    "red" => count <= reds_allowed,
                    "green" => count <= greens_allowed,
                    _ => true
                };
                if !possible_game {
                    break 'game;
                }
            }
        }
        if possible_game {
            sum += game_num;
        }
    }
    println!("Possible game sum {}", sum)
}