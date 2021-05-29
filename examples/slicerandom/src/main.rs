use rand::seq::SliceRandom;

fn main() {
    let mut x = vec![11, 22, 33];
    let mut rng = rand::thread_rng();
    let choice = x.choose(&mut rng).unwrap();

    println!("you picked {}", choice);
    println!("{:?}", x);
    x.shuffle(&mut rng);
    println!("{:?}", x);
}
