use crate::sudoku::data::SudokuData;

#[derive(Debug)]
pub struct Square {
    value: u64,
    row: SudokuData,
    col: SudokuData,
    bx: SudokuData,
}

impl Square {
    pub fn new(row: SudokuData, col: SudokuData, bx: SudokuData) -> Self {
        Square { value: 0, row, col, bx }
    }

}