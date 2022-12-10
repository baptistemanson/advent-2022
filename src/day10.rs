pub fn pb1() {
    let mut out = 0;
    let mut tick = |c: &mut i32, r| {
        *c += 1;
        if (*c - 20) % 40 == 0 {
            out += *c * r;
        }
    };
    process(&mut tick, 0, 1);
    dbg!(out);
}
pub fn pb2() {
    let mut tick = |c: &mut i32, r: i32| {
        let y = (*c % 40) as i32;
        if y == 0 {
            print!("\n");
        }
        print!("{}", if (r - y).abs() < 2 { "█" } else { " " });
        *c += 1;
    };
    process(&mut tick, 0, 1);
}

fn process<F: FnMut(&mut i32, i32)>(tick: &mut F, mut clock: i32, mut reg: i32) {
    INPUT.lines().for_each(|l| {
        tick(&mut clock, reg);
        match l {
            "noop" => {}
            _ => {
                tick(&mut clock, reg);
                reg += l.split_once(" ").unwrap().1.parse::<i32>().unwrap();
            }
        };
    });
}

#[allow(dead_code)]
const INPUT: &str = "addx 2
addx 3
noop
addx 1
addx 27
addx -23
addx 5
noop
addx 1
noop
addx 4
addx 1
noop
addx 4
addx 5
noop
noop
noop
addx 5
addx -4
addx 4
noop
addx 1
addx -38
noop
noop
addx 7
addx 8
addx -3
noop
addx 3
noop
addx 5
noop
noop
addx -2
addx 2
addx 9
addx -2
addx 6
addx 1
addx -4
addx 5
addx 2
addx -14
addx -6
addx -16
addx 1
addx 5
addx 1
addx 4
addx -2
noop
addx -7
addx -3
addx 17
addx 5
noop
noop
addx 19
addx -16
noop
addx 14
addx -8
addx 2
noop
addx 4
noop
addx -35
addx -2
noop
noop
addx 7
addx 19
addx -26
addx 10
addx 29
addx -21
noop
addx 4
noop
noop
addx -9
addx 4
addx 8
addx 7
noop
addx -2
addx 5
addx 2
addx -19
addx -18
noop
noop
noop
noop
addx 7
addx -7
addx 37
addx -27
addx 5
addx 2
addx -12
addx 4
addx 11
noop
noop
noop
addx 5
addx -14
addx 21
addx -4
addx 5
addx 2
noop
addx -35
noop
noop
noop
noop
addx 7
addx 1
noop
noop
addx 5
addx -1
addx 5
addx 1
noop
addx 4
addx 1
noop
noop
addx 4
noop
addx 1
addx 2
addx 5
addx 2
addx 1
noop
noop
noop
noop";

#[allow(dead_code)]
const INPUT_TEST: &str = "addx 15
addx -11
addx 6
addx -3
addx 5
addx -1
addx -8
addx 13
addx 4
noop
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx -35
addx 1
addx 24
addx -19
addx 1
addx 16
addx -11
noop
noop
addx 21
addx -15
noop
noop
addx -3
addx 9
addx 1
addx -3
addx 8
addx 1
addx 5
noop
noop
noop
noop
noop
addx -36
noop
addx 1
addx 7
noop
noop
noop
addx 2
addx 6
noop
noop
noop
noop
noop
addx 1
noop
noop
addx 7
addx 1
noop
addx -13
addx 13
addx 7
noop
addx 1
addx -33
noop
noop
noop
addx 2
noop
noop
noop
addx 8
noop
addx -1
addx 2
addx 1
noop
addx 17
addx -9
addx 1
addx 1
addx -3
addx 11
noop
noop
addx 1
noop
addx 1
noop
noop
addx -13
addx -19
addx 1
addx 3
addx 26
addx -30
addx 12
addx -1
addx 3
addx 1
noop
noop
noop
addx -9
addx 18
addx 1
addx 2
noop
noop
addx 9
noop
noop
noop
addx -1
addx 2
addx -37
addx 1
addx 3
noop
addx 15
addx -21
addx 22
addx -6
addx 1
noop
addx 2
addx 1
noop
addx -10
noop
noop
addx 20
addx 1
addx 2
addx 2
addx -6
addx -11
noop
noop
noop";
