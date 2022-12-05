use lazy_static::lazy_static;
use regex::Regex;

pub fn pb1() {
    let mut stacks: Vec<Vec<char>> = vec![];
    let mut stacks_as_str: Vec<&str> = vec![];
    let mut is_building_stacks = true;
    for line in INPUT.lines() {
        if is_building_stacks {
            if line.chars().nth(1) != Some('1') {
                stacks_as_str.push(line);
            } else {
                is_building_stacks = false;
                build_stacks(line, &mut stacks, &stacks_as_str);
            }
        } else {
            let (iter, source, dest) = parse(line);
            for _ in 0..iter {
                move_crate(&mut stacks, source, dest);
            }
        }
    }
    let top = stacks.iter().map(|s| s[s.len() - 1]).collect::<String>();
    dbg!(top);
}

fn move_crate(stacks: &mut Vec<Vec<char>>, source: usize, dest: usize) {
    let c = stacks[source - 1].pop().unwrap();
    stacks[dest - 1].push(c);
}

fn build_stacks(line: &str, stacks: &mut Vec<Vec<char>>, lines: &Vec<&str>) {
    let nb_stacks = (line.len() + 1) / 4;
    for _ in 0..nb_stacks {
        stacks.push(vec![]);
    }
    for i in (0..lines.len()).rev() {
        for c in (1..lines[i].len()).step_by(4) {
            let letter = lines[i].chars().nth(c).unwrap();
            if letter != ' ' {
                stacks[(c + 1) / 4].push(letter);
            }
        }
    }
}

fn parse(line: &str) -> (usize, usize, usize) {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"move (\d+) from (\d+) to (\d+)").unwrap();
    }
    if line.len() == 0 {
        return (0, 0, 0);
    }
    let matches = RE.captures(line).unwrap();
    return (
        matches[1].parse::<usize>().unwrap(),
        matches[2].parse::<usize>().unwrap(),
        matches[3].parse::<usize>().unwrap(),
    );
}

// This implementation is trying to limit the memory space needed
// only keeping the top 3 results.
pub fn pb2() {
    let mut stacks: Vec<Vec<char>> = vec![];
    let mut stacks_as_str: Vec<&str> = vec![];
    let mut is_building_stacks = true;
    for line in INPUT.lines() {
        if is_building_stacks {
            if line.chars().nth(1) != Some('1') {
                stacks_as_str.push(line);
            } else {
                is_building_stacks = false;
                build_stacks(line, &mut stacks, &stacks_as_str);
            }
        } else {
            let (iter, source, dest) = parse(line);
            if iter == 0 {
                continue;
            }
            let split_point = stacks[source - 1].len() - iter;
            let mut tail = stacks[source - 1].split_off(split_point);
            stacks[dest - 1].append(&mut tail);
        }
    }
    let top = stacks.iter().map(|s| s[s.len() - 1]).collect::<String>();
    dbg!(top);
}

#[allow(dead_code)]
const INPUT_DBG: &str = "
    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2";

#[allow(dead_code)]
const INPUT: &str = "
[P]     [C]         [M]            
[D]     [P] [B]     [V] [S]        
[Q] [V] [R] [V]     [G] [B]        
[R] [W] [G] [J]     [T] [M]     [V]
[V] [Q] [Q] [F] [C] [N] [V]     [W]
[B] [Z] [Z] [H] [L] [P] [L] [J] [N]
[H] [D] [L] [D] [W] [R] [R] [P] [C]
[F] [L] [H] [R] [Z] [J] [J] [D] [D]
 1   2   3   4   5   6   7   8   9 

