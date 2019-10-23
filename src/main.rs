pub mod sudoku;
use rayon::ThreadPoolBuilder;
use std::time::Instant;
use sudoku::board::SudokuBoard;
use structopt::StructOpt;
use std::fs;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use crate::SudokuError::{IOError, RayonError};

#[derive(Debug)]
enum SudokuError {
    IOError(std::io::Error),
    RayonError(rayon::ThreadPoolBuildError),
}

impl From<std::io::Error> for SudokuError {
    fn from(e: std::io::Error) -> Self {
        IOError(e)
    }
}

impl From<rayon::ThreadPoolBuildError> for SudokuError {
    fn from(e: rayon::ThreadPoolBuildError) -> Self {
        RayonError(e)
    }
}

#[derive(StructOpt, Debug)]
enum Action {
    Write(Output),
    Watch(Watch),
    Count(Count),
}

#[derive(StructOpt, Debug)]
struct Output {
    /// The path to a puzzle or a puzzle string (for examples: sudoku-cli --help)
    #[structopt(short = "i", long = "input")]
    input: String,

    /// The number of threads to use [default: CPU count]
    #[structopt(short = "t", long = "threads")]
    threads: Option<usize>,

    /// The path to a file to which the solutions will be written.
    #[structopt(short = "o", long = "output")]
    output: Option<String>,
    
    /// Writes solutions as a compact string of 81 consecutive digits.
    #[structopt(short = "c", long = "compact")]
    compact: bool,
}

#[derive(StructOpt, Debug)]
struct Watch {
    /// The path to a puzzle or a puzzle string (for examples: sudoku-cli --help)
    #[structopt(short = "i", long = "input")]
    input: String,

    /// The number of milliseconds per frame
    #[structopt(short = "m", long = "ms-per-frame", default_value = "50")]
    ms_per_frame: u64,
}

#[derive(StructOpt, Debug)]
struct Count {
    /// The path to a puzzle or a puzzle string (for examples: sudoku-cli --help)
    #[structopt(short = "i", long = "input")]
    input: String,

    /// The number of threads to use [default: CPU count]
    #[structopt(short = "t", long = "threads")]
    threads: Option<usize>, 
}

/// 
/// EXAMPLES:
/// 
/// — sudoku-cli count --input=".75.....42139.5.7...8.7...9..2417...4...6...1...8324..3...9.7...5.3.46988.....31."
/// 
/// — sudoku-cli count --input=path/to/puzzle --threads=1 (defaults to CPU count if --threads is unspecified)
/// 
/// — sudoku-cli watch --input=path/to/puzzle --ms-per-frame=40 (defaults to 50 milliseconds per frame)
/// 
/// — sudoku-cli write --input=path/to/puzzle --compact (defaults to terminal output)
/// 
/// — sudoku-cli write --input=path/to/puzzle --output=path/to/output-file
/// 
/// For more details on each subcommand: sudoku-cli <SUBCOMMAND> --help
#[derive(StructOpt, Debug)]
#[structopt(name = "sudoku-cli")]
struct Opt {
    #[structopt(subcommand)]
    action: Action,
}

fn build_thread_pool(threads: Option<usize>) -> Result<(), SudokuError> {
    let num_threads = threads.unwrap_or(num_cpus::get());
    ThreadPoolBuilder::new().num_threads(num_threads).build_global().map_err(SudokuError::from)
}

fn puzzle_input(input: &str) -> Result<String, SudokuError> {
    if Path::new(input).exists() {
        fs::read_to_string(input).map_err(SudokuError::from)
    } else {
        Ok(String::from(input))
    }
}


fn main() -> Result<(), SudokuError> {
    match Opt::from_args().action {
        Action::Watch(opts) => {
            let mut board = SudokuBoard::from(puzzle_input(&opts.input)?);
            board.watch_find_solutions(opts.ms_per_frame)
        },
        Action::Count(opts) => {
            build_thread_pool(opts.threads)?;
            let mut board = SudokuBoard::from(puzzle_input(&opts.input)?);
            println!("\n{}", board);
            let now = Instant::now();
            let count = board.count_solutions();
            let elapsed = now.elapsed();
            println!("  Found: {} solution(s)\n", count);
            println!("  Time:  {} second(s)\n", elapsed.as_secs_f64());
        },
        Action::Write(opts) => {
            build_thread_pool(opts.threads)?;
            let mut board = SudokuBoard::from(puzzle_input(&opts.input)?);
            println!("\n{}", board);
            let now = Instant::now();
            let solutions = if opts.compact {
                board.find_solutions_compact()
            } else {
                board.find_solutions()
            };
            let elapsed = now.elapsed();
            let count = board.count_solutions();
            if let Some(path) = opts.output {
                let mut file = File::create(&path)?;
                file.write_all(solutions.as_bytes())?;
                println!("  Found: {} solution(s)\n", count);
                println!("  Time:  {} second(s)\n", elapsed.as_secs_f64());
                println!("  Writing solutions to file: {}\n\n", path);
            } else {
                println!("  Solutions:\n\n{}", solutions);
                println!("  Found: {} solution(s)\n", count);
                println!("  Time:  {} second(s)\n", elapsed.as_secs_f64());
            }
        },
    }
    Ok(())
}
