use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

pub fn pb11() {
    let mut max = 0;
    let mut total: i32 = 0;
    let lines = read_lines("./src/day1.txt").unwrap();
    // Consumes the iterator, returns an (Optional) String
    for line in lines {
        if let Ok(str) = line {
            total = if str.is_empty() {
                0
            } else {
                total + str.parse::<i32>().unwrap()
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
    let mut top = [0; 3];
    let mut total: i32 = 0;
    let lines = read_lines("./src/day1.txt").unwrap();
    for line in lines {
        if let Ok(str) = line {
            total = if str.is_empty() {
                0
            } else {
                total + str.parse::<i32>().unwrap()
            }
        }
        // including the last iteration
        // this could be only be executed when we reset total
        // and when we finish the loop
        // but clarity before performance
        if top[0] < total {
            // if total is greater than the current smallest top
            // then the new top contains total and all the others,
            // in any order
            top[0] = total;
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
