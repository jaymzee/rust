fn mean(data: &[f64]) -> Option<f64> {
    match data.len() {
        n if n > 0 => {
            let sum: f64 = data.iter().sum();
            Some(sum / n as f64)
        },
        _ => None,
    }
}

fn variance(x: &[f64]) -> Option<f64> {
    match mean(x) {
        Some(mu) => {
            let n = x.len() as f64;
            let x2s: f64 = x.iter().map(|x| x*x).sum();
            Some(x2s / n - mu*mu)
        },
        _ => None
    }
}

fn std_deviation(x: &[f64]) -> Option<f64> {
    match variance(x) {
        Some(var) => Some(var.sqrt()),
        _ => None
    }
}

fn main() {
    let data = [3., 1., 6., 1., 5., 8., 1., 8., 10., 11.];

    let data_mean = mean(&data);
    println!("Mean is {:?}", data_mean);

    let data_std_deviation = std_deviation(&data);
    println!("Standard deviation is {:?}", data_std_deviation);

    let zscore = match (data_mean, data_std_deviation) {
        (Some(mean), Some(std_deviation)) => {
            let diff = data[4] as f64 - mean;

            Some(diff / std_deviation)
        },
        _ => None
    };
    println!("Z-score of data at index 4 (with value {}) is {:?}",
             data[4], zscore);
}
