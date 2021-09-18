use nalgebra::DMatrix;

pub fn example()-> Result<(), std::io::Error> {
    let row_slice: Vec<i32> = (1..=5000).collect();
    let matrix = DMatrix::from_row_slice(50, 100, &row_slice);

    let ser_matrix = serde_json::to_string(&matrix)?;
    let des_matrix: DMatrix<i32> = serde_json::from_str(&ser_matrix)?;

    assert!(des_matrix == matrix);

    Ok(())
}
