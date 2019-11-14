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

const ABOUT: &str = r#"
ABOUT:

    A command-line tool for solving sudoku. 
    
COMMANDS:
    
    sudoku-cli has five sub-commands, each with their own long and short options:
    
        Command: sudoku-cli find-one  
            Description: Finds one solution to a puzzle and writes it to an output.
                Long:    --input=value,  --output=value,  --threads=value,  --compact
                Short:        -i=value,        -o=value,         -t=value,         -c
                Default:      Required,        Terminal,   Number of CPUs,        Off
            
        Command: sudoku-cli find-all
            Description: Finds all solutions to a puzzle and writes them to an output.
                Long:    --input=value,  --output=value,  --threads=value,  --compact
                Short:        -i=value,        -o=value,         -t=value,         -c
                Default:      Required,        Terminal,   Number of CPUs,        Off
            
        Command: sudoku-cli watch-one 
            Description: Watch the solver find one solution in the terminal.
                Long:    --input=value,  --ms-per-frame=value
                Short:        -i=value,              -m=value
                Default:      Required,                    50
            
        Command: sodoku-cli watch-all 
            Description: Watch the solver find all solutions in the terminal.
                Long:    --input=value,  --ms-per-frame=value
                Short:        -i=value,              -m=value
                Default:      Required,                    50
            
        Command: sudoku-cli count-all
            Description: Count all solutions without writing them to an output.
                Long:    --input=value,  --threads=value
                Short:        -i=value,         -t=value
                Default:      Required,   Number of CPUs 

INPUT:

    If your input is a valid file path, sudoku-cli will read from the file.
    Otherwise it treats the input value as a string.

    sudoku-cli reads the first 81 non-whitespace characters from the input file or string. 
    
        - The first 9 of those characters are placed in the top row from left to right.
        - The next 9 characters are placed in the second row, and so on.
        - If a character is a digit, it will show on the board. Otherwise, it counts as a blank square.
        
    Example Inputs:

        --input=./path/to/puzzle
    
        --input=.75.....42139.5.7...8.7...9..2417...4...6...1...8324..3...9.7...5.3.46988.....31.
    
        --input="- 7 5 - - - - - 4
                 2 1 3 9 - 5 - 7 -
                 - - 8 - 7 - - - 9
                 - - 2 4 1 7 - - -
                 4 - - - 6 - - - 1
                 - - - 8 3 2 4 - -
                 3 - - - 9 - 7 - -
                 - 5 - 3 - 4 6 9 8
                 8 - - - - - 3 1 -"

OUTPUT:

    sudkou-cli can write to a new file, or overwrite an existing file; but it will not create a new directory.
    If you specify an output file, the path to the directory must already exist.

EXAMPLES:
    
    sudoku-cli find-one  --input=./path/to/puzzle
        Find one solution and print it to the terminal.

    sudoku-cli find-all  --input=path/to/puzzle
        Find all solutions and print them to the terminal.

    sudoku-cli find-all  --input=path/to/puzzle --threads=2 --compact
        Find all solutions using only 2 threads. Print solutiosn to the terminal in a compact format.

    sudoku-cli find-all  --input=path/to/puzzle --output=path/to/output/file
        Find all solutions and write them to a file.

    sudoku-cli watch-all --input=path/to/puzzle --ms-per-frame=5
        Watch the solver find all solutions at 5 milliseconds per frame.

    sudoku-cli watch-one --input=path/to/puzzle
        Watch the solver find one solution.

    sudoku-cli count-all --input=path/to/puzzle
        Count all solutions without writing them to an output.

MORE:

    For more details on each subcommand, use the help command:
    
        sudoku-cli help find-one
        sudoku-cli help find-all
        sudoku-cli help watch-one
        sudoku-cli help watch-all
        sudoku-cli help count-all
"#;
#[derive(StructOpt, Debug)]
#[structopt(name = "sudoku-cli", about = ABOUT)]
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
