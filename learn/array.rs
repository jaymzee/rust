// print slice of int32
fn foo(n: &str, slice: &[i32]) {
    print!("{} = [", n);
    for v in slice.iter() {
        print!(" {}", v);
    }
    println!(" ] len={}", slice.len());
}

// mutate slice of int32, then print it
fn bar(n: &str, a: &mut[i32]) {
    if a.len() > 1 {
        a[1] += 1;
    }
    foo(n, a);
}

fn main() {
    // declare arrays
    let mut x: [i32; 3] = [1, 2, 3];
    let y: [i32; 5] = [5, 7, 9, 11, 42];

    foo("x", &x);
    bar("x", &mut x);
    foo("x", &x);
    foo("y", &y);
}
