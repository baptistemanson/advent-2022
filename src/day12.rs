use core::panic;
use itertools::Itertools;
type Pos = (isize, isize);
type Heights = Vec<Vec<u8>>;

pub fn pb1() {
    let (start, end, heights) = parse_input(INPUT);
    let check = |_: Pos, neighbour: Pos| -> bool { neighbour == start };
    println!("{}", navigate(end, &heights, check));
}

pub fn pb2() {
    let (_, end, heights) = parse_input(INPUT);
    let check = |pos: Pos, neighbour: Pos| -> bool {
        get(neighbour, &heights) == 0 && is_within_one(pos, neighbour, &heights)
    };
    println!("{}", navigate(end, &heights, check));
}

fn navigate<F: Fn(Pos, Pos) -> bool>(end: Pos, heights: &Heights, check: F) -> u16 {
    let dim = (heights.len() as isize, heights[0].len() as isize);
    let mut dists = vec![vec![0 as u16; dim.1 as usize]; dim.0 as usize];
    let mut curr_dist: u16 = 1;
    let mut to_scan = vec![end];
    loop {
        let mut next = vec![];
        for pos in to_scan {
            for n_pos in [(-1, 0), (0, -1), (0, 1), (1, 0)] {
                let n_pos = match add_within_bounds(pos, n_pos, dim) {
                    Some(value) => value,
                    None => continue,
                };
                if check(pos, n_pos) {
                    return curr_dist;
                }
                // not end, not already set, one height diff max => register
                if n_pos != end && get(n_pos, &dists) == 0 && is_within_one(pos, n_pos, &heights) {
                    dists[n_pos.0 as usize][n_pos.1 as usize] = curr_dist;
                    next.push(n_pos);
                }
            }
        }
        curr_dist += 1;
        to_scan = next;
        if to_scan.len() == 0 {
            panic!("no solution");
        }
    }
}

fn is_within_one(pos: Pos, neighbour: Pos, heights: &Heights) -> bool {
    get(pos, heights) as i16 - get(neighbour, heights) as i16 <= 1
}

fn get<T: Copy>(pos: Pos, a: &Vec<Vec<T>>) -> T {
    a[pos.0 as usize][pos.1 as usize]
}

fn add_within_bounds(a: Pos, b: Pos, dim: Pos) -> Option<Pos> {
    let n = (a.0 + b.0, a.1 + b.1);
    // out of bounds
    if n.0 < 0 || n.1 < 0 || n.0 >= dim.0 || n.1 >= dim.1 {
        return None;
    }
    Some(n)
}

fn parse_input(input: &str) -> (Pos, Pos, Heights) {
    let mut start: Pos = (0, 0);
    let mut end: Pos = (0, 0);
    let out = input
        .lines()
        .enumerate()
        .map(|(x, l)| {
            l.chars()
                .enumerate()
                .map(|(y, c)| match c {
                    'S' => {
                        start = (x as isize, y as isize);
                        0
                    }
                    'E' => {
                        end = (x as isize, y as isize);
                        'z' as u8 - 'a' as u8
                    }
                    o => o as u8 - 'a' as u8,
                })
                .collect_vec()
        })
        .collect_vec();
    (start, end, out)
}

#[allow(dead_code)]
const INPUT_TEST: &str = "\
Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi";

