fn main() {
    let data1 = [3., 1., 6., 1., 5., 8., 1., 8., 10., 11.];
    let data2 = [];
    stats(&data1);
    stats(&data2);
}

fn stats(nums: &[f64]) {
    let sum: f64 = nums.iter().sum();
    let mean = match nums.len() {
        n if n > 0 => Some(sum / n as f64),
        _ => None
    };
    println!("data = {:?}", nums);
    println!(" sum = {}", sum);
    println!(" mean = {:?}", mean);
}
