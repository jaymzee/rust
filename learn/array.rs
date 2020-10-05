// just print the array (array as a slice)
fn foo(n: &str, a: &[i32]) {
    print!("{} = [", n);
    for e in a.iter() {
        print!(" {}", e);
    }
    println!(" ] len={}", a.len());
}

// mutate then print array (array as a slice)
fn bar(n: &str, a: &mut[i32]) {
    if a.len() > 1 {
        a[1] += 1;
    }
    foo(n, a);
}

fn main() {
    let mut x: [i32; 3] = [1, 2, 3];
    let y: [i32; 5] = [5, 7, 9, 11, 42];

    foo("x", &x);
    bar("x", &mut x);
    foo("x", &x);
    foo("y", &y);
}
