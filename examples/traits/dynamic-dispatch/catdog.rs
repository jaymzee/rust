trait Speaks {
     fn speak(&self);
}

trait Animal {
    fn animal_type(&self) -> &str;
    fn noise(&self) -> &str;
}

impl<T> Speaks for T where T: Animal {
    fn speak(&self) {
        println!("The {} said {}", self.animal_type(), self.noise());
    }
}

struct Dog {}
impl Animal for Dog {
    fn animal_type(&self) -> &str {
        "dog"
    }

    fn noise(&self) -> &str {
        "woof"
    }
}

struct Cat {}
impl Animal for Cat {
    fn animal_type(&self) -> &str {
        "cat"
    }

    fn noise(&self) -> &str {
        "meow"
    }
}

// dynamic dispatch of trait, similar to C++ abstract class.
// dispatch at runtime
fn display_sound(a: &dyn Animal) {
    println!("the sound made by {} is {}", a.animal_type(), a.noise())
}

fn main() {
    let dog = Dog {};
    let cat = Cat {};
    dog.speak();
    cat.speak();
    display_sound(&cat);
    display_sound(&dog);
}
