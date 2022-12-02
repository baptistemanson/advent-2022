use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
// A X Rock
// B Y Paper
// C Z Scissors

pub fn pb1() {
    let mut score: u32 = 0;
    let x_score = 1;
    let y_score = 2;
    let z_score = 3;
    let lose_score = 0;
    let draw_score = 3;
    let win_score = 6;
    let lines = read_lines("./src/day2.txt").unwrap();
    for line in lines {
        if let Ok(str) = line {
            score += match str.as_str() {
                "A X" => x_score + draw_score,
                "A Y" => y_score + win_score,
                "A Z" => z_score + lose_score,
                "B X" => x_score + lose_score,
                "B Y" => y_score + draw_score,
                "B Z" => z_score + win_score,
                "C X" => x_score + win_score,
                "C Y" => y_score + lose_score,
                "C Z" => z_score + draw_score,
                _ => panic!("uknown combination"),
            };
        }
    }
    println!("{}", score);
}

// This implementation is trying to limit the memory space needed
// only keeping the top 3 results.
pub fn pb2() {
    let mut score: u32 = 0;
    let x_score = 0;
    let y_score = 3;
    let z_score = 6;
    let a_score = 1; // rock
    let b_score = 2; // paper
    let c_score = 3; // scissor
    let lines = read_lines("./src/day2.txt").unwrap();
    for line in lines {
        if let Ok(str) = line {
            score += match str.as_str() {
                "A X" => x_score + c_score, // Rock > Scissor
                "A Y" => y_score + a_score,
                "A Z" => z_score + b_score,
                "B X" => x_score + a_score, // Paper > Rock
                "B Y" => y_score + b_score,
                "B Z" => z_score + c_score,
                "C X" => x_score + b_score, // Scissor > Paper
                "C Y" => y_score + c_score,
                "C Z" => z_score + a_score,
                _ => panic!("uknown combination"),
            };
        }
    }
    println!("{}", score);
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
