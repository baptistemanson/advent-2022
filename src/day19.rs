use std::collections::HashSet;

use itertools::Itertools;

pub fn pb1() {
    let blueprints = parse(&INPUT);
    let s: i32 = blueprints
        .iter()
        .map(|b| {
            b.idx
                * get_best(
                    &b,
                    State {
                        time_left: 24,
                        ore: 0,
                        clay: 0,
                        obs: 0,
                        geode: 0,
                        ore_robot: 1,
                        clay_robot: 0,
                        obs_robot: 0,
                        geode_robot: 0,
                    },
                )
        })
        .sum();
    assert_eq!(s, 1009);
}
pub fn pb2() {
    let blueprints = parse(&INPUT);
    let s: i32 = blueprints
        .iter()
        .take(3)
        .map(|b| {
            get_best(
                &b,
                State {
                    time_left: 32,
                    ore: 0,
                    clay: 0,
                    obs: 0,
                    geode: 0,
                    ore_robot: 1,
                    clay_robot: 0,
                    obs_robot: 0,
                    geode_robot: 0,
                },
            )
        })
        .product();
    assert_eq!(s, 18816);
}

fn get_best(blueprint: &Blueprint, start_state: State) -> i32 {
    let mut scanned: HashSet<State> = HashSet::with_capacity(50 * 1024 * 1024);
    let mut to_scan = Vec::with_capacity(50 * 1024 * 1024);
    to_scan.push(start_state);

    // we use the current best state in terms of geodes
    // to discover if the evaluated state that will always be worse than the current best
    // if it is, we stop there.
    let mut best_geode = 0;
    let mut best_state = State::default();

    // as we can only build one robot a turn,
    // there is no point in producing more than what we can spend
    let max_ore = *[
        blueprint.clay_cost_in_ore,
        blueprint.ore_cost_in_ore,
        blueprint.obs_cost_in_ore,
        blueprint.geode_cost_in_ore,
    ]
    .iter()
    .max()
    .unwrap();
    let max_obsidian = blueprint.geode_cost_in_obsidian;
    let max_clay = blueprint.obs_cost_in_clay;

    while let Some(state) = to_scan.pop() {
        let geode_expected = state.geode + state.time_left * state.geode_robot;
        if state.time_left == 1 {
            if best_geode < geode_expected {
                best_geode = geode_expected;
                best_state = state;
            }
            continue; // end of investigation
        }

        if worse(best_state, state) {
            continue; // will always be worse than the best
        }

        if scanned.contains(&state) {
            continue; // already processed.
        }
        scanned.insert(state);

        if best_geode < geode_expected {
            best_geode = geode_expected;
            best_state = state;
        }

        to_scan.push(dig(state));
        // geode robot
        if state.ore >= blueprint.geode_cost_in_ore && state.obs >= blueprint.geode_cost_in_obsidian
        {
            let mut new_state = dig(state);
            new_state.geode_robot += 1;
            new_state.obs -= blueprint.geode_cost_in_obsidian;
            new_state.ore -= blueprint.geode_cost_in_ore;
            to_scan.push(new_state);
            continue; // seems to always be better to build a geode robot when we
        }
        // obsidian robot
        if state.ore >= blueprint.obs_cost_in_ore
            && state.clay >= blueprint.obs_cost_in_clay
            && !dont_need_more(state.time_left, state.obs, state.obs_robot, max_obsidian)
        {
            let mut new_state = dig(state);
            new_state.obs_robot += 1;
            new_state.clay -= blueprint.obs_cost_in_clay;
            new_state.ore -= blueprint.obs_cost_in_ore;
            to_scan.push(new_state);
        }
        // clay robot payoffs in >3turns
        if state.ore >= blueprint.clay_cost_in_ore
            && !dont_need_more(state.time_left, state.clay, state.clay_robot, max_clay)
            && state.time_left > 3
        {
            let mut new_state = dig(state);
            new_state.clay_robot += 1;
            new_state.ore -= blueprint.clay_cost_in_ore;
            to_scan.push(new_state);
        }
        // ore robot
        if state.ore >= blueprint.ore_cost_in_ore
            && !dont_need_more(state.time_left, state.ore, state.ore_robot, max_ore)
        {
            let mut new_state = dig(state);
            new_state.ore_robot += 1;
            new_state.ore -= blueprint.ore_cost_in_ore;
            to_scan.push(new_state);
        }
    }
    best_geode
}

fn dont_need_more(time_left: i32, stock: i32, robots: i32, max: i32) -> bool {
    robots >= max || time_left * robots + stock > time_left * max
}

