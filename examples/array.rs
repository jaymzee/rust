fn print_i32(name: &str, slice: &[i32], pretty: bool) {
    print!("{} = [", name);
    if pretty {
        println!();
    }
    for (i, v) in slice.iter().enumerate() {
        if pretty {
            print!("    ");
        } else if i > 0 {
            print!(" ");
        }
        print!("{}", v);
        if pretty {
            println!(",");
        }
    }
    println!("] len={}", slice.len());
}

// mutate array, then print it
fn bar(name: &str, slice: &mut[i32]) {
    if slice.len() > 1 {
        slice[1] += 1;
    }
    print_i32(name, slice, false);
}

fn main() {
    // fixed size arrays
    let mut x = [1, 2, 3];
    let y = [5, 7, 9, 11, 42];
    let mut z = vec![8, 9, 10, 11, 12, 13];

    z.push(42);
    z.push(88);

    print_i32("x", &x, false);
    println!("x = {:?}", x);
    println!("x = {:#?}", x);
    bar("x", &mut x);
    print_i32("y", &y, false);
    print_i32("y[1..4]", &y[1..4], false);
    print_i32("z", &z, false);
    print_i32("z[1..6]", &z[1..6], true);
    if let Some(num) = z.pop() {
        println!("num = {}", num);
    }
}
