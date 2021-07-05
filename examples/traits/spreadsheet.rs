fn main() {
    let mut sheet = Spreadsheet::new();

    sheet.cells[0][0] = SpreadsheetCell::Text(String::from("Hello, World!"));
    sheet.cells[0][1] = SpreadsheetCell::Integer(42);
    sheet.cells[1][0] = SpreadsheetCell::Float(3.14);
    sheet.cells[1][1] = SpreadsheetCell::Formula("=A1+B1".to_string());

    sheet.render();
}

struct Spreadsheet {
    cells: [[SpreadsheetCell; Self::COLS]; Self::ROWS],
}

impl Spreadsheet {
    const ROWS: usize = 12;
    const COLS: usize = 10;

    fn new() -> Self {
        const ROWS: usize = Spreadsheet::ROWS;
        const COLS: usize = Spreadsheet::COLS;
        const CELL: SpreadsheetCell = SpreadsheetCell::Empty;
        const ROW: [SpreadsheetCell; COLS] = [CELL; COLS];
        Self { cells: [ROW; ROWS] }
    }

    fn render(&self) {
        for row in 0..Self::ROWS {
            for col in 0..Self::COLS {
                self.cells[row][col].render();
            }
            println!();
        }
    }
}

#[derive(Debug)]
enum SpreadsheetCell {
    Empty,
    Integer(i32),
    Float(f64),
    Text(String),
    Formula(String),
}

impl SpreadsheetCell {
    fn render(&self) {
        print!("{:?}\t", self);
    }
}
