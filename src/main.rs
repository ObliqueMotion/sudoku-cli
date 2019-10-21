pub mod sudoku;
use std::time::Instant;
use sudoku::board::SudokuBoard;

static HARD_PUZZLE: &str = include_str!("../puzzles/50k");

fn main() {
    let mut board = SudokuBoard::from(HARD_PUZZLE);
    println!("\n{}", board);
    let now = Instant::now();
    let solutions = board.count_all_solutions();
    let time = now.elapsed();
    println!("  Found: {} solution(s)", solutions);
    println!("\n\n  {} second(s)\n\n", time.as_secs_f64())
}
