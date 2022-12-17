use itertools::Itertools;
use std::{cmp::max, collections::HashMap};
#[derive(Debug, PartialEq, Eq, Clone)]
struct UncompressedValve {
    flow_rate: i32,
    connections: Vec<&'static str>,
}
type UncompressedCave = HashMap<&'static str, UncompressedValve>;

#[derive(Debug, PartialEq, Eq, Clone)]
struct Valve {
    flow_rate: i32,
    connections: Vec<u8>, // index is dest node
}
type Cave = Vec<Valve>;

pub fn pb1() {
    let cave = parse(INPUT);
    let (cave, start) = compress(cave);
    let path: u64 = 0;
    let o = find_max(&cave, start, 30, path);
    dbg!(o);
}

pub fn pb2() {
    let cave = parse(INPUT_TEST);
    let (cave, start) = compress(cave);
    let path: u64 = 0;
    // ignore empty paths
    let time = 26;
    let o = find_max_elephant(&cave, start, start, time, time, path);
    dbg!(o);
}

fn find_max(cave: &Cave, pos: usize, clock: u8, visited: u64) -> i32 {
    let valve = &cave[pos];
    valve.flow_rate * (clock as i32)
        + valve
            .connections
            .iter()
            .enumerate()
            .filter(|(i, d)| **d < (clock - 1) && !(is_in(visited, *i)))
            .map(|(i, d)| find_max(cave, i, clock - *d - 1, add(visited, i)))
            .max()
            .unwrap_or(0)
}

fn find_max_elephant(
    cave: &Cave,
    pos1: usize,
    pos2: usize,
    clock1: u8,
    clock2: u8,
    visited: u64,
) -> i32 {
    // open the valve?
    let valve1 = &cave[pos1];
    let valve2 = &cave[pos2];
    let pressure = valve1.flow_rate * (clock1 as i32) + valve2.flow_rate * (clock2 as i32);
    let mut maximum = 0;
    for (c1, d1) in valve1.connections.iter().enumerate() {
        if clock1.saturating_sub(*d1) < 1 || is_in(visited, c1) {
            continue;
        }
        for (c2, d2) in valve2.connections.iter().enumerate() {
            if clock2.saturating_sub(*d2) < 1 || is_in(visited, c2) {
                continue;
            } else {
                // double path eval
                maximum = max(
                    maximum,
                    find_max_elephant(
                        cave,
                        c1,
                        c2,
                        clock1 - d1 - 1, // movement and open the van when arrived
                        clock2 - d2 - 1, // movement and open the van when arrived
                        add(add(visited, c1), c2),
                    ),
                );
            }
        }
    }
    if maximum == 0 {
        // single path eval
        maximum = max(
            find_max(cave, pos1, clock1, visited),
            find_max(cave, pos2, clock2, visited),
        );
    }
    pressure + maximum
}

fn compress(cave: UncompressedCave) -> (Cave, usize) {
    let mut keys = cave.keys().collect_vec();
    keys.sort();
    let map_id = |s: &str| keys.iter().position(|i| s == **i).unwrap();
    let mut compressed = vec![
        Valve {
            flow_rate: 0,
            connections: vec![0; cave.len()]
        };
        cave.len()
    ];
    for (id, valve) in cave.iter() {
        let mut curr_dist: u8 = 1;
        // find length to all other valves
        let short_id = map_id(id);
        compressed[short_id].flow_rate = valve.flow_rate;
        compressed[short_id].connections[short_id] = 255; // remove loops
        let mut to_scan = vec![*id];
        let mut next = vec![];
        loop {
            for src in to_scan.drain(..) {
                let conns = &cave.get(src).unwrap().connections;
                for conn_id in conns {
                    let short_conn_id = map_id(conn_id);
                    if compressed[short_id].connections[short_conn_id] == 0 {
                        next.push(*conn_id);
                        // do not store dest with flow rate = 0
                        if cave[conn_id].flow_rate != 0 {
                            compressed[short_id].connections[short_conn_id] = curr_dist;
                        } else {
                            compressed[short_id].connections[short_conn_id] = 255;
                        }
                    }
                }
            }
            curr_dist += 1;
            std::mem::swap(&mut to_scan, &mut next); // reduce allocations
            if to_scan.len() == 0 || curr_dist >= 30 {
                break;
            }
        }
    }
    // compress the 0 flow rate rooms by nulling them.
    (compressed, map_id("AA"))
}
//Valve AA has flow rate=0; tunnels lead to valves DD, II, BB
//Valve AA has flow rate=0; tunnel lead to valve DD
fn parse(input: &'static str) -> UncompressedCave {
    input
        .lines()
        .map(|l| {
            let (valve, connections) = l.split_once(";").unwrap();
            let connections = connections
                .split_ascii_whitespace()
                .skip_while(|e| *e != "valve" && *e != "valves")
                .skip(1)
                .map(|e| {
                    if &e[(e.len() - 1)..] == "," {
                        &e[0..e.len() - 1]
                    } else {
                        e
                    }
                })
                .collect_vec();
            let (valve_id, flow_rate) = valve.split_once(" has flow rate=").unwrap();
            let valve_id = &valve_id[6..];
            let flow_rate = flow_rate.parse::<i32>().unwrap();
            (
                valve_id,
                UncompressedValve {
                    flow_rate,
                    connections,
                },
            )
        })
        .collect::<HashMap<_, _>>()
}

fn add(visited: u64, i: usize) -> u64 {
    visited | (1 << i)
}

