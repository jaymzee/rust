fn main() {
    let a = [1.0, 2.0, 3.0];
    let b = [4.0, 5.0];

    println!("{:?}", convolve(&a, &b));
    println!("{:?}", convolve(&b, &a));
}

fn convolve(x: &[f64], h: &[f64]) -> Vec<f64> {
    let xl = x.len();
    let hm = h.len();
    let mut y = vec![0.0; xl + hm - 1];

    for i in 0..y.len() {
        let mut k = i.min(hm - 1) + 1;
        let mut acc = 0.0;
        let start = hm.max(i+1) - hm;
        for j in start..xl.min(i+1) {
            k -= 1;
            acc += h[k] * x[j];
        }
        y[i] = acc;
    }

    y
}
