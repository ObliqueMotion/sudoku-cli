pub mod sudoku;
use rayon::ThreadPoolBuilder;
use std::time::Instant;
use sudoku::board::SudokuBoard;
use structopt::StructOpt;
use clap::arg_enum;
use std::fs;
use std::fs::File;
use std::io::Write;

arg_enum! {
    #[derive(Debug)]
    enum Action {
        Count,
        Watch,
        Write,
        Display,
    }
}

#[derive(StructOpt, Debug)]
#[structopt(name = "sudoku-cli")]
struct Opt {
    /// The number of threads to use [default: system CPU count]
    #[structopt(short, long)]
    threads: Option<usize>,
    
    /// The number of milliseconds per frame for {--action=watch} mode
    #[structopt(short = "m", long = "millis", default_value = "50")]
    millis: u64,

    /// The input path for a puzzle (required if --str is unspecified)
    #[structopt(short = "p", long = "path")]
    input_path: Option<String>,
    
    /// The output path for {--action=write} mode
    #[structopt(short = "o", long = "out")]
    output_path: Option<String>,
    
    /// The input string for a puzzle (required if --path is unspecified)
    #[structopt(short = "s", long = "str")]
    input_str: Option<String>,

    /// Toggles compact output
    #[structopt(short = "c", long = "compact")]
    compact: bool,
    
    /// The action to perform { count, watch, display, write }
    #[structopt(short = "a", long = "action")]
    action: Action,
}

fn main() {
    let opt = Opt::from_args();
    if let Some(num_threads) = opt.threads {
        ThreadPoolBuilder::default()
            .num_threads(num_threads)
            .build_global()
            .expect(&format!("Failed to build thread pool with {} threads.", num_threads));
    }
   
    let input = match (&opt.input_path, &opt.input_str) {
        (None, None) => panic!("\n\nsudoku-cli error: Must specify either --path or --str\n\n"),
        (Some(_), Some(_)) => panic!("\n\nsudoku-cli error: Must specify only one of --path or --str\n\n"),
        (Some(path), _) => fs::read_to_string(&path).expect(&format!("\n\nsudoku-cli error: Failed to read from path {}\n\n", &path)),
        (_, Some(string)) => String::from(string),
    };
    
    let mut board = SudokuBoard::from(input);
    
    match opt.action {
        Action::Watch => board.watch_find_solutions(opt.millis),
        Action::Count => {
            println!("{}", board);
            let now = Instant::now();
            let count = board.count_solutions_par();
            let elapsed = now.elapsed();
            println!("  Found: {} solution(s)\n", count);
            println!("  Time:  {} second(s)\n", elapsed.as_secs_f64());
        },
        Action::Display => {
            println!("\n\n  Sudoku Board:\n\n{}", board);
            let now = Instant::now();
            let solutions = if opt.compact { 
               board.find_solutions_compact_par()
            } else {
               board.find_solutions_par()
            };
            let elapsed = now.elapsed();
            let count = board.count_solutions_par();
            println!("  Solutions:\n\n{}\n\n", solutions);
            println!("  Found: {} solution(s)\n", count);
            println!("  Time:  {} second(s)\n", elapsed.as_secs_f64());
        },
        Action::Write => {
            let path = opt.output_path.unwrap();
            let mut file = File::create(&path).unwrap();
            println!("\n\n  Sudoku Board:\n\n{}", board);
            let now = Instant::now();
            let solutions = if opt.compact {
                board.find_solutions_compact_par()
            } else {
                board.find_solutions_par()
            };
            let elapsed = now.elapsed();
            let count = board.count_solutions_par();
            file.write_all(solutions.as_bytes()).unwrap();
            println!("  Found: {} solution(s)\n", count);
            println!("  Time:  {} second(s)\n", elapsed.as_secs_f64());
            println!("  Writing solutions to file: {}\n\n", path);
        },
    }
    
}
