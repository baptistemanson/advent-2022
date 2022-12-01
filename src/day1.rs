use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

pub fn pb11() {
    // File hosts must exist in current path before this produces output
    let mut total: i32 = 0;
    let mut max = 0;
    let lines = read_lines("./src/day1.txt").unwrap();
    // Consumes the iterator, returns an (Optional) String
    for line in lines {
        if let Ok(str) = line {
            if str.is_empty() {
                total = 0;
            } else {
                total = total + str.parse::<i32>().unwrap();
            }
        }
        // including the last one
        if max < total {
            max = total
        }
    }
    println!("{}", max);
}

// This implementation is trying to limit the memory space needed
// only keeping the top 3 results.
pub fn pb12() {
    let mut total: i32 = 0;
    let mut top = [0, 0, 0];
    let lines = read_lines("./src/day1.txt").unwrap();
    // no memory is allocated during the loop
    for line in lines {
        if let Ok(str) = line {
            if str.is_empty() {
                total = 0;
            } else {
                total = total + str.parse::<i32>().unwrap();
            }
        }
        // including the last iteration
        if top[0] < total {
            // if total is greater than the current top 3
            // the top contains top 1, top 2 and total,
            // in any order
            top = [total, top[1], top[2]];
            top.sort();
        }
    }
    println!("{}", top[0] + top[1] + top[2]);
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