// worse if it has the same or less robots for the same or less stock
// or if even if building one geode robot per turn til the end, it cannot catch up
fn worse(max: State, other: State) -> bool {
    (max.time_left == other.time_left
        && max.geode_robot >= other.geode_robot
        && max.obs_robot >= other.obs_robot
        && max.clay_robot >= other.clay_robot
        && max.ore_robot >= other.ore_robot
        && max.geode >= other.geode
        && max.obs >= other.obs
        && max.clay >= other.clay
        && max.ore >= other.ore)
        || other.geode
            + other.geode_robot * other.time_left
            + other.time_left * (other.time_left / 2)
            <= max.geode + max.geode_robot * max.time_left
}

fn dig(mut state: State) -> State {
    state.ore += state.ore_robot;
    state.clay += state.clay_robot;
    state.obs += state.obs_robot;
    state.geode += state.geode_robot;
    state.time_left -= 1;
    state
}

#[derive(Debug, Default, PartialEq, Eq, Clone, Copy, Hash)]
struct State {
    time_left: i32,
    ore: i32,
    clay: i32,
    obs: i32,
    geode: i32,
    ore_robot: i32,
    clay_robot: i32,
    obs_robot: i32,
    geode_robot: i32,
}
#[derive(Debug, PartialEq, Eq)]
struct Blueprint {
    idx: i32,
    ore_cost_in_ore: i32,
    clay_cost_in_ore: i32,
    obs_cost_in_ore: i32,
    obs_cost_in_clay: i32,
    geode_cost_in_ore: i32,
    geode_cost_in_obsidian: i32,
}

fn parse(input: &str) -> Vec<Blueprint> {
    input
        .lines()
        .map(|l| {
            let el = l.split_ascii_whitespace().collect_vec();
            Blueprint {
                idx: el[1][..el[1].len() - 1].parse::<i32>().unwrap(),
                ore_cost_in_ore: el[6].parse::<i32>().unwrap(),
                clay_cost_in_ore: el[12].parse::<i32>().unwrap(),
                obs_cost_in_ore: el[18].parse::<i32>().unwrap(),
                obs_cost_in_clay: el[21].parse::<i32>().unwrap(),
                geode_cost_in_ore: el[27].parse::<i32>().unwrap(),
                geode_cost_in_obsidian: el[30].parse::<i32>().unwrap(),
            }
        })
        .collect_vec()
}

#[allow(dead_code)]
const INPUT_TEST: &str = "\
Blueprint 1: Each ore robot costs 4 ore. Each clay robot costs 2 ore. Each obsidian robot costs 3 ore and 14 clay. Each geode robot costs 2 ore and 7 obsidian.
Blueprint 2: Each ore robot costs 2 ore. Each clay robot costs 3 ore. Each obsidian robot costs 3 ore and 8 clay. Each geode robot costs 3 ore and 12 obsidian.";

