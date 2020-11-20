use std::collections::HashMap;

fn books() {
    let mut book_reviews = HashMap::new();

    book_reviews.insert(
        "Adventures of Huckleberry Finn",
        "My favorite book",
    );
    book_reviews.insert(
        "Pride and Prejudice",
        "Very enjoyable",
    );

    println!("{:#?}", book_reviews);
    println!("{:#?}", book_reviews["Pride and Prejudice"]);
}

fn timber() {
    let timber_resources: HashMap<&str, i32> = [
        ("Norway", 100),
        ("Denmark", 50),
        ("Iceland", 10),
    ].iter().cloned().collect();

    println!("{:#?}", timber_resources);
}

fn main() {
    books();
    timber();
}
