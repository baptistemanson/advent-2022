#![feature(iter_array_chunks)]
mod day1;
mod day2;
mod day3;
mod day3bis;

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
        _ => panic!("unknown problem"),
    }
}