#[allow(dead_code)]
const INPUT: &str = "\
Blueprint 1: Each ore robot costs 2 ore. Each clay robot costs 4 ore. Each obsidian robot costs 4 ore and 17 clay. Each geode robot costs 3 ore and 11 obsidian.
Blueprint 2: Each ore robot costs 3 ore. Each clay robot costs 3 ore. Each obsidian robot costs 3 ore and 20 clay. Each geode robot costs 2 ore and 12 obsidian.
Blueprint 3: Each ore robot costs 4 ore. Each clay robot costs 4 ore. Each obsidian robot costs 4 ore and 12 clay. Each geode robot costs 3 ore and 8 obsidian.
Blueprint 4: Each ore robot costs 3 ore. Each clay robot costs 4 ore. Each obsidian robot costs 4 ore and 8 clay. Each geode robot costs 2 ore and 10 obsidian.
Blueprint 5: Each ore robot costs 4 ore. Each clay robot costs 4 ore. Each obsidian robot costs 2 ore and 12 clay. Each geode robot costs 3 ore and 15 obsidian.
Blueprint 6: Each ore robot costs 3 ore. Each clay robot costs 4 ore. Each obsidian robot costs 4 ore and 5 clay. Each geode robot costs 4 ore and 8 obsidian.
Blueprint 7: Each ore robot costs 2 ore. Each clay robot costs 3 ore. Each obsidian robot costs 3 ore and 8 clay. Each geode robot costs 3 ore and 20 obsidian.
Blueprint 8: Each ore robot costs 4 ore. Each clay robot costs 3 ore. Each obsidian robot costs 2 ore and 13 clay. Each geode robot costs 2 ore and 9 obsidian.
Blueprint 9: Each ore robot costs 4 ore. Each clay robot costs 3 ore. Each obsidian robot costs 3 ore and 15 clay. Each geode robot costs 2 ore and 13 obsidian.
Blueprint 10: Each ore robot costs 3 ore. Each clay robot costs 4 ore. Each obsidian robot costs 4 ore and 18 clay. Each geode robot costs 3 ore and 8 obsidian.
Blueprint 11: Each ore robot costs 3 ore. Each clay robot costs 3 ore. Each obsidian robot costs 3 ore and 8 clay. Each geode robot costs 2 ore and 12 obsidian.
Blueprint 12: Each ore robot costs 4 ore. Each clay robot costs 4 ore. Each obsidian robot costs 2 ore and 17 clay. Each geode robot costs 3 ore and 11 obsidian.
Blueprint 13: Each ore robot costs 2 ore. Each clay robot costs 4 ore. Each obsidian robot costs 3 ore and 19 clay. Each geode robot costs 4 ore and 13 obsidian.
Blueprint 14: Each ore robot costs 4 ore. Each clay robot costs 4 ore. Each obsidian robot costs 2 ore and 15 clay. Each geode robot costs 3 ore and 16 obsidian.
Blueprint 15: Each ore robot costs 3 ore. Each clay robot costs 4 ore. Each obsidian robot costs 3 ore and 12 clay. Each geode robot costs 3 ore and 17 obsidian.
Blueprint 16: Each ore robot costs 3 ore. Each clay robot costs 4 ore. Each obsidian robot costs 4 ore and 18 clay. Each geode robot costs 2 ore and 11 obsidian.
Blueprint 17: Each ore robot costs 2 ore. Each clay robot costs 4 ore. Each obsidian robot costs 2 ore and 15 clay. Each geode robot costs 3 ore and 16 obsidian.
Blueprint 18: Each ore robot costs 3 ore. Each clay robot costs 4 ore. Each obsidian robot costs 2 ore and 19 clay. Each geode robot costs 2 ore and 12 obsidian.
Blueprint 19: Each ore robot costs 3 ore. Each clay robot costs 3 ore. Each obsidian robot costs 2 ore and 16 clay. Each geode robot costs 2 ore and 18 obsidian.
Blueprint 20: Each ore robot costs 3 ore. Each clay robot costs 3 ore. Each obsidian robot costs 3 ore and 19 clay. Each geode robot costs 3 ore and 17 obsidian.
Blueprint 21: Each ore robot costs 3 ore. Each clay robot costs 4 ore. Each obsidian robot costs 4 ore and 20 clay. Each geode robot costs 4 ore and 16 obsidian.
Blueprint 22: Each ore robot costs 3 ore. Each clay robot costs 3 ore. Each obsidian robot costs 3 ore and 16 clay. Each geode robot costs 3 ore and 20 obsidian.
Blueprint 23: Each ore robot costs 4 ore. Each clay robot costs 4 ore. Each obsidian robot costs 3 ore and 5 clay. Each geode robot costs 4 ore and 11 obsidian.
Blueprint 24: Each ore robot costs 2 ore. Each clay robot costs 4 ore. Each obsidian robot costs 2 ore and 16 clay. Each geode robot costs 4 ore and 12 obsidian.
Blueprint 25: Each ore robot costs 2 ore. Each clay robot costs 4 ore. Each obsidian robot costs 3 ore and 20 clay. Each geode robot costs 2 ore and 17 obsidian.
Blueprint 26: Each ore robot costs 4 ore. Each clay robot costs 4 ore. Each obsidian robot costs 2 ore and 11 clay. Each geode robot costs 2 ore and 7 obsidian.
Blueprint 27: Each ore robot costs 4 ore. Each clay robot costs 3 ore. Each obsidian robot costs 2 ore and 19 clay. Each geode robot costs 3 ore and 10 obsidian.
Blueprint 28: Each ore robot costs 4 ore. Each clay robot costs 4 ore. Each obsidian robot costs 3 ore and 14 clay. Each geode robot costs 4 ore and 8 obsidian.
Blueprint 29: Each ore robot costs 4 ore. Each clay robot costs 3 ore. Each obsidian robot costs 3 ore and 10 clay. Each geode robot costs 3 ore and 10 obsidian.
Blueprint 30: Each ore robot costs 4 ore. Each clay robot costs 4 ore. Each obsidian robot costs 3 ore and 7 clay. Each geode robot costs 4 ore and 11 obsidian.";
