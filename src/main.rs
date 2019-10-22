pub mod sudoku;
use rayon::ThreadPoolBuilder;
use std::time::Instant;
use sudoku::board::SudokuBoard;

static HARD_PUZZLE: &str = include_str!("../puzzles/1mil");

fn main() {
    ThreadPoolBuilder::default()
        .num_threads(8)
        .build_global()
        .expect("Failed to initlalize rayon.");
    let mut board = SudokuBoard::from(HARD_PUZZLE);

    //println!("\n\nStarting Board:\n\n{}", board);

    let now = Instant::now();
    let solutions = board.count_solutions_par();
    let time = now.elapsed();

    println!("{} solution(s)", solutions);
    println!("\nTime: {} second(s)\n\n", time.as_secs_f64());
}
