pub fn pb1() {
    let input = INPUT.as_bytes();
    let nb_target = 1000000000001;

    let mut cave = [0 as u8; 50_000];
    let mut highest: usize = 0;
    let mut virtual_highest: usize = 0;

    let mut falling = [0 as u8; 4];
    let mut vertical_pos: usize = 3;

    let mut nb_rocks = 0;
    let mut tick = 0;
    let spawn =
        |vertical_pos: &mut usize, nb_rocks: &mut usize, falling: &mut [u8; 4], highest: usize| {
            falling.copy_from_slice(&ROCK_PATTERNS[*nb_rocks % 5]);
            *nb_rocks += 1;
            *vertical_pos = highest + 4;
        };

    let move_falling =
        |a: &mut u8, to_the_left: bool| if to_the_left { *a <<= 1 } else { *a >>= 1 };

    spawn(&mut vertical_pos, &mut nb_rocks, &mut falling, highest);
    vertical_pos = 3;

    let mut first_alignment = None;
    let mut first_alignment_rock = 0;

    while nb_rocks < nb_target {
        // jet patterns
        let (wall, to_the_left) = match input[tick % input.len()] as char {
            '<' => (1 << 7, true),
            '>' => (1 << 1, false),
            _ => panic!("unknown char"),
        };
        if does_not_hit_wall(&falling, wall) {
            falling
                .iter_mut()
                .for_each(|a| move_falling(a, to_the_left));
        }
        if does_overlap(&falling, &cave, vertical_pos) {
            // undo if its now overlapping other walls
            falling
                .iter_mut()
                .for_each(|a| move_falling(a, !to_the_left));
        };

        // gravity
        if vertical_pos == 0 || does_overlap(&falling, &cave, vertical_pos - 1) {
            rest_falling(&falling, &mut cave, vertical_pos);
            highest = find_new_highest(highest, &cave);
            spawn(&mut vertical_pos, &mut nb_rocks, &mut falling, highest);
            // cycle detection, we skip the first 10k
            if tick % input.len() == 0 && nb_rocks > 10_000 && virtual_highest == 0 {
                if first_alignment == None {
                    first_alignment = Some(highest);
                    first_alignment_rock = nb_rocks;
                } else {
                    let first_alignment = first_alignment.unwrap();
                    let s = &cave[first_alignment..highest - 15]; // 15 to take some margin for the other pieces to come.
                    let f = &cave[(2 * first_alignment - highest)..first_alignment - 15];
                    if *f == *s {
                        let cycle_rock = nb_rocks - first_alignment_rock;
                        let cycle_height = highest - first_alignment;
                        let rock_left = nb_target - nb_rocks;
                        let nb_cycles = rock_left / cycle_rock;
                        virtual_highest = nb_cycles * cycle_height;
                        nb_rocks = nb_rocks + cycle_rock * nb_cycles;
                    }
                }
            };
            if nb_rocks == 2023 {
                // assert_eq!(highest + 1, 3193);
                println!(" part 1 {}", highest + 1);
            }
        } else {
            vertical_pos -= 1;
        }
        tick += 1;
    }
    let total = virtual_highest + highest + 1;
    println!("part 2 {}", total);
}

fn does_not_hit_wall(falling: &[u8; 4], pos: u8) -> bool {
    (falling[0] | falling[1] | falling[2] | falling[3]) & pos == 0
}

fn does_overlap(falling: &[u8], cave: &[u8], vertical_pos: usize) -> bool {
    (falling[0] & cave[vertical_pos]
        | falling[1] & cave[vertical_pos + 1]
        | falling[2] & cave[vertical_pos + 2]
        | falling[3] & cave[vertical_pos + 3])
        != 0
}

fn rest_falling(falling: &[u8; 4], cave: &mut [u8], vertical_pos: usize) {
    cave[vertical_pos] |= falling[0];
    cave[vertical_pos + 1] |= falling[1];
    cave[vertical_pos + 2] |= falling[2];
    cave[vertical_pos + 3] |= falling[3];
}

fn find_new_highest(highest: usize, cave: &[u8]) -> usize {
    let mut i = highest;
    while cave[i] != 0 {
        i += 1;
    }
    i - 1
}

