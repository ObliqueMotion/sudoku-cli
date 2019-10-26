pub mod sudoku;
use crate::SudokuError::{IOError, RayonError};
use ansi_escapes::ClearScreen;
use rayon::ThreadPoolBuilder;
use std::fs;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::time::Instant;
use structopt::StructOpt;
use sudoku::board::SudokuBoard;

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

/// Toast
#[derive(StructOpt, Debug)]
enum Action {
    /// Finds all possible solutions and writes them to a specified output.
    FindAll(Output),
    /// Finds one possible solution and writes it to a specified output.
    FindOne(Output),
    /// Watch the solver find one solution to a puzzle.
    WatchOne(Watch),
    /// Watch the solver find all solutions to a puzzle.
    WatchAll(Watch),
    /// Counts the number of solutions to a sudoku puzzle.
    CountAll(Count),
}

#[derive(StructOpt, Debug)]
struct Output {
    /// The path to a puzzle or a puzzle string (for examples: sudoku-cli help)
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
    /// The path to a puzzle or a puzzle string (for examples: sudoku-cli help)
    #[structopt(short = "i", long = "input")]
    input: String,

    /// The number of milliseconds per frame
    #[structopt(short = "m", long = "ms-per-frame", default_value = "50")]
    ms_per_frame: u64,
}

#[derive(StructOpt, Debug)]
struct Count {
    /// The path to a puzzle or a puzzle string (for examples: sudoku-cli help)
    #[structopt(short = "i", long = "input")]
    input: String,

    /// The number of threads to use [default: CPU count]
    #[structopt(short = "t", long = "threads")]
    threads: Option<usize>,
}

/// ABOUT:
///
/// │   A command-line tool for solving sudoku. There are five sub-commands (see usage details below):
///
/// │   find-one, find-all, watch-one, watch-all, count-all
///
/// INPUT:
///
/// │   If your input is a file path, sudoku-cli will read the file. Otherwise it treats the string as input.
///
/// │   sudoku-cli reads the first 81 non-whitespace characters and fills each row from left to right starting with the top.
/// |   Any digits (1-9) will show up on the board. All other characters will count as a blank square.
///
/// │   Valid Input: .75.....42139.5.7...8.7...9..2417...4...6...1...8324..3...9.7...5.3.46988.....31.
///
/// │   Valid Input: ./path/to/puzzle
///
/// OUTPUT:
///
/// │   The directory to a specified output file must already exist.
///
/// EXAMPLES:
///
/// │   sudoku-cli find-one  --input=".75.....4.1...5.....8.7.........7.......6...1...8.2...3...9.7...5.3.4.........31."
///
/// │   sudoku-cli find-all  --input=".75.....4.1...5.....8.7.........7.......6...1...8.2...3...9.7...5.3.4.........31."
///
/// |   sudoku-cli find-all  --input=path/to/puzzle --threads=3 --compact
///
/// |   sudoku-cli find-all  --input=path/to/puzzle --output=path/to/output/file
///
/// |   sudoku-cli watch-all --input=path/to/puzzle --ms-per-frame=5
///
/// |   sudoku-cli watch-one --input=path/to/puzzle
///
/// │   sudoku-cli count-all --input=path/to/puzzle
///
/// │   For more details on each subcommand: sudoku-cli help <SUBCOMMAND>
#[derive(StructOpt, Debug)]
#[structopt(name = "sudoku-cli")]
struct Opt {
    #[structopt(subcommand)]
    action: Action,
}

fn build_thread_pool(threads: Option<usize>) -> Result<(), SudokuError> {
    let num_threads = threads.unwrap_or(num_cpus::get());
    ThreadPoolBuilder::new()
        .num_threads(num_threads)
        .build_global()
        .map_err(SudokuError::from)
}

fn puzzle_input(input: &str) -> Result<String, SudokuError> {
    if Path::new(input).exists() {
        fs::read_to_string(input).map_err(SudokuError::from)
    } else {
        Ok(String::from(input))
    }
}

fn print_count(count: usize) {
    if 1 == count {
        println!("  Found: 1 solution\n");
    } else {
        println!("  Found: {} solutions\n", count);
    }
}

fn main() -> Result<(), SudokuError> {
    println!("{}", ClearScreen);
    match Opt::from_args().action {
        Action::WatchOne(opts) => {
            let mut board = SudokuBoard::from(puzzle_input(&opts.input)?);
            board.watch_find_one(opts.ms_per_frame)
        }
        Action::WatchAll(opts) => {
            let mut board = SudokuBoard::from(puzzle_input(&opts.input)?);
            board.watch_find_all(opts.ms_per_frame)
        }
        Action::CountAll(opts) => {
            build_thread_pool(opts.threads)?;
            let mut board = SudokuBoard::from(puzzle_input(&opts.input)?);
            println!("\n{}", board);
            let now = Instant::now();
            let count = board.count_solutions();
            let elapsed = now.elapsed();
            print_count(count);
            println!("  Time:  {} seconds\n", elapsed.as_secs_f64());
        }
        Action::FindOne(opts) => {
            build_thread_pool(opts.threads)?;
            let mut board = SudokuBoard::from(puzzle_input(&opts.input)?);
            println!("\n{}", board);
            let now = Instant::now();
            let (count, solutions) = if opts.compact {
                board.find_one_compact()
            } else {
                board.find_one()
            };
            let elapsed = now.elapsed();
            if let Some(path) = opts.output {
                let mut file = File::create(&path)?;
                file.write_all(solutions.as_bytes())?;
                print_count(count);
                println!("  Time:  {} seconds\n", elapsed.as_secs_f64());
                println!("  Writing solution to file: {}\n\n", path);
            } else {
                println!("  Solutions:\n\n{}", solutions);
                print_count(count);
                println!("  Time:  {} seconds\n", elapsed.as_secs_f64());
            }
        }
        Action::FindAll(opts) => {
            build_thread_pool(opts.threads)?;
            let mut board = SudokuBoard::from(puzzle_input(&opts.input)?);
            println!("\n{}", board);
            let now = Instant::now();
            let (count, solutions) = if opts.compact {
                board.find_all_compact()
            } else {
                board.find_all()
            };
            let elapsed = now.elapsed();
            if let Some(path) = opts.output {
                let mut file = File::create(&path)?;
                file.write_all(solutions.as_bytes())?;
                print_count(count);
                println!("  Time:  {} seconds\n", elapsed.as_secs_f64());
                if 1 == count {
                    println!("  Writing solution to file: {}\n\n", path);
                } else {
                    println!("  Writing solutions to file: {}\n\n", path);
                }
            } else {
                println!("  Solutions:\n\n{}", solutions);
                print_count(count);
                println!("  Time:  {} seconds\n", elapsed.as_secs_f64());
            }
        }
    }
    Ok(())
}
