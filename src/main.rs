pub mod sudoku;
use std::time::Instant;
use sudoku::board::SudokuBoard;

static HARD_PUZZLE: &str = include_str!("../puzzles/50k");

fn main() {
    let board = SudokuBoard::from(HARD_PUZZLE);
    println!("\n{}", board);
    let now = Instant::now();
    println!("\nFound: {} solution(s).", board.solve());
    println!("\n\n{} second(s).\n\n", now.elapsed().as_secs_f64())
}
