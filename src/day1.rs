use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

pub fn pb11() {
    // File hosts must exist in current path before this produces output
    let mut total: i32 = 0;
    let mut max = 0;
    if let Ok(lines) = read_lines("./src/day1.txt") {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(str) = line {
                if str.is_empty() {
                    total = 0;
                } else {
                    total = total + str.parse::<i32>().unwrap();
                    if max < total {
                        max = total
                    }
                }
            }
        }
        if max < total {
            max = total
        }
        println!("{}", max);
    } else {
        panic!("cannot open file")
    }
}
pub fn pb12() {
    // File hosts must exist in current path before this produces output
    let mut total: i32 = 0;
    let mut max1 = 0;
    let mut max2 = 0;
    let mut max3 = 0;
    if let Ok(lines) = read_lines("./src/day1.txt") {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(str) = line {
                if str.is_empty() {
                    total = 0;
                } else {
                    total = total + str.parse::<i32>().unwrap();
                    if max3 < total {
                        [max1, max2, max3] = sort3(max1, max2, total);
                    }
                }
            }
        }
        if max3 < total {
            [max1, max2, max3] = sort3(max1, max2, total);
        }
        println!("{}", max1 + max2 + max3);
    } else {
        panic!("cannot open file")
    }
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

fn sort3(a: i32, b: i32, c: i32) -> [i32; 3] {
    if a > b {
        if a > c {
            if b > c {
                [a, b, c]
            } else {
                [a, c, b]
            }
        } else {
            [c, a, b]
        }
    } else {
        if a > c {
            [b, a, c]
        } else {
            if b > c {
                [b, c, a]
            } else {
                [c, b, a]
            }
        }
    }
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_order() {
        assert_eq!(sort3(0, 1, 2), [2, 1, 0]);
        assert_eq!(sort3(0, 1, 5), [5, 1, 0]);
        assert_eq!(sort3(0, 5, 2), [5, 2, 0]);
        assert_eq!(sort3(1, 1, 2), [2, 1, 1]);
        assert_eq!(sort3(3, 1, 2), [3, 2, 1]);
    }
}
