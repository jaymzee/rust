use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    println!("argc: {}", args.len());
    println!("argv: {:?}", args);
    println!("argv[0]: {}", args[0]);
}
