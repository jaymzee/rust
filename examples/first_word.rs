fn main() {
    println!("first word: {}", first_word("hello world"));
    let s = String::from("hello world");
    let fw = first_word(&s);
    println!("first word: {}", fw);
}

fn first_word(s: &str) -> &str {
    for (i, b) in s.bytes().enumerate() {
        if b == b' ' {
            return &s[..i];
        }
    }
    &s
}
