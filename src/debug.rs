pub fn display<T: ToString>(t: &Vec<Vec<T>>) {
    println!("");
    println!("");
    print!(
        "{}",
        t.iter()
            .map(|l| { l.iter().map(|a| a.to_string()).join(",") })
            .join("\n")
    );
}