fn is_in(visited: u64, i: usize) -> bool {
    visited & (1 << i) == (1 << i)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let mut t: u64 = 0;
        assert_eq!(is_in(t, 3), false);
        assert_eq!(is_in(add(t, 3), 3), true);
        t = add(t, 3);
        t = add(t, 4);
        assert_eq!(is_in(t, 3), true);
        assert_eq!(is_in(t, 4), true);
        assert_eq!(is_in(t, 5), false);
    }
}

#[allow(dead_code)]
const INPUT_CUSTOM: &str = "";

#[allow(dead_code)]
const INPUT_TEST: &str = "\
Valve AA has flow rate=0; tunnels lead to valves DD, II, BB
Valve BB has flow rate=13; tunnels lead to valves CC, AA
Valve CC has flow rate=2; tunnels lead to valves DD, BB
Valve DD has flow rate=20; tunnels lead to valves CC, AA, EE
Valve EE has flow rate=3; tunnels lead to valves FF, DD
Valve FF has flow rate=0; tunnels lead to valves EE, GG
Valve GG has flow rate=0; tunnels lead to valves FF, HH
Valve HH has flow rate=22; tunnel leads to valve GG
Valve II has flow rate=0; tunnels lead to valves AA, JJ
Valve JJ has flow rate=21; tunnel leads to valve II";

#[allow(dead_code)]
const INPUT: &str = "\
Valve AW has flow rate=0; tunnels lead to valves DS, AA
Valve NT has flow rate=4; tunnels lead to valves AO, IT, AM, VZ
Valve FI has flow rate=0; tunnels lead to valves NK, RH
Valve NK has flow rate=13; tunnels lead to valves VZ, QE, FI
Valve ZB has flow rate=0; tunnels lead to valves IC, TX
Valve DS has flow rate=3; tunnels lead to valves ME, JY, OV, RA, AW
Valve JT has flow rate=0; tunnels lead to valves RA, OE
Valve OH has flow rate=0; tunnels lead to valves KT, AK
Valve OE has flow rate=9; tunnels lead to valves SH, MR, JT, QI
Valve CT has flow rate=0; tunnels lead to valves JH, NA
Valve CB has flow rate=0; tunnels lead to valves XC, JH
Valve EK has flow rate=0; tunnels lead to valves GB, ZZ
Valve NA has flow rate=0; tunnels lead to valves GL, CT
Valve JY has flow rate=0; tunnels lead to valves DS, IH
Valve RA has flow rate=0; tunnels lead to valves JT, DS
Valve QT has flow rate=0; tunnels lead to valves ZG, KM
Valve SM has flow rate=0; tunnels lead to valves AK, AM
Valve XC has flow rate=11; tunnel leads to valve CB
Valve BF has flow rate=10; tunnels lead to valves BU, MR
Valve OV has flow rate=0; tunnels lead to valves BV, DS
Valve GB has flow rate=25; tunnel leads to valve EK
Valve SD has flow rate=0; tunnels lead to valves JF, CN
Valve IH has flow rate=0; tunnels lead to valves JY, KM
Valve DF has flow rate=0; tunnels lead to valves ON, IC
Valve BV has flow rate=6; tunnels lead to valves OV, JN, ZG, UF
Valve PO has flow rate=0; tunnels lead to valves AK, QE
Valve JH has flow rate=12; tunnels lead to valves CB, MI, CT
Valve CN has flow rate=22; tunnel leads to valve SD
Valve JF has flow rate=0; tunnels lead to valves KM, SD
Valve QI has flow rate=0; tunnels lead to valves MI, OE
Valve JN has flow rate=0; tunnels lead to valves BV, BS
Valve TX has flow rate=0; tunnels lead to valves KM, ZB
Valve ME has flow rate=0; tunnels lead to valves VG, DS
Valve ON has flow rate=0; tunnels lead to valves DF, AA
Valve GL has flow rate=20; tunnel leads to valve NA
Valve AA has flow rate=0; tunnels lead to valves ON, UF, WR, ML, AW
Valve BS has flow rate=0; tunnels lead to valves JN, IC
Valve RH has flow rate=0; tunnels lead to valves FI, KT
Valve BU has flow rate=0; tunnels lead to valves BF, BG
Valve IT has flow rate=0; tunnels lead to valves NT, KT
Valve MR has flow rate=0; tunnels lead to valves OE, BF
Valve AO has flow rate=0; tunnels lead to valves ML, NT
Valve KM has flow rate=16; tunnels lead to valves WR, IH, QT, TX, JF
Valve ML has flow rate=0; tunnels lead to valves AO, AA
Valve VG has flow rate=0; tunnels lead to valves ME, IC
Valve MI has flow rate=0; tunnels lead to valves QI, JH
Valve AM has flow rate=0; tunnels lead to valves NT, SM
Valve KT has flow rate=23; tunnels lead to valves BG, OH, RH, SH, IT
Valve AK has flow rate=14; tunnels lead to valves SM, PO, OH
Valve BG has flow rate=0; tunnels lead to valves KT, BU
Valve QE has flow rate=0; tunnels lead to valves NK, PO
Valve IC has flow rate=17; tunnels lead to valves VG, ZZ, BS, ZB, DF
Valve UF has flow rate=0; tunnels lead to valves BV, AA
Valve SH has flow rate=0; tunnels lead to valves KT, OE
Valve WR has flow rate=0; tunnels lead to valves AA, KM
Valve ZZ has flow rate=0; tunnels lead to valves IC, EK
Valve ZG has flow rate=0; tunnels lead to valves BV, QT
Valve VZ has flow rate=0; tunnels lead to valves NK, NT";
