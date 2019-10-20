pub mod sudoku;
use sudoku::board::SudokuBoard;

static HARD_PUZZLE: &str = include_str!("../puzzles/hard");

fn main() {
    let board = SudokuBoard::default()
        .insert(5, 0, 3)
        .insert(7, 3, 1)
        .insert(2, 5, 7);
    println!("\n{}", board);

    let board2 = SudokuBoard::from(HARD_PUZZLE);
    println!("\n{}", board2);
}
