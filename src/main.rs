#![feature(iter_array_chunks)]
mod day1;
mod day2;
mod day3;
mod day3bis;
mod day4;
mod day5;
mod day6;
mod day7;
mod day7bis;

fn main() {
    let pb = std::env::args().nth(1).expect("expected problem number");
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
        _ => panic!("unknown problem"),
    }
}
