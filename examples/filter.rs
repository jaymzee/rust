fn main() {
    let nums = vec![1,2,3,4,5];

    let list: Vec<_> = nums.iter().cloned().filter(|&n| n < 4).collect();
    println!("{:?}", nums);
    println!("{:?}", list);
    let names = vec!["joe", "peter", "paul"];
    let filtered: Vec<_> = names.into_iter()
        .filter(|&name| name < "p")
        .collect();
    println!("{:?}", filtered);
}
