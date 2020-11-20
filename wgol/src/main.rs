use wgol::Universe;                 // wasm-game-of-life

fn main() {
    let mut u = Universe::new();    // create universe
    print!("\x1b[2J");              // clear screen
    for i in 1..1000 {
        println!("\x1b[1;1H{}", u); // home cursor and print universe
        u.tick();                   // update universe
    }
}
