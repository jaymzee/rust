
macro_rules! myvec {
    ( $($x:expr),* ) => {
        {
            let mut temp_vec = Vec::new();
            $(
                temp_vec.push($x);
            )*
            temp_vec
        }
    };
}

fn main() {
    let numbers:Vec<i32> = myvec![1,2,3];
    println!("{:?}", numbers);
}