move 4 from 9 to 1
move 6 from 3 to 1
move 7 from 4 to 1
move 2 from 8 to 5
move 1 from 9 to 7
move 1 from 8 to 5
move 3 from 6 to 4
move 6 from 1 to 5
move 14 from 1 to 2
move 1 from 6 to 1
move 2 from 6 to 2
move 9 from 5 to 9
move 2 from 4 to 5
move 2 from 5 to 3
move 6 from 9 to 6
move 4 from 1 to 2
move 2 from 1 to 2
move 5 from 6 to 1
move 1 from 4 to 9
move 4 from 9 to 4
move 2 from 3 to 7
move 2 from 4 to 9
move 2 from 9 to 6
move 5 from 2 to 9
move 1 from 4 to 9
move 1 from 4 to 3
move 5 from 9 to 8
move 1 from 6 to 5
move 3 from 7 to 5
move 2 from 1 to 6
move 5 from 6 to 8
move 1 from 9 to 4
move 1 from 6 to 5
move 9 from 2 to 7
move 1 from 2 to 3
move 1 from 4 to 6
move 8 from 5 to 4
move 1 from 6 to 1
move 2 from 8 to 6
move 1 from 6 to 4
move 7 from 4 to 6
move 1 from 3 to 1
move 1 from 3 to 4
move 3 from 4 to 1
move 2 from 3 to 4
move 2 from 4 to 5
move 3 from 5 to 7
move 7 from 8 to 2
move 5 from 1 to 2
move 12 from 7 to 6
move 2 from 1 to 9
move 2 from 9 to 1
move 1 from 7 to 5
move 6 from 2 to 3
move 5 from 2 to 6
move 6 from 2 to 6
move 4 from 3 to 1
move 3 from 2 to 1
move 1 from 5 to 4
move 7 from 1 to 2
move 1 from 4 to 8
move 7 from 2 to 9
move 5 from 2 to 8
move 2 from 6 to 8
move 21 from 6 to 9
move 8 from 9 to 1
move 2 from 6 to 1
move 3 from 8 to 7
move 6 from 6 to 4
move 7 from 1 to 8
move 1 from 9 to 1
move 7 from 7 to 3
move 1 from 7 to 4
move 1 from 7 to 4
move 7 from 8 to 1
move 5 from 4 to 8
move 10 from 1 to 2
move 3 from 1 to 4
move 3 from 2 to 9
move 1 from 4 to 5
move 3 from 3 to 6
move 1 from 6 to 4
move 1 from 6 to 7
move 1 from 7 to 8
move 7 from 2 to 4
move 10 from 9 to 1
move 10 from 4 to 5
move 2 from 5 to 2
move 2 from 2 to 1
move 11 from 8 to 9
move 7 from 1 to 4
move 1 from 6 to 1
move 1 from 8 to 3
move 1 from 4 to 6
move 6 from 4 to 5
move 1 from 5 to 7
move 1 from 6 to 8
move 6 from 1 to 6
move 19 from 9 to 2
move 1 from 1 to 8
move 1 from 4 to 7
move 9 from 2 to 6
move 1 from 9 to 2
move 2 from 8 to 1
move 1 from 1 to 9
move 7 from 3 to 6
move 3 from 9 to 2
move 5 from 2 to 6
move 1 from 9 to 3
move 15 from 6 to 7
move 6 from 6 to 7
move 1 from 1 to 9
move 5 from 6 to 2
move 1 from 6 to 1
move 6 from 5 to 8
move 1 from 3 to 4
move 1 from 9 to 7
move 6 from 8 to 1
move 3 from 4 to 6
move 1 from 6 to 1
move 3 from 5 to 2
move 1 from 5 to 7
move 5 from 1 to 5
move 2 from 6 to 9
move 2 from 9 to 2
move 7 from 5 to 1
move 1 from 5 to 7
move 1 from 5 to 9
move 20 from 7 to 1
move 23 from 1 to 7
move 1 from 1 to 2
move 4 from 7 to 9
move 4 from 9 to 8
move 1 from 9 to 2
move 16 from 7 to 6
move 4 from 1 to 5
move 9 from 7 to 6
move 11 from 2 to 6
move 1 from 1 to 9
move 1 from 1 to 7
move 1 from 8 to 2
move 1 from 9 to 7
move 4 from 5 to 2
move 3 from 8 to 3
move 2 from 2 to 4
move 2 from 7 to 4
move 4 from 4 to 9
move 28 from 6 to 9
move 5 from 2 to 7
move 8 from 6 to 5
move 6 from 2 to 6
move 2 from 7 to 3
move 5 from 5 to 7
move 1 from 5 to 9
move 14 from 9 to 4
move 18 from 9 to 8
move 5 from 6 to 4
move 6 from 7 to 8
move 1 from 2 to 6
move 19 from 4 to 7
move 1 from 2 to 5
move 1 from 9 to 3
move 2 from 5 to 2
move 14 from 7 to 3
move 1 from 5 to 3
move 12 from 8 to 6
move 6 from 6 to 5
move 4 from 5 to 4
move 21 from 3 to 4
move 10 from 8 to 3
move 2 from 3 to 2
move 7 from 4 to 6
move 2 from 8 to 1
move 2 from 2 to 3
move 5 from 7 to 2
move 2 from 1 to 4
move 3 from 3 to 7
move 2 from 5 to 7
move 2 from 2 to 7
move 2 from 2 to 3
move 7 from 4 to 1
move 3 from 1 to 4
move 3 from 2 to 5
move 2 from 1 to 5
move 7 from 4 to 3
move 15 from 6 to 2
move 1 from 1 to 4
move 1 from 5 to 1
move 14 from 3 to 1
move 9 from 4 to 1
move 5 from 7 to 1
move 1 from 3 to 5
move 1 from 4 to 2
move 20 from 1 to 2
move 17 from 2 to 5
move 1 from 3 to 7
move 5 from 7 to 3
move 6 from 5 to 1
move 3 from 3 to 2
move 10 from 1 to 9
move 3 from 5 to 6
move 12 from 5 to 6
move 1 from 5 to 1
move 15 from 6 to 5
move 13 from 5 to 3
move 1 from 5 to 1
move 10 from 3 to 2
move 3 from 3 to 2
move 1 from 5 to 3
move 2 from 3 to 6
move 1 from 3 to 4
move 2 from 6 to 4
move 3 from 4 to 2
move 8 from 9 to 4
move 8 from 4 to 8
move 7 from 2 to 1
move 5 from 8 to 7
move 2 from 2 to 3
move 13 from 1 to 2
move 2 from 3 to 8
move 2 from 9 to 7
move 3 from 8 to 1
move 2 from 1 to 2
move 2 from 8 to 4
move 6 from 7 to 2
move 3 from 1 to 8
move 1 from 7 to 5
move 24 from 2 to 1
move 2 from 8 to 5
move 15 from 1 to 4
move 1 from 5 to 8
move 9 from 1 to 4
move 2 from 8 to 5
move 26 from 2 to 4
move 1 from 5 to 8
move 1 from 5 to 8
move 50 from 4 to 1
move 1 from 8 to 9
move 1 from 4 to 6
move 1 from 4 to 9
move 22 from 1 to 5
move 1 from 6 to 2
move 1 from 5 to 8
move 1 from 2 to 4
move 1 from 8 to 1
move 28 from 1 to 3
move 2 from 9 to 4
move 21 from 5 to 8
move 1 from 1 to 8
move 1 from 5 to 8
move 1 from 5 to 7
move 3 from 4 to 8
move 1 from 7 to 9
move 1 from 9 to 7
move 20 from 8 to 4
move 2 from 8 to 1
move 1 from 7 to 6
move 2 from 1 to 4
move 27 from 3 to 1
move 4 from 8 to 4
move 1 from 6 to 9
move 19 from 4 to 2
move 5 from 2 to 5
move 1 from 4 to 1
move 1 from 9 to 2
move 17 from 1 to 9
move 1 from 3 to 8
move 15 from 9 to 2
move 2 from 4 to 8
move 2 from 5 to 8
move 2 from 5 to 9
move 3 from 9 to 8
move 9 from 1 to 2
move 2 from 1 to 3
move 4 from 4 to 5
move 2 from 5 to 7
move 1 from 8 to 5
move 2 from 3 to 8
move 4 from 5 to 2
move 1 from 9 to 6
move 5 from 8 to 5
move 1 from 7 to 9
move 29 from 2 to 3
move 1 from 8 to 6
move 1 from 9 to 7
move 2 from 2 to 8
move 2 from 5 to 2
move 2 from 7 to 5
move 4 from 5 to 9
move 1 from 5 to 9
move 10 from 3 to 4
move 10 from 4 to 7
move 1 from 3 to 4
move 5 from 2 to 9
move 5 from 8 to 6
move 1 from 6 to 5
move 2 from 6 to 3
move 4 from 6 to 7
move 1 from 5 to 2
move 2 from 2 to 7
move 5 from 7 to 8
move 8 from 7 to 2
move 6 from 8 to 7
move 14 from 2 to 5
move 3 from 7 to 3
move 1 from 4 to 7
move 2 from 7 to 2
move 3 from 2 to 8
move 3 from 8 to 5
move 8 from 9 to 1
move 3 from 7 to 2
move 2 from 7 to 4
move 17 from 3 to 6
move 8 from 1 to 6
move 16 from 5 to 2
move 1 from 5 to 2
move 1 from 3 to 1
move 21 from 6 to 7
move 1 from 4 to 8
move 7 from 7 to 8
move 1 from 1 to 3
move 11 from 7 to 2
move 7 from 2 to 6
move 8 from 8 to 5
move 2 from 7 to 4
move 4 from 5 to 6
move 8 from 2 to 8
move 17 from 2 to 3
move 4 from 5 to 3
move 7 from 6 to 9
move 2 from 6 to 9
move 1 from 4 to 1
move 1 from 4 to 2
move 3 from 6 to 2
move 1 from 6 to 8
move 1 from 4 to 1
move 1 from 7 to 5
move 10 from 9 to 2
move 1 from 5 to 6
move 1 from 8 to 2
move 1 from 1 to 4
move 12 from 3 to 4
move 1 from 6 to 2
move 2 from 8 to 6
move 1 from 1 to 2
move 1 from 9 to 8
move 2 from 8 to 7
move 6 from 3 to 2
move 1 from 3 to 5
move 8 from 4 to 9
move 22 from 2 to 9
move 7 from 3 to 5
move 3 from 8 to 2
move 2 from 7 to 8
move 3 from 6 to 9
move 1 from 2 to 9
move 1 from 6 to 2
move 4 from 8 to 5
move 5 from 5 to 9
move 1 from 3 to 6
move 1 from 5 to 6
move 2 from 4 to 1
move 2 from 2 to 4
move 4 from 4 to 6
move 1 from 1 to 5
move 5 from 6 to 3
move 35 from 9 to 1
move 4 from 9 to 1
move 1 from 4 to 7
move 3 from 3 to 7
move 37 from 1 to 7
move 2 from 2 to 3
move 3 from 3 to 7
move 1 from 5 to 8
move 2 from 1 to 8
move 2 from 5 to 2
move 1 from 6 to 9
move 16 from 7 to 1
move 5 from 1 to 5
move 3 from 8 to 2
move 10 from 7 to 9
move 6 from 7 to 9
move 3 from 2 to 1
move 4 from 5 to 3
move 2 from 1 to 2
move 5 from 7 to 9
move 5 from 7 to 9
move 5 from 5 to 3
move 8 from 3 to 7
move 6 from 9 to 4
move 8 from 7 to 3
move 2 from 3 to 6
move 1 from 6 to 7
move 1 from 6 to 7
move 5 from 4 to 9
move 3 from 7 to 1
move 2 from 2 to 8
move 1 from 8 to 6
move 6 from 1 to 8
move 1 from 7 to 9
move 1 from 3 to 9
move 4 from 3 to 2
move 8 from 1 to 6
move 1 from 3 to 9
move 5 from 8 to 4
move 2 from 3 to 1
move 1 from 8 to 2
move 4 from 9 to 1
move 2 from 1 to 5
move 1 from 8 to 5
move 11 from 9 to 5
move 1 from 2 to 8
move 10 from 5 to 4
move 1 from 1 to 9
move 3 from 5 to 4
move 5 from 2 to 3
move 1 from 5 to 1
move 9 from 9 to 4
move 1 from 6 to 7
move 1 from 3 to 9
move 4 from 3 to 1
move 1 from 2 to 4
move 1 from 1 to 4
move 1 from 4 to 7
move 5 from 1 to 3
move 1 from 3 to 2
move 1 from 8 to 3
move 3 from 9 to 5
move 1 from 2 to 9
move 4 from 1 to 4
move 1 from 7 to 4
move 2 from 5 to 8
move 1 from 7 to 6
move 4 from 3 to 1
move 1 from 5 to 8
move 1 from 3 to 4
move 22 from 4 to 1
move 11 from 1 to 9
move 2 from 1 to 4
move 11 from 1 to 6
move 8 from 6 to 7
move 1 from 8 to 7
move 7 from 9 to 2
move 6 from 7 to 6
move 2 from 4 to 9
move 2 from 7 to 1
move 14 from 6 to 3
move 2 from 3 to 1
move 3 from 6 to 7
move 6 from 1 to 3
move 8 from 9 to 6
move 7 from 4 to 6
move 7 from 6 to 8
move 1 from 9 to 1
move 2 from 9 to 8
move 4 from 3 to 4
move 1 from 8 to 4
move 1 from 4 to 3
move 6 from 3 to 7
move 7 from 2 to 5
move 8 from 4 to 6
move 1 from 7 to 2
move 1 from 5 to 7
move 6 from 7 to 3
move 1 from 7 to 1
move 8 from 8 to 4
move 8 from 4 to 2
move 3 from 7 to 3
move 6 from 5 to 6
move 15 from 3 to 1
move 21 from 6 to 1
move 4 from 2 to 6
move 5 from 6 to 5
move 1 from 2 to 6
move 1 from 4 to 5
move 1 from 4 to 3
move 1 from 8 to 6
move 4 from 5 to 7
move 18 from 1 to 4
move 2 from 5 to 7
move 6 from 7 to 6
move 1 from 3 to 2
move 6 from 1 to 2
move 3 from 3 to 9
move 3 from 9 to 4
move 1 from 8 to 3
move 1 from 6 to 5
move 6 from 2 to 5
move 1 from 5 to 9
move 1 from 3 to 5
move 2 from 6 to 8
move 2 from 1 to 4
move 5 from 4 to 6
move 15 from 4 to 9
move 5 from 9 to 1
move 2 from 6 to 2
move 6 from 6 to 3
move 1 from 8 to 6
move 6 from 5 to 9
move 3 from 6 to 5
move 2 from 4 to 7";
