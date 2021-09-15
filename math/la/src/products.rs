use ndarray::{arr1, arr2};

pub fn matrix_vector_product_example() {
    let c = 4;
    let x = arr1(&[1, 2, 3]);
    let a = arr2(&[[4, 5, 6],
                   [7, 8, 9]]);

    let y = a.dot(&(c * x));
    println!("{}", y);
}

pub fn matrix_matrix_product_example() {
    let a = arr2(&[[1, 2, 3],
                   [4, 5, 6]]);
    let b = arr2(&[[6, 3],
                   [5, 2],
                   [4, 1]]);
    println!("{}", a.dot(&b));
}