#[allow(dead_code)]
const INPUT: &str = "\
abaacccccccccccccaaaaaaaccccccccccccccccccccccccccccccccccaaaaaa
abaaccccccccccccccaaaaaaaaaaccccccccccccccccccccccccccccccccaaaa
abaaaaacccccccccaaaaaaaaaaaaccccccccccccccccccccccccccccccccaaaa
abaaaaaccccccccaaaaaaaaaaaaaacccccccccccccccccdcccccccccccccaaaa
abaaaccccccccccaaaaaaaaccacacccccccccccccccccdddcccccccccccaaaaa
abaaacccccccccaaaaaaaaaaccaaccccccccccccciiiiddddcccccccccccaccc
abcaaaccccccccaaaaaaaaaaaaaaccccccccccciiiiiijddddcccccccccccccc
abccaaccccccccaccaaaaaaaaaaaacccccccccciiiiiijjddddccccaaccccccc
abccccccccccccccaaacaaaaaaaaaaccccccciiiiippijjjddddccaaaccccccc
abccccccccccccccaacccccaaaaaaacccccciiiippppppjjjdddddaaaaaacccc
abccccccccccccccccccccaaaaaaccccccckiiippppppqqjjjdddeeeaaaacccc
abccccccccccccccccccccaaaaaaccccckkkiippppuupqqjjjjdeeeeeaaccccc
abccccccccccccccccccccccccaaccckkkkkkipppuuuuqqqjjjjjeeeeeaccccc
abccccccccccccccccccccccccccckkkkkkoppppuuuuuvqqqjjjjjkeeeeccccc
abcccccccccccccccccccccccccckkkkooooppppuuxuvvqqqqqqjkkkeeeecccc
abccaaccaccccccccccccccccccckkkoooooopuuuuxyvvvqqqqqqkkkkeeecccc
abccaaaaacccccaaccccccccccckkkoooouuuuuuuxxyyvvvvqqqqqkkkkeecccc
abcaaaaacccccaaaacccccccccckkkooouuuuxxxuxxyyvvvvvvvqqqkkkeeeccc
abcaaaaaaaaaaaaacccccccccccjjjooottuxxxxxxxyyyyyvvvvrrrkkkeecccc
abcccaaaacaaaaaaaaacaaccccccjjoootttxxxxxxxyyyyyyvvvrrkkkfffcccc
SbccaacccccaaaaaaaaaaaccccccjjjooottxxxxEzzzyyyyvvvrrrkkkfffcccc
abcccccccccaaaaaaaaaaaccccccjjjooootttxxxyyyyyvvvvrrrkkkfffccccc
abcaacccccaaaaaaaaaaaccccccccjjjooottttxxyyyyywwvrrrrkkkfffccccc
abaaacccccaaaaaaaaaaaaaacccccjjjjonnttxxyyyyyywwwrrlllkfffcccccc
abaaaaaaaaaaacaaaaaaaaaaccccccjjjnnnttxxyywwyyywwrrlllffffcccccc
abaaaaaaaaaaaaaaaaaaaaaaccccccjjjnntttxxwwwwwywwwrrlllfffccccccc
abaaccaaaaaaaaaaaaaaacccccccccjjjnntttxwwwsswwwwwrrlllfffccccccc
abaacccaaaaaaaacccaaacccccccccjjinnttttwwsssswwwsrrlllgffacccccc
abccccaaaaaaccccccaaaccccccccciiinnntttsssssssssssrlllggaacccccc
abccccaaaaaaaccccccccccaaccccciiinnntttsssmmssssssrlllggaacccccc
abccccaacaaaacccccccaacaaaccccciinnnnnnmmmmmmmsssslllgggaaaacccc
abccccccccaaacccccccaaaaacccccciiinnnnnmmmmmmmmmmllllgggaaaacccc
abaaaccccccccccccccccaaaaaacccciiiinnnmmmhhhmmmmmlllgggaaaaccccc
abaaaaacccccccccccaaaaaaaaaccccciiiiiiihhhhhhhhmmlgggggaaacccccc
abaaaaaccccaaccccaaaaaaacaacccccciiiiihhhhhhhhhhggggggcaaacccccc
abaaaaccccaaaccccaaaacaaaaacccccccciiihhaaaaahhhhggggccccccccccc
abaaaaaaacaaacccccaaaaaaaaaccccccccccccccaaaacccccccccccccccccaa
abaacaaaaaaaaaaaccaaaaaaaaccccccccccccccccaaaccccccccccccccccaaa
abcccccaaaaaaaaacccaaaaaaaccccccccccccccccaacccccccccccccccccaaa
abccccccaaaaaaaaaaaaaaaaacccccccccccccccccaaacccccccccccccaaaaaa
abcccccaaaaaaaaaaaaaaaaaaaaaccccccccccccccccccccccccccccccaaaaaa";
