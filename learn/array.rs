// just print the array
fn foo(a: &[i32; 3]) {
    print!("[");
    for x in a.iter() {
        print!(" {}", x);
    }
    println!(" ]");
}

// mutate then print array
fn bar(a: &mut[i32; 3]) {
    a[1] += 1;
    foo(a);
}

fn main() {
    let mut arr: [i32; 3] = [1, 2, 3];

    foo(&arr);
    bar(&mut arr);
    foo(&arr);
}
