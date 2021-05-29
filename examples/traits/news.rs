pub trait Summary {
    fn summarize(&self) -> String;
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

pub fn notify(item: &impl Summary) {
    println!("Breading news! {}", item.summarize());
}

pub fn notify2<T: Summary>(item: &T) {
    println!("Breading news! {}", item.summarize());
}

fn main() {
    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from(
            "of course, as you probably alread know, people",
        ),
        reply: false,
        retweet: false,
    };
    let article = NewsArticle {
        headline: String::from("Flood"),
        location: String::from("Florida"),
        author: String::from("Joe"),
        content: String::from("blah blah"),
    };
    let stuff: Vec<&dyn Summary> = vec![&tweet, &article];
    println!("1 new tweet: {}", stuff[0].summarize());
    println!("1 new article: {}", stuff[1].summarize());

    notify(&tweet);
    notify(&article);
    notify2(&tweet);
}