#[allow(dead_code)]
fn display(stuff: &[u8], start: usize, end: usize) {
    (start..=end)
        .rev()
        .for_each(|i| println!("{:08b}", stuff[i]));
}
#[allow(dead_code)]
fn display_falling(falling: &[u8]) {
    display(falling, 0, 3);
}

pub fn pb2() {}

#[cfg(test)]
mod tests {
    //use super::*;

    #[test]
    fn test_1() {}
}
// appear 2 away from the left edge
const HL: [u8; 4] = [0b00111100, 0b00000000, 0b00000000, 0b00000000];
const X: [u8; 4] = [0b00010000, 0b00111000, 0b00010000, 0b00000000];
const IL: [u8; 4] = [0b00111000, 0b00001000, 0b00001000, 0b00000000];
const I: [u8; 4] = [0b00100000, 0b00100000, 0b00100000, 0b00100000];
const S: [u8; 4] = [0b00110000, 0b00110000, 0b00000000, 0b00000000];
const ROCK_PATTERNS: [[u8; 4]; 5] = [HL, X, IL, I, S];
#[allow(dead_code)]
const INPUT_CUSTOM: &str = "";

#[allow(dead_code)]
const INPUT_TEST: &str = ">>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>";

#[allow(dead_code)]
const INPUT: &str = "><<<>>><<<><<<>>><<>>>><<<>>>><>>>><<<>>><<<<>>><<>>>><<<>>><<<<>>><<<<>>><<<<>>>><<<<>>>><<<<>><<<<><<<<><<<>>>><<<>><<>>><<<>><<>>><<>>><>><<<>><<<<>>><<>><>>><<<<>>><>><>>><<<<>>>><<<>>><<<><<<<>>><><>>>><<<><<<<>><<<>><<<>>>><><<<><<>>>><<>>>><<<><<>>>><<>><<><<<>><>>>><<<>>>><<<>><>><<<>>>><<<<><<><<<>><<>>>><<<<>><<<<><<<<><<<<><>><<><<>>><>><<<<>>><>>><>>>><<<<>>><><<>><<<<><<<>>><>>><<<><<<<><<<<><<<<>><>>><><><<<<>><>>>><<<<><>><<<<>>><<<>>><<>>>><<<<><>>>><<>><<<<>>>><<>>><<<>>><><<>>>><<<<>><<>>><<<>><><<<<>>>><<<<>>><>>>><<>>>><<<>>><>>>><<<>>>><<><<>>><>>><<<>>><<<>><>>><<<<>><<<><<>>><>>>><<<<>>><<<>>><<<<>>><<<<>><<>><><<<<>><<<>>><<<<><<<<>>>><<<>><<<><>>><<>>>><<<>>>><<>>><<<<>>><<<>>>><<<>><<<<>>><<<<>>><<>>>><<<>>>><>>><<<>>>><<>><>>>><<>>>><<<>>>><<<<><<<><<>><<<>><<<>>>><<>>>><<<<>><>>>><<<<><<<<>><<>><<>><<<<>>><<>><<><>>>><<<>><>><<>><<>><>><<<>><<<<><<<>>>><<<>>>><<<<>><>><<<<><<<>><<<<>>><>>><<<<><<<>><<<<>><<>>>><<<>>>><<<<>><<<>><><<<<>>><>><<><<<<>><<<><<<><<<<>>>><<<<>>>><><>><<<>>>><<<<>><>>><<<<><<>>><<<>>>><<>><<<>>><>>><<><<<>>><<<><<<>>><<<<><<<<>><<><<>><><<<<>><<<<>>>><><<<>>>><<<>><>>>><<>>>><<<<>>>><<<>>><<<<><<>>>><<<><<<>>>><<><<<>>>><><<<><>>>><>>>><<<>>><>><>>>><>><<>>><>>>><<<>>><<>>>><<>>>><<><<<<>>>><<>>><>><<<<>>><<<>>>><<<>>>><<>>><>><>><<<<>>>><<><<><>>>><<>>>><>>><<<<>>>><<<<>>><<><<<><<>>>><<>>><>>><>>><<<><<<>>>><<<>><<>>><><<<<>>><<<<>><<<>>>><<>><<<<><<<><<<<>><<<<>>>><<><<<><<<<>>>><<<<><<<<>>><<<>>>><<<>>><><<<<>>>><<<>><<<<><>><<<><<>>><>>>><<<><<<<>><<<<><<<<>><<>>>><>><<<>>>><<<<><<<>><<<>>>><><<<>><<<><<<<>>>><><<>><<>>><<<<><<<<><<<<><<>>>><<>>>><<><<<<>><<<<>><<<><>><<<<><<<>>>><<<>><>>>><<<<><<<<><<>><<>><<<<>><<<<>><><><<>><<<<>>>><<<>>>><>>><<<>>>><<>>><><><<><>>><<<<>>>><<>>><<<>>>><<<>>>><>><<<<>>>><<<<>>><<>>>><>>><<<<>>>><<<>>>><<<<>><><>><<<>>>><<>><<<>>><>>>><<<>>>><>><<<>>><<<<>><<<>><<<<>>>><>>>><><><<>><<<<>><<<>><<><<<<>><<>>><>><<><<>><<<<>>><>>>><<<>>>><<>>><>>>><<<><<<<>>>><<<><<<><<<>><<>>>><<<><<<<><<<<>>>><<<<>>>><<<><<<<>>><<>>>><<<>><<<>><<>>>><<>><<<>>>><><<<<>><><<<<>><<<>>><<<>>>><<<>>>><<>>><<<>><<<><>><><<<><>>>><<><<<<>>>><>>><<>>><>>>><<<<>>>><<>><<<<>><<<>><><<<<>><<>>>><<>>>><<<>><<<<>>><<><<<<>>><<>>>><<>><<<<>><<><<<><>><>>><>><<<>>><<<<>>><<>>>><<<><<<<><<<<>><<<<>>>><<<<>>><<>><<<><>>>><<<><<>>>><<<>><>>>><<<<>><<<<>><<>>>><<><<<><>>><<<>>><<>>><<>><<<<>>>><<<<>><<<<>><<<>>>><<<<>>><<>>><<<<><<<>>><<<><<>><<<<>><>>><<><<>>><<<<>>>><<<>>><<<>><<<<>>>><<>><<<<>><><<>>>><>><<<>><<<>><<<<>><>><>><<<>>>><<>><<><<>>><<>>>><<<<>>><<>>>><><<<>>><<>><<<<><<<<>><<>>>><<<<>>>><>>><>>><<<>><><<<>>><<<<>>><>>>><<<>><<<<><<<>>><<<<>>>><<<>>>><<<<>>><<<>><<<<>>><<<>>>><<<<>>>><>>><<<<><><<><<><<>>>><>>>><<<>>><<><><><<<<><<<<>>><<<>>>><<<<>>>><>>><<>>><<>><<<>>>><<><<>><<>>><<<><<<><<<>>>><>><>>><>>>><<<<>>><>><<<><>>>><<<>>>><<<><<<<>>><<<<><<>>><>>>><><<><<<<>>><>>><>><<>>><<<>>>><<<>>>><<<<><<<>>><<<<><<<>>>><<<>>>><>>><<>>>><<>>><<<<><>>><<<><<>><>>><>><<>><<>><<<>>>><<<<>><<<>>><<<<>>>><<>><>>><>>><<<>><<<><<>><>><><<<>>>><<<><<>><<<>>><<<<>><<<>>>><<<<>><>>>><>><<<><>>>><<<<><<<><>>>><<<>>>><><>>><><>>><<<>>>><<>>><<>><<>><>><>>>><<<>>>><><>>><<<<>>><<<>><<><>><<<<>>><<>>>><<<>>><<<>>><<<><<<>><<<<>>><><>>>><<>><<<>>><<<<>><<<>><><<<>>><<<><>>>><><<>><<<><<<>>><<<>>><<>>>><<<>>><<<<>>>><<>>>><<>>><<<<>><>>>><<>>>><><<>><><<<<>><<>>><<>>><<<><<>>><<<>><<<>>>><<>>><>>>><<<><<<><<>>>><><<<<>>><<>>><<<<>>>><<<>>><<<<>>>><<<<>><<>><<>>><<<>>><<<><<<<>>>><<>><<<><<<<>><<<<>>>><<><<<<><>><><<<>><<><<<>>>><<><<<><>>>><<<>>>><<<<><<>>><<<>>><<>>><<><<<<><>>><<>><<<<><<<>><<><><<<><<<><><>>><<><>>>><<><<<><<<<>>>><><<>>>><<><<><<<>>><<<<>><<<><<<>>><<<>>>><<<<>>><<>>>><<<><<<<>><<<<><<<>>>><<<>>>><<<>>>><<<>><<>>>><>><<<><<>>><<<<>>>><><<<<>>><<>><<>><<<<>><<<<><<<<>><<<<>><<<<>>><<>>>><<<<>>>><<<<>>><<<><><<<<>>><<<>>><<>><<><<<>>>><<>><<<><>><<<<>>><<<<><<<<><<><>><<>>><<<><<><<<<>>><<<>>><<<<>><<<<>>>><>>>><><<>><<>><>><<<><<>>>><<<>><<<><<>><<><<<<>>><<>>><<<<>>>><>><<>>>><<<>>><>><<<<>>><<<<>>>><<<>>><<>>><<<<><<<><>>><>><<<>>>><><<<>><<><>><<<><<<>><<>>>><<>>>><<<>><<<>>>><>>><><>>><<<>>>><<<>>><<<><<<<>><<<<><><<<><<><<<<>><><<<<>>><<<>><<<<>>>><<<>>>><<<>>><<<>>><<>>>><<>>>><<<>>>><<<>>><<<>><>>><>>><<<>>>><<>>>><<><>>>><<<><<<>>>><<><<<>><<>><<<<><<<<>>>><<<<><>>><<<<>>>><<<>>>><><<><<<>>>><<<><<>><<>>>><>>><>>>><<>>><<>><<>>><>>><<<>>>><<<>>><<<<>>><<<>>>><<<<>><><<>>>><<<<><>>><<><<<>>>><>>>><<>>>><<<><<><<<<>>>><>>><<<>>>><<<>><><<>>><<<><<>><><<<>>><<<<><<><<<>>><<>>><>>>><>>><<>>><>><><<<<>>>><<<<>>>><><<<<><<<<><<<<>>>><<<<>>><<<<><<<>><<>>>><<><<<>>><<><<<>><>>><<<<>>><<><<>>><<<><<><<<>><<<<><<>>><><<>><><><<<><<>>><<>>>><<<<>>><<<>><<<<><<>>>><>>>><<<<><<<<>>>><<>>><<<<>>><<>>><<<>>><>>><<<>>>><<<>><<><<<<><>>>><<>><<>>>><<<>>><<<>>><>>><<<<>><<<<>><<><<<<>>><<>>>><>>>><>>><<<<>>>><><<<>>>><<<<>>>><<<>>>><<<>>>><<<<><<<>>>><<<>>><<<<>>>><<<><>>><><>>>><<<<>><<>>><<<><<<<><<>><<<<>><><<><<<>>><<<<>><<<>>>><<<>>>><<><>>><<>>><<<>>><<<<><<<<>>><<<<>><>>>><<<<>>>><<<><<<<>>><<>>>><<<>>>><<><>>>><<>>>><<>><<<>><<>>><<<<>>>><<><>>><<><<<>>>><<<<>><>>><><>>>><<<>><<<<>>>><<<<><<>>>><<<<>><<<>>><><<<<>>><<>>>><<>>>><<<><<><<<<>>><<<<>>>><<<><<<<><><<<<>>>><<<<>>><<<<><<<><<<>><><<<<><<>>><<<>><<<>>>><<>>>><<>>>><<>>>><<><<<<>><<<<><<<<>>><<><>>>><>>><><>><<><<>>>><<<<><>><<<<><<>><<>>><>><<<>><>><>><<<>>><<>>>><<<>>>><<>>><<>>>><<>>><<<<>>><<<<>>><<<<><<>>>><<<<>>>><<<>>><>>><<>>>><<<>>>><>>>><>><<<<>>>><<<<><<>>>><<<<><>>>><<<>><<<<>>>><>>><<<<>>>><<>>><<<<>><<<<><<<>><<<>><<>><>>>><>><<<<>>>><<<>><<>><<>>><<<>>>><>>>><<>>><>>><<><<>><<<<>><<<>>><<<><<<>>><<<<>>><<<><<<>>>><<<<><<>>><<<>>>><>>>><<<<><><<<<>>>><<>>><>>><<<<>>>><<<<>><<>><<<>>>><<<>><>>>><>>>><><<<>>>><<<<><<<>>><<>>><><<<<>><<<<>><>>><>><<<>>>><<<>>>><<<<>><<>>><<<><<<>><<<>><<<>><<>>><<<<>><<<>>>><<>>>><<<>><>>><>>>><<<<>>>><<<>><>>><<>><><<<>>>><>><<<<>>>><<<<><>>>><<<<><<><<<<>>>><<>><<<>>>><<<><<>>><<<<><<<<>><<>>><<<<>>><<><<<<>>><<><<>>><<>><<<<><>>><<<<>>><<>><<<<>>><<<><>>>><><<><><<<>><>>><>><>>><<<>>>><>><<<<>><<<>><<<<>>><<<>>><<<>>>><><<><<<>><<<<>><<><<<<>><<<<>>>><<<<>><<<>>><<<<>>>><<>>>><<<><<<>>>><<>>><<>>>><<<<>>><<>>>><>>>><<<<>>>><<<>>><<<>>><<<><>>>><>>>><<<<>>><<<>>>><>>>><<>>><<>>><><<<<>><<>><<>><<<<>>><<<<>>>><<<>>>><<<>><>><<<<><>><>>>><<<<>>>><<<>>>><<<>>>><<>>>><>>>><<<<>>><>>>><<<<>><<><<<>><<<<>><<<>><<<<><<<>>>><<>><>><>><<<>>>><>><<<><<><<<<>>><<<<><>>>><<><>><<<>><>><<<>>>><<<><<<<>>>><<<<><>>>><<<<>><<<<>><<<<><>>>><>>><<<<>>>><<<><<<>>><<<>><<>><<<<>>>><<<>><<<<>>><<>>>><<>><<>>>><<><<><<<<>>>><<<>>><<<><>>>><<>>>><><<<>><>><<><<>>>><><>>>><<<>>>><<<<>><<>><<<>>>><>>><<>>><>>><><<><<<<>><>><<<>>><<<>><><<<<>><<<>>><><<<<><>>><<<><><<<<>>><<>>><>>><>>>><<<>><<<<>>>><>>><<>>>><<>>><<<>>>><>>><<<>>>><<<<>>><<<<>>><<><<><<>>><<<<>>>><<<><>><>>>><<>>>><<<>>>><>>><<<><<><>>>><<>>><<<>>>><<>>>><<<><<<<>>>><>><<<>>><<<><<<>>><><<<<>>>><>><>>>><<<<><<<<>>>><<>>>><><<<>><>>><<<<>><<<<>>><<<<>>>><<<>><<<<>>>><<><>><<<>>><><<<<>>>><<<<><<<>><>><<>>><<<><<<<>><<>>>><<<>>>><<>>>><<<<>><<>><>>><<<<>>><<<>>>><<<<>>>><<>><<><<>><<<<>>><<<<>>><<<<>>>><><<<>>><<<<>>><<<<><><<<<><>><<<<>>><<<>>><<<<>>>><<<<>>><>>><<>>><<<<>><<<>><>><<>>>><>>>><>><<<>>>><<<<>>>><<<>>>><<<><<<><<<>><>>><<<<>>>><<<>>>><<<<>>><<<<>><<>><<<<><<<><>><>>><><<<><<<>>>><<<>>><<<<>>>><<<<><<>>>><<<<><<<>>><<>><<<<>><>>><<>>>><<<><<>>><>>><<>>><<<>>>><>><><<<<>>><><<<<>>><<<>>>><<><><<><<>>><>><<<<>>><<<>>>><<<><<<>><<<<>>><><>><<>>>><<<><<<<><>>><>><<<<>>>><<>>><<<<>>>><<<<>>><<>>><<<<><<>>>><<<>>>><<>>><>>><<>>><<<><<<>><<<<>>>><<<<>>><<>><>>>><<<<>>>><<<><>>>><<<<><<<<>><<>>><<<<>>>><><<>><<<<><>>>><<<<>><>>><><>>><<>>>><<<><><<<<>><<<><<<<>>>><><>>><<>>><<<>>><<<<>>><<<<>><<>>>><<<>>><>>><<<<>>>><<><>>><<<><<<>>>><<<>>>><<<><>>>><<<<>>><>>>><<<>>><<<><<<<>>><<<<><<<<>>><<>>>><<<>>><<<<><<><<<<>>>><<<>><<<<>><>>><<<<><<>>>><<>><>>>><<<<><<<>><<>>>><<>>>><<<<>><<<<>>>><<>>>><<<>>>><<>>><><<<>>><<><<>>>><<<>>>><<>>>><>><<<<>>>><<<<>>>><<><<><<<<><<>>><<>>><<<<>>>><<<>><<<<><<<<>>><<<<>>><<<>>><<<<><<>>>><<<<>>>><<<>><<<>><>>>><<<><><<<<><><<<>>><<<<>>>><<<><<<<>>>><<<>>>><>><<<<>>>><<>>>><<<>>>><<<<><<<>>>><>>>><><<<><<<>><>>>><<<>><<<<><<<<>><<>>><>>>><<>><>><>><<<>>><<<>>><<<>>><<>>><<>><<<>>>><<<<>>><<<>><>>>><<<<><<><>>>><<>>><<><<<<>><<<<>>>><<>><<<><<>>><<<<>><<<>><<><<<<>>>><<<>>>><<<<>>><>><<<<>><<<><>><<>>><<<<><<<<>>>><<<>><<<>>>><<<<>>>><<<>>><<>><>>><>>><<<>>><><<<<>>><>>><>>>><>>><<<<>>>><<<><<><><<<>>>><<<><<<>><<<<>>><<<><<><<<<>>>><<><<<<>>><>>>><>>>><>><<<<>>>><<<<>>><>>><<<>><>>>><<<<>><<<<>>>><>>><>>><<<<>><<<>>>><<<><<<<><>>><<<<>>><<<>>>><><<<><>><<<<>><<<><<<<>>>><>><><<<<><<<<>>>><<<>>>><<>><<<<><<>>><<<<><<<><<<<>><<<><<>>>><<<<>><>>>><<<>>><<>><<<>><><<<<>>>><<<>><<>><>>>><<<>>><<>>><<<>>><<>>><<<><<<<>>><>><<<>>>><<<<>>>><<<>><<<<>>>><>><<<><><<<<>><<<>>>><>>>><<<<>>>><<>>><<>>><><>>><<<><>><<>>>><>>><<<>>>><><>><>>>><<>>><>><<<<>><<>>><<>>>><><<><<<>><<>>>><<<>>><<><<<>>>><<<><><<>><<<>><<<><<<>>>><<<>>><>><<<>>><>>><>>><<>>><<<<><<<<>><<<>>><<<><>>><<<<>><>><<<>>><<>><>><<><>>><<<<>><>>><<<<><<<><<<><<<>>><<>>>><<<<>><<>><<>>><<>><<>><<<<>>><<><<<><>>><>><<><<>>>><<>>><<<<>>><<<>>>><>>><>>><<>>><><<<><<><<>>><>>>><>>>><<<<>>><>><<<>>>><>><<<>><<>>><>><<><<<><<<>>><<>>><<<<>>><><<<<>>><<<>><>>>><<<>>>><<<<><<<<>>>><<>><><<>>><<<><<>>><<>>>><<>><<<><<<><<<>>><<>><<<>>>><><<<<>>>><<>>><<>><<>>><<>>><<>>>><<<><<<<>><<>>>><<<>>><><<><<><<>>><<<<><<<><<><<>>>><<>>><><<<<>><<>><><<><>>><>>><<>>><<<<>>><<<<><>><<>>>><<<<>>>><>><<<<>><<><<<>>>><>><<<<>>>><<<><<<>><><<<<>>>><><<<<>><<<><<>>>><<><<<>><<<>><<<>><<<>>><<><<<>>>><>><<>>>><<>>><>>>><>>><><>><<<>>><<<><><<>><<<>>><<<<>>><<<>>>><><>>><<><>><<<>><<>><<>>><<<<>>><<<><<<<><>><<<>>><<<>>><<>>>><<<<>>><<>>><<<><<>>><<<>><<<>>><>>>><<<<>>>><<<<>><<<<>>><<>><<<<>><<<>>><<<>><><<<>>>><<>>>><<<<>>>><<>><>>>><<<<>>><>>>><>><<<<>>><>>><<>>>><<<<><<<><<<>><<>>><<><>>><<<<>>><>>>><<>>><<<<>>><>><<<>><<><<<><<<<>>><><<<<>>><<<<>>><<><<<<>><<>>>><>>>><<><><<<>>><>><>><>>>><><<<<>>><<<<>>>><<>><<>";