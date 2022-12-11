use itertools::Itertools;

pub fn pb1() {
    let (monkeys, items) = parse_cmd();
    let next_panic = |m: &Monkey, i: i64| -> i64 { apply_op(&m.op, i).div_floor(3) };
    juggle(monkeys, items, next_panic, 20);
}

pub fn pb2() {
    let (monkeys, items) = parse_cmd();
    // chinese remainder
    let prod_all: i64 = monkeys.iter().map(|m| m.div).product();
    let next_panic = |m: &Monkey, i: i64| -> i64 { apply_op(&m.op, i) % prod_all };
    juggle(monkeys, items, next_panic, 10_000);
}

fn juggle<F: Fn(&Monkey, i64) -> i64>(
    monkeys: Vec<Monkey>,
    mut items: Vec<Vec<i64>>,
    next_panic: F,
    iteration: usize,
) {
    let mut inspected: Vec<usize> = vec![0; monkeys.len()];
    for _ in 0..iteration {
        for (idx, monkey) in monkeys.iter().enumerate() {
            inspected[idx] += items[idx].len();
            items[idx]
                .clone()
                .iter()
                .map(|i| next_panic(&monkey, *i))
                .for_each(|i| items[which_dest(monkey, i)].push(i));
            items[idx] = vec![];
        }
    }
    inspected.sort();
    dbg!(inspected.iter().rev().take(2).fold(1, |a, acc| a * acc));
}

#[derive(Debug)]
enum Op {
    Add(i64),
    Mul(i64),
    Square,
}
#[derive(Debug)]
struct Monkey {
    op: Op,
    div: i64,
    if_true: usize,
    if_false: usize,
}

fn which_dest(m: &Monkey, n: i64) -> usize {
    if n % m.div == 0 {
        m.if_true
    } else {
        m.if_false
    }
}

fn apply_op(op: &Op, n: i64) -> i64 {
    match op {
        Op::Add(x) => n + x,
        Op::Mul(x) => n * x,
        Op::Square => n * n,
    }
}

fn parse_cmd() -> (Vec<Monkey>, Vec<Vec<i64>>) {
    let mut l = INPUT.lines();
    let mut monkeys: Vec<Monkey> = vec![];
    let mut hands: Vec<Vec<i64>> = vec![];
    while let Some(_monkey_name) = l.next() {
        let (_, items) = l.next().unwrap().split_once(": ").unwrap();
        let items = items
            .split(", ")
            .map(|i| i.parse::<i64>().unwrap())
            .collect_vec();
        hands.push(items);
        let op = l.next().unwrap().split_once("old ").unwrap().1;
        let op = match op.split_once(" ").unwrap() {
            ("*", "old") => Op::Square,
            ("*", a) => Op::Mul(a.parse::<i64>().unwrap()),
            ("+", a) => Op::Add(a.parse::<i64>().unwrap()),
            _ => panic!("dont regonize op"),
        };
        let div = parse_last_i64(&mut l);
        let if_true = parse_last_i64(&mut l) as usize;
        let if_false = parse_last_i64(&mut l) as usize;
        monkeys.push(Monkey {
            div,
            op,
            if_false,
            if_true,
        });
        l.next(); // blank
    }
    (monkeys, hands)
}
fn parse_last_i64(l: &mut std::str::Lines) -> i64 {
    l.next()
        .unwrap()
        .split_whitespace()
        .last()
        .unwrap()
        .parse::<i64>()
        .unwrap()
}

#[allow(dead_code)]
const INPUT_CUSTOM: &str = "";

#[allow(dead_code)]
const INPUT_TEST: &str = "\
Monkey 0:
Starting items: 79, 98
Operation: new = old * 19
Test: divisible by 23
  If true: throw to monkey 2
  If false: throw to monkey 3

Monkey 1:
Starting items: 54, 65, 75, 74
Operation: new = old + 6
Test: divisible by 19
  If true: throw to monkey 2
  If false: throw to monkey 0

Monkey 2:
Starting items: 79, 60, 97
Operation: new = old * old
Test: divisible by 13
  If true: throw to monkey 1
  If false: throw to monkey 3

Monkey 3:
Starting items: 74
Operation: new = old + 3
Test: divisible by 17
  If true: throw to monkey 0
  If false: throw to monkey 1";

#[allow(dead_code)]
const INPUT: &str = "\
Monkey 0:
  Starting items: 59, 65, 86, 56, 74, 57, 56
  Operation: new = old * 17
  Test: divisible by 3
    If true: throw to monkey 3
    If false: throw to monkey 6

Monkey 1:
  Starting items: 63, 83, 50, 63, 56
  Operation: new = old + 2
  Test: divisible by 13
    If true: throw to monkey 3
    If false: throw to monkey 0

Monkey 2:
  Starting items: 93, 79, 74, 55
  Operation: new = old + 1
  Test: divisible by 2
    If true: throw to monkey 0
    If false: throw to monkey 1

Monkey 3:
  Starting items: 86, 61, 67, 88, 94, 69, 56, 91
  Operation: new = old + 7
  Test: divisible by 11
    If true: throw to monkey 6
    If false: throw to monkey 7

Monkey 4:
  Starting items: 76, 50, 51
  Operation: new = old * old
  Test: divisible by 19
    If true: throw to monkey 2
    If false: throw to monkey 5

Monkey 5:
  Starting items: 77, 76
  Operation: new = old + 8
  Test: divisible by 17
    If true: throw to monkey 2
    If false: throw to monkey 1

Monkey 6:
  Starting items: 74
  Operation: new = old * 2
  Test: divisible by 5
    If true: throw to monkey 4
    If false: throw to monkey 7

Monkey 7:
  Starting items: 86, 85, 52, 86, 91, 95
  Operation: new = old + 6
  Test: divisible by 7
    If true: throw to monkey 4
    If false: throw to monkey 5";
