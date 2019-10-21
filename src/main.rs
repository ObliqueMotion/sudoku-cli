pub mod sudoku;
use std::time::Instant;
use sudoku::board::SudokuBoard;

static HARD_PUZZLE: &str = include_str!("../puzzles/easy");

fn main() {
    let board = SudokuBoard::from(HARD_PUZZLE);
    println!("\n{}", board);
    let now = Instant::now();
    board.watch_find_all_solutions(100);
    println!("\n\n{} second(s).\n\n", now.elapsed().as_secs_f32())
}
