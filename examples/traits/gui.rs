fn main() {
    let mut screen = Screen::new();

    let button = Button {
        width: 3,
        height: 5,
        label: String::from("foo")
    };
    screen.add(Box::new(button));

    let select = SelectBox {
        width: 4,
        height: 6,
        options: vec![String::from("foo=bar")]
    };
    screen.add(Box::new(select));
    screen.run();
}

struct Screen {
    components: Vec<Box<dyn Draw>>,
}

impl Screen {
    fn new() -> Screen {
        Screen { components: vec![] }
    }

    fn run(&self) {
        for component in self.components.iter() {
            component.draw();
        }
    }

    fn add(&mut self, component: Box<dyn Draw>) {
        self.components.push(component);
    }
}

trait Draw {
    fn draw(&self);
}

#[derive(Debug)]
struct Button {
    width: u32,
    height: u32,
    label: String,
}

#[derive(Debug)]
struct SelectBox {
    width: u32,
    height: u32,
    options: Vec<String>,
}

impl Draw for Button {
    fn draw(&self) {
        println!("drawing Button, {:#?}", self);
    }
}

impl Draw for SelectBox {
    fn draw(&self) {
        println!("drawing SelectBox, {:#?}", self);
    }
}
