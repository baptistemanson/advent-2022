#![feature(iter_array_chunks)]
#![feature(int_roundings)]

use std::time::Instant;
mod day1;
mod day10;
mod day11;
mod day12;
mod day13;
mod day14;
mod day15;
mod day16;
mod day17;
mod day18;
mod day2;
mod day3;
mod day3bis;
mod day4;
mod day5;
mod day6;
mod day7;
mod day7bis;
mod day8;
mod day9;
mod debug;
mod playground;
use thousands::Separable;
fn main() {
    let pb = std::env::args().nth(1).expect("expected problem number");
    std::thread::Builder::new()
        .stack_size(50 * 1024 * 1024)
        .spawn(|| {
            execute(pb);
        })
        .unwrap()
        .join()
        .unwrap();
}

fn execute(pb: String) {
    let now = Instant::now();
    match pb.as_str() {
        "11" => day1::pb1(),
        "12" => day1::pb2(),
        "21" => day2::pb1(),
        "22" => day2::pb2(),
        "22bis" => day2::pb2bis(),
        "31" => day3::pb1(),
        "32" => day3::pb2(),
        "31bis" => day3bis::pb1(),
        "32bis" => day3bis::pb2(),
        "41" => day4::pb1(),
        "42" => day4::pb2(),
        "51" => day5::pb1(),
        "52" => day5::pb2(),
        "61" => day6::pb1(),
        "62" => day6::pb2(),
        "71" => day7::pb1(),
        "72" => day7::pb2(),
        "71bis" => day7bis::pb1(),
        "72bis" => day7bis::pb2(),
        "81" => day8::pb1(),
        "82" => day8::pb2(),
        "91" => day9::pb1(),
        "92" => day9::pb2(),
        "101" => day10::pb1(),
        "102" => day10::pb2(),
        "111" => day11::pb1(),
        "112" => day11::pb2(),
        "121" => day12::pb1(),
        "122" => day12::pb2(),
        "131" => day13::pb1(),
        "132" => day13::pb2(),
        "141" => day14::pb1(),
        "142" => day14::pb2(),
        "151" => day15::pb1(),
        "152" => day15::pb2(),
        "161" => day16::pb1(),
        "162" => day16::pb2(),
        "171" => day17::pb1(),
        "172" => day17::pb2(),
        "181" => day18::pb1(),
        "182" => day18::pb2(),
        "playground" => playground::main(),
        _ => panic!("unknown problem"),
    };
    println!(
        "took {} ms",
        now.elapsed().as_millis().separate_with_commas()
    );
}
