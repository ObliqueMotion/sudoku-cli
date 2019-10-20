pub mod sudoku;

use sudoku::board::SudokuBoard;

fn main() {
    let board = SudokuBoard::default();
    println!("{}", board);
}
