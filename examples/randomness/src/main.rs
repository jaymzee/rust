use rand::Rng;
use rand::distributions::{Distribution, Standard};

#[derive(Debug)]
enum Spinner {
    Cow,
    Sheep,
    Chicken
}

impl Distribution<Spinner> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Spinner {
        match rng.gen_range(0..=2) {
            0 => Spinner::Cow,
            1 => Spinner::Sheep,
            _ => Spinner::Chicken,
        }
    }
}

struct Sheep {}
struct Cow {}
struct Chicken {}

trait Animal {
    fn noise(&self) -> &'static str;
}

impl Animal for Sheep {
    fn noise(&self) -> &'static str {
        "baaaah!"
    }
}

impl Animal for Cow {
    fn noise(&self) -> &'static str {
        "mooooo!"
    }
}

impl Animal for Chicken {
    fn noise(&self) -> &'static str {
        "cluck!"
    }
}

fn random_animal() -> Box<dyn Animal> {
    let mut rng = rand::thread_rng();
    let roll: Spinner = rng.gen();

    match roll {
        Spinner::Cow => Box::new(Cow {}),
        Spinner::Chicken => Box::new(Chicken {}),
        Spinner::Sheep => Box::new(Sheep {}),
    }
}

fn main() {
    for _ in 0..10 {
        let animal = random_animal();
        println!("chosen animal at random and it says {}", animal.noise());
    }
}
