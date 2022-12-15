use std::cmp::max;

use itertools::Itertools;
type Point = (i32, i32);

pub fn pb1() {
    let points = parse_all(INPUT);
    let y = 2000000;
    let val: i32 = blocked(&points, y).iter().map(|i| i.1 - i.0).sum();
    assert_eq!(val, 4886370);
}

fn blocked(points: &Vec<(Point, i32)>, y: i32) -> Vec<Point> {
    let intervals = points
        .iter()
        .filter_map(|(s, dist)| {
            let delta = dist - (y - s.1).abs();
            if delta >= 0 {
                Some((s.0 - delta, s.0 + delta))
            } else {
                None
            }
        })
        .collect_vec();
    merge_intervals(intervals)
}

fn merge_intervals(mut arr: Vec<Point>) -> Vec<Point> {
    arr.sort_by(|p1, p2| p1.0.cmp(&p2.0));
    let mut index = 0;
    for i in 1..arr.len() {
        if arr[index].1 >= arr[i].0 {
            arr[index].1 = max(arr[index].1, arr[i].1);
        } else {
            index += 1;
            arr[index] = arr[i];
        }
    }
    arr.resize(index + 1, (0, 0));
    arr
}

pub fn pb2() {
    let points = parse_all(INPUT);
    let search_area = 4000000;
    let mut res: i64 = 0;
    for y in 0..=search_area {
        let intervals = blocked(&points, y);
        if let Some(finishing) = intervals.iter().find(|(s, e)| *s > 0 || *e < search_area) {
            res = (finishing.1 as i64 + 1) * 4_000_000 + y as i64; // it will overflow if its not i64
            break;
        }
    }
    assert_eq!(res, 11374534948438);
    dbg!(res);
}

// Sensor at x=2, y=18: closest beacon is at x=-2, y=15
fn parse_all(input: &str) -> Vec<(Point, i32)> {
    fn dist(a: Point, b: Point) -> i32 {
        (a.0 - b.0).abs() + (a.1 - b.1).abs()
    }
    input
        .lines()
        .flat_map(|l| {
            let (s, b) = l.split_once(": closest beacon is at ").unwrap();
            vec![parse_point(&s["Sensor at ".len()..]), parse_point(&b)]
        })
        .collect_vec()
        .chunks(2)
        .map(|a| {
            if a.len() != 2 {
                panic!()
            }
            (a[0], dist(a[0], a[1]))
        })
        .collect_vec()
}
//x=2, y=18
fn parse_point(s: &str) -> Point {
    let (x, y) = s.split_once(", y=").unwrap();
    let x = &x[2..];
    (x.parse::<i32>().unwrap(), y.parse::<i32>().unwrap())
}

#[allow(dead_code)]
const INPUT_CUSTOM: &str = "\
Sensor at x=2, y=18: closest beacon is at x=-2, y=15
Sensor at x=9, y=16: closest beacon is at x=10, y=16";

#[allow(dead_code)]
const INPUT_TEST: &str = "\
Sensor at x=2, y=18: closest beacon is at x=-2, y=15
Sensor at x=9, y=16: closest beacon is at x=10, y=16
Sensor at x=13, y=2: closest beacon is at x=15, y=3
Sensor at x=12, y=14: closest beacon is at x=10, y=16
Sensor at x=10, y=20: closest beacon is at x=10, y=16
Sensor at x=14, y=17: closest beacon is at x=10, y=16
Sensor at x=8, y=7: closest beacon is at x=2, y=10
Sensor at x=2, y=0: closest beacon is at x=2, y=10
Sensor at x=0, y=11: closest beacon is at x=2, y=10
Sensor at x=20, y=14: closest beacon is at x=25, y=17
Sensor at x=17, y=20: closest beacon is at x=21, y=22
Sensor at x=16, y=7: closest beacon is at x=15, y=3
Sensor at x=14, y=3: closest beacon is at x=15, y=3
Sensor at x=20, y=1: closest beacon is at x=15, y=3";

#[allow(dead_code)]
const INPUT: &str = "\
Sensor at x=220580, y=684270: closest beacon is at x=436611, y=263737
Sensor at x=3329538, y=3016377: closest beacon is at x=3355914, y=2862466
Sensor at x=2605308, y=2023938: closest beacon is at x=2197530, y=2271330
Sensor at x=1810202, y=3423309: closest beacon is at x=1829362, y=3182862
Sensor at x=480296, y=3999646: closest beacon is at x=1694700, y=4178942
Sensor at x=46556, y=1283362: closest beacon is at x=-91140, y=1441882
Sensor at x=3741660, y=3959257: closest beacon is at x=3537901, y=3368697
Sensor at x=3399994, y=700264: closest beacon is at x=3748004, y=2000000
Sensor at x=1531981, y=3801761: closest beacon is at x=1694700, y=4178942
Sensor at x=193367, y=2712458: closest beacon is at x=-91140, y=1441882
Sensor at x=3199067, y=2194575: closest beacon is at x=3748004, y=2000000
Sensor at x=1878117, y=2578817: closest beacon is at x=2197530, y=2271330
Sensor at x=2439089, y=3168242: closest beacon is at x=1829362, y=3182862
Sensor at x=273443, y=171076: closest beacon is at x=436611, y=263737
Sensor at x=3680413, y=2477027: closest beacon is at x=3748004, y=2000000
Sensor at x=3620241, y=2904998: closest beacon is at x=3355914, y=2862466
Sensor at x=1728351, y=2895399: closest beacon is at x=1829362, y=3182862
Sensor at x=1894207, y=1168355: closest beacon is at x=2197530, y=2271330
Sensor at x=856867, y=3271314: closest beacon is at x=1829362, y=3182862
Sensor at x=3056788, y=2626224: closest beacon is at x=3355914, y=2862466
Sensor at x=3598024, y=3322247: closest beacon is at x=3537901, y=3368697
Sensor at x=1662543, y=3128823: closest beacon is at x=1829362, y=3182862
Sensor at x=3992558, y=1933059: closest beacon is at x=3748004, y=2000000
Sensor at x=1844282, y=2994285: closest beacon is at x=1829362, y=3182862
Sensor at x=3604375, y=3668021: closest beacon is at x=3537901, y=3368697
Sensor at x=2569893, y=3911832: closest beacon is at x=1694700, y=4178942
Sensor at x=117970, y=37503: closest beacon is at x=436611, y=263737
Sensor at x=3951385, y=3125577: closest beacon is at x=3537901, y=3368697
Sensor at x=2482373, y=2648092: closest beacon is at x=2197530, y=2271330
Sensor at x=915040, y=1835970: closest beacon is at x=-91140, y=1441882
Sensor at x=3047883, y=3301452: closest beacon is at x=3537901, y=3368697
Sensor at x=117432, y=1503889: closest beacon is at x=-91140, y=1441882
Sensor at x=1136011, y=261705: closest beacon is at x=436611, y=263737
Sensor at x=2343111, y=66183: closest beacon is at x=2081841, y=-807749
Sensor at x=608229, y=955721: closest beacon is at x=436611, y=263737
Sensor at x=1189379, y=3999750: closest beacon is at x=1694700, y=4178942
Sensor at x=766640, y=26597: closest beacon is at x=436611, y=263737
Sensor at x=3891093, y=2110588: closest beacon is at x=3748004, y=2000000";
