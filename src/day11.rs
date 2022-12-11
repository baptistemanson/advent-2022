use itertools::Itertools;
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

pub fn pb1() {
    let (monkeys, mut items) = parse_cmd();
    let mut inspected: Vec<usize> = vec![0; monkeys.len()];
    let f = |m, items: &mut Vec<Vec<i64>>, i: i64| {
        let t = i.div_floor(3);
        items[test(m, t)].push(t);
    };
    for _ in 0..20 {
        for (m_n, m) in monkeys.iter().enumerate() {
            inspected[m_n] += items[m_n].len();
            items[m_n]
                .clone()
                .iter()
                .map(|i| apply(&m.op, *i))
                .for_each(|i| f(&m, &mut items, i));
            items[m_n] = vec![];
        }
    }
    inspected.sort();
    dbg!(inspected.iter().rev().take(2).fold(1, |a, acc| a * acc));
}

pub fn pb2() {
    let (monkeys, mut items) = parse_cmd();
    let mut inspected: Vec<usize> = vec![0; monkeys.len()];
    // chinese remainder
    let prod_all = monkeys.iter().map(|m| m.div).fold(1, |acc, x| acc * x);
    let f = |m, items: &mut Vec<Vec<i64>>, i: i64| {
        items[test(m, i)].push(i % prod_all);
    };
    for _ in 0..10_000 {
        for (m_n, m) in monkeys.iter().enumerate() {
            inspected[m_n] += items[m_n].len();
            items[m_n]
                .clone()
                .iter()
                .map(|i| apply(&m.op, *i))
                .for_each(|i| f(&m, &mut items, i));
            items[m_n] = vec![];
        }
    }
    inspected.sort();
    dbg!(inspected.iter().rev().take(2).fold(1, |a, acc| a * acc));
}

fn test(m: &Monkey, n: i64) -> usize {
    if n % m.div == 0 {
        m.if_true
    } else {
        m.if_false
    }
}

fn apply(op: &Op, n: i64) -> i64 {
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
