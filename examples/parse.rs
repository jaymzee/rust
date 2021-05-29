fn parse_i32(s: &str) -> i32 {
    match s.parse() {
        Ok(i) => i,
        Err(_e) => -1,
    }
}

fn main() {
    let s = "42";
    let i: i32 = s.parse().unwrap_or(0);

    println!("{}", i);
    println!("{}", parse_i32("43"));
    println!("{}", parse_i32("x"));
}
