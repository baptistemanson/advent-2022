type Stacks = Vec<Vec<char>>;
type Command = (usize, usize, usize);

pub fn pb1() {
    let (stacks_desc, commands) = INPUT.split_once("\n\n").unwrap();
    let mut stacks: Stacks = build_stack(stacks_desc.lines().collect::<Vec<&str>>());
    commands
        .lines()
        .map(parse_command)
        .for_each(|(iter, src, dest)| {
            for _ in 0..iter {
                let tail = stacks[src].pop().unwrap();
                stacks[dest].push(tail);
            }
        });
    dbg!(stacks.iter().map(|s| s.last().unwrap()).collect::<String>());
}

pub fn pb2() {
    let (stacks_desc, commands) = INPUT.split_once("\n\n").unwrap();
    let mut stacks: Stacks = build_stack(stacks_desc.lines().collect::<Vec<&str>>());
    commands
        .lines()
        .map(parse_command)
        .for_each(|(iter, src, dest)| {
            let s = stacks[src].len() - iter;
            let mut tail = stacks[src].split_off(s);
            stacks[dest].append(&mut tail);
        });
    dbg!(stacks.iter().map(|s| s.last().unwrap()).collect::<String>());
}

fn build_stack(desc: Vec<&str>) -> Stacks {
    let col_to_stack = |col: usize| (col + 1) / 4; // column in the description -> stack id
    let stacks_len = col_to_stack(desc.last().unwrap().len()); // get nb of stacks from last line
    let mut stacks = vec![vec![]; stacks_len];
    for i in (0..desc.len() - 1).rev() {
        for col in (1..desc[i].len()).step_by(4) {
            let letter = desc[i].chars().nth(col).unwrap();
            if letter != ' ' {
                stacks[col_to_stack(col)].push(letter);
            }
        }
    }
    return stacks;
}

fn parse_command(line: &str) -> Command {
    let mut split = line.split(" ");
    (
        split.nth(1).unwrap().parse::<usize>().unwrap(),
        split.nth(1).unwrap().parse::<usize>().unwrap() - 1,
        split.nth(1).unwrap().parse::<usize>().unwrap() - 1,
    )
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
