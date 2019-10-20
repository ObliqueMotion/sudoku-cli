pub mod sudoku;

use sudoku::board::SudokuBoard;

fn main() {
    let board = SudokuBoard::default();
    board.insert(5, 0, 3);
    board.insert(7, 3, 1);
    board.insert(2, 5, 7);
    println!("\n{}", board);
}
