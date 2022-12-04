use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

pub fn pb1() {
    let mut total: u32 = 0;
    let lines = read_lines("./src/day3.txt").unwrap();
    for line in lines {
        if let Ok(str) = line {
            assert!(str.len() % 2 == 0, "{} is not of even length", str);
            let (a, b) = &str.split_at(str.len() / 2);
            total += (to_bitmap(a) & to_bitmap(b)).trailing_zeros();
        }
    }
    println!("{}", total);
}

fn to_bitmap(c: &str) -> u64 {
    return c
        .chars()
        .map(|c| {
            return if c.is_lowercase() {
                (c as u8) - ('a' as u8) + 1
            } else {
                (c as u8) - ('A' as u8) + 27
            };
        })
        .fold(0u64, |acc, x| acc | 1 << x);
}

pub fn pb2() {
    let total: u32 = read_lines("./src/day3.txt")
        .unwrap()
        .map(|s| to_bitmap(&s.unwrap()))
        .array_chunks::<3>()
        .map(|[a, b, c]| a & b & c)
        .map(u64::trailing_zeros)
        .sum();
    println!("{}", total);
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
