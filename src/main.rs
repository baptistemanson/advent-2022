mod day1;

fn main() {
    let pb = std::env::args().nth(1).expect("expected problem number");
    match pb.as_str() {
        "11" => day1::pb11(),
        "12" => day1::pb12(),
        _ => panic!("unknown problem"),
    }
}
