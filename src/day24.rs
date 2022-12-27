use itertools::Itertools;
use std::collections::{HashSet, VecDeque};
// off by 1, ot resolve later!

pub fn pb1() {
    let world = parse(&INPUT);
    let end = (world.x - 1, world.y - 2);
    let r = solve(&world, (0, 1), end, 0);
    assert_eq!(r, 322);
}

pub fn pb2() {
    let world = parse(&INPUT);
    let start = (0, 1);
    let end = (world.x - 1, world.y - 2);
    let r_end = solve(&world, start, end, 1);
    let r_beginning = solve(&world, end, start, r_end + 1);
    let r_final = solve(&world, start, end, r_beginning + 1);
    dbg!(r_final);
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum T {
    U,
    D,
    L,
    R,
    E,
    W,
}

fn from_char(c: char) -> T {
    match c {
        '#' => T::W,
        '.' => T::E,
        '>' => T::R,
        '<' => T::L,
        '^' => T::U,
        'v' => T::D,
        _ => panic!(),
    }
}
struct World {
    x: usize,
    y: usize,
    w: Vec<Vec<T>>,
}
fn solve(world: &World, start: (usize, usize), end: (usize, usize), round: isize) -> isize {
    let mut to_scan = VecDeque::new();
    let mut seen: HashSet<((usize, usize), isize)> = HashSet::new();

    to_scan.push_back((start, round));
    while let Some((curr_pos, round)) = to_scan.pop_front() {
        for delta in [(-1, 0), (1, 0), (0, -1), (0, 1), (0, 0)] {
            let (x, y) = (curr_pos.0 as isize + delta.0, curr_pos.1 as isize + delta.1);
            if x < 0 || y < 0 {
                continue;
            }
            let x = x as usize;
            let y = y as usize;
            if x >= world.x || y >= world.y {
                continue;
            }
            if (x, y) == start && delta != (0, 0) {
                continue;
                // cant go back to the start
            }

            if (x, y) == end {
                return round + 1;
            }

            if can_move(x, y, round + 1, world) {
                let new = ((x, y), round + 1);
                if !seen.contains(&new) {
                    to_scan.push_back(new);
                    seen.insert(new);
                }
            }
        }
    }
    panic!("no solution");
}

fn can_move(x: usize, y: usize, round: isize, world: &World) -> bool {
    world.w[x][y] != T::W
        && !has_wind_up(x, y, round, world)
        && !has_wind_down(x, y, round, world)
        && !has_wind_left(x, y, round, world)
        && !has_wind_right(x, y, round, world)
}

// need to check those
fn has_wind_up(x: usize, y: usize, round: isize, world: &World) -> bool {
    let x_round = (x as isize - 1 + round).rem_euclid(world.x as isize - 2) + 1;
    world.w[x_round as usize][y] == T::U
}

fn has_wind_down(x: usize, y: usize, round: isize, world: &World) -> bool {
    let x_round = (x as isize - 1 - round).rem_euclid(world.x as isize - 2) + 1;
    world.w[x_round as usize][y] == T::D
}

fn has_wind_left(x: usize, y: usize, round: isize, world: &World) -> bool {
    let y_round = (y as isize - 1 + round).rem_euclid(world.y as isize - 2) + 1;
    world.w[x][y_round as usize] == T::L
}

fn has_wind_right(x: usize, y: usize, round: isize, world: &World) -> bool {
    let y_round = (y as isize - 1 - round).rem_euclid(world.y as isize - 2) + 1;
    world.w[x][y_round as usize] == T::R
}
fn parse(input: &str) -> World {
    let w = input
        .lines()
        .map(|l| l.chars().map(from_char).collect_vec())
        .collect_vec();
    let y = w.first().unwrap().len();
    let x = w.len();
    World { x, y, w }
}

#[allow(dead_code)]
fn display(world: &World, round: isize, pos: (usize, usize)) {
    println!("");
    println!("Display world at min {round}");
    (0..world.x).for_each(|x| {
        let line = (0..world.y)
            .map(|y| {
                if x == pos.0 && y == pos.1 {
                    return 'E';
                }
                if world.w[x][y] == T::W {
                    return '#';
                }
                match (
                    has_wind_up(x, y, round, world),
                    has_wind_down(x, y, round, world),
                    has_wind_left(x, y, round, world),
                    has_wind_right(x, y, round, world),
                ) {
                    (false, false, false, false) => '.',
                    (true, false, false, false) => '^',
                    (false, true, false, false) => 'v',
                    (false, false, true, false) => '<',
                    (false, false, false, true) => '>',
                    _ => 'x',
                }
            })
            .collect::<String>();
        println!("{}", line);
    });
}

#[allow(dead_code)]
const INPUT_TEST: &str = "\
#.######
#>>.<^<#
#.<..<<#
#>v.><>#
#<^v^^>#
######.#";

#[allow(dead_code)]
const INPUT: &str = "\
#.######################################################################################################################################################
#.v><><<>^^^<><<>.^v<>v^>v^v^^<..^v>^>^vv.>v^<<>^.v<vvv<.>><v>><^>><>^<><v<>>>><v>v<^<>^.v..v^vv<v<<vv.v>>v<>v.<..^<vv<^v^v.v>>v>><^v>^<.vv^^<<><<>v>><#
#.vv<vv<>.v<<<<<>>vv<>^v^^<^<>><.<^>^<^>^<v<<<<v>v>><.><.^><>vv<^^><<<<>^<.<v^.vv^vv>v>^.<<<><v<.v.<><<v^^v>^^<^>>^.<v^v.v><<v^vv<<v>^v>><^^<^<v>^>><v<#
#<v^>v^vv<><v><<^<>^<^vv>v>>^^><<<vv><v.v.^><<^><>v>^<^v<<^>v<vv><v<<^.vv^>>>.^v>v.><v>v<^<^^<><<>><<^^^>v<^>>^v>>vv.<>^<><<^v>><^.>v^v><<^^^^<v<>^v<v.#
#<<^<vv^<^><>^><>^vv.v^v>><^^<>^<>>^><^.^^<.v...>^^.^<<v<vvv>>>v^..^v.v<^><>.>v^.vv<^^vv>vv^.^.vv<v>>^>v.^^^<v<>><vv^>>^^.v>.v^^<^>vvv^<<v^<v<^><^>v<>>#
#<.<^>>^<v><^>^^.>>v^<v^vvv>>vv.v^<>^^>v<<vv^<>^v.<<<^>>>.^>vvv.^^^<^><<^><v>><^.^>vv^^vv.><<v<^<v^^^.>v>^>v.^^<<^^^>^><><<vv<.<>v>vv>><^^v>>vvv^<...<<#
#><v.v>vv>^^^>>><v>^^v>^>>><vv^<^<^<<<>.<<.<.<..><><v.v^^.v..v^>>>^v><.^^><<>v^<^<>^>>.v^v.v.>v>^>>>v>>^>><^<.v.v<^<<<<<^^>^^^^>^^<v^>>v><>.<>v><>.>><>#
#<><<.>^<>^^v<v>^><^>v^.>><vv^vv^<v<<>.>.>>.vv.v<.^<^<>>^><><^.v>>v^^v><>><<<vv<<>v><>^.>>^>v^^><^<v<<><vv^>>.<^<.^<^>vvv>^<.v<^<>.><.>>v<^>^v<<.^<v>^<#
#<>.<v^>.^<<^v^.<<^>v^^vv<^><^<^>><v^^vvv<<vvvv>>^v><<>^>vvvvv^<<v^.>^<.<^>.^>^><v^<>>.<<^^vv>>>^>vv><vvv^v.>^vv><<<<>^v^^^.>.^>>^^v^v>v>vv^<<^vv^<>v<>#
#<<>v>v>>v^^v><^<.vv<^>^^>.v^<.^^vv>.<>^<>v>^>vvv<vv^>v^><<<v>vv<..^v^v>v.v><^.<^>.v<v><v>v^>^<<<<<>^.v<^.<v.<<^>v^^^<^<<>v.<<^<<<<.<^><<<^<v<v>>v>.><>#
#>>.>vvv<<..>^^vv^>.<.v..v^^>^v<<>>>vv.<v>vv^<vv^v><v^^.<<.><..><^><<>^<<><v<.<v.<^<^.^<^<<><><<^v^>.<<v^^>.^<...v.^<^^<><><>v^<><^>^>^v^.>^^>^.vv.<v^>#
#>.<><v>v<v>v>^v^^v>.v^.v>v>.<>>>^vv<<v>>^>^.^<^vv.vvvv<<>v<^^.>v<^v<<^><^.^^v<>.v<vv><v<v>vvv>^^>v>..<<<v..>v..<^^^<v>^^<<vv<>>^^v^.vv<><^>>.^<<^>>v.>#
#><v..vv^<v^<>>v<>..<^>v^^><.>vv^vv^<^^v<>vv<.vv^^<>^>>^v<^^<<>^vv>v><>v<^vvv^<>><>^^^v^<v.v^<<^^>>>>><^^>><^v>>vv^v><^>^<<^.v>vv>^.v^<vv^v>>>vv>v^v<^>#
#...>><v.<v<>^.v.v..^>^<><vv^><<<vv^<vv.><^.vv>>><v^v^>^v>>v<.^<^^<v^<>v<^<><<<.^<<vv.<>>^v^^.v>.<v>^.^>v^.v><^<<>>>^.<<<^v<<<><^>.^.^v><>^>><vv<v.<.^>#
#<^vv>>v>v<^><^^^<^>vv<>vv>^^^.><><^>v>><v.<>vv^^<^<<<v^<><v>v^v>^<><><><><>^^>><>..^^>v<...vv<^>><.vv<^<v><v<vv^.><>v^<>v.^>>>v<<^<<^vv.>^<.^<>^<v^^^>#
#>>^vv^.<<.<.<>>vvv<<<..<v><^<>^><<>^.<<^^^<v^<<v.<vv<^<>>vv^>.v^v<^<>>vv<<^>>.v>.v>^^^vvv^v<^><v^><>><^<.>v^^^<vv^>^v<v^<vv><>..^><.>^^<^>>..^.v^<^<v>#
#<<><^^><v^v<^^<vv>><v..>>>^><>>>v>v>v>v><>>^vv><>>v>>vvvv<<^><>v^>vv>^.>>v.^<v<^.>>>^^v>>><vvv>><.<v<^<v<^v<v<<vvv..^^<<^<.<><<<<^<>v<<<^^>.^v>^^v<^^<#
#>v^^^<v>^^<v<<<vv><^v.v<v^^^vvv<<><><^^><^^>>>>>><^>^^.>vv>^><<<v<<<^vv.^^^<v>v<^^^^>.^^vv<^<.<v^<vv^^^>><..><^>v><<v>v>^v^>>^^>vv.v^^^>^^vv.<<v.^><<.#
#<^v<<^^vvv.>v>v>.v><v<^^>^>^>^^v>v^vv^<.^.>>>>>v<>.<<.<^<<vvv..<^>^><^>^<^<<>><^>v<v^v>^v^^^>>>>v>^>>^<v..><.>>^^><^<^.<v^>^.v>^^vv^v.<>>>>.v.<<>^^vv>#
#<v^v.>.>^v>^<v.<^<^>v^>^^.>.>v<>>>^>vv<>v<>^>v^<<^<.<<^>v^<><>><^^<^^v<>>vv<<.<>v>.>^^^>><^^<^vv.><v<.<<^^>^>><>v><^vv.>v^v^v..<vvv..<<v<v^<<v>^<^>><<#
#<^v<v^^v^>^<>v^<^<>>..>><>v<.^<.>v.>^v.>>^^<^v>>v^.v>^><>>vv.<<<>v>^.>v>..<<<v.<v^<>^>^<>v<^<v<><^>^v><.<<^^^<^.<<<^.^v^^.>^.<<vv>^v><<>^<^v^^>^v><<^>#
######################################################################################################################################################.#";
