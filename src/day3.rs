use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

pub fn pb1() {
    let mut total: u32 = 0;
    let lines = read_lines("./src/day3.txt").unwrap();
    for line in lines {
        if let Ok(str) = line {
            let first_half = &str[0..str.len() / 2];
            let second_half = &str[str.len() / 2..];
            total += to_priority(find_missing(first_half, second_half)) as u32;
        }
    }
    println!("{}", total);
}

fn to_priority(c: char) -> u8 {
    return if c.is_lowercase() {
        (c as u8) - ('a' as u8) + 1
    } else {
        (c as u8) - ('A' as u8) + 27
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_prio() {
        assert_eq!(to_priority('p'), 16);
        assert_eq!(to_priority('L'), 38);
        assert_eq!(to_priority('P'), 42);
        assert_eq!(to_priority('v'), 22);
        assert_eq!(to_priority('t'), 20);
    }
}

fn find_missing(a: &str, b: &str) -> char {
    for to_find in a.chars() {
        if b.contains(to_find) {
            return to_find;
        }
    }
    panic!("couldnt find")
}

fn find_missing_3(a: &str, b: &str, c: &str) -> char {
    for to_find in a.chars() {
        if b.contains(to_find) {
            if c.contains(to_find) {
                return to_find;
            }
        }
    }
    panic!("couldnt find")
}
// This implementation is trying to limit the memory space needed
// only keeping the top 3 results.
pub fn pb2() {
    let mut total: u32 = 0;
    let mut i = 0;
    let mut l = [String::from(""), String::from(""), String::from("")];
    let lines = read_lines("./src/day3.txt").unwrap();
    for line in lines {
        if let Ok(str) = line {
            l[i % 3] = str;
            if i % 3 == 2 {
                total += to_priority(find_missing_3(&l[0], &l[1], &l[2])) as u32;
            }
            i += 1;
        }
    }
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
