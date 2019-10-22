pub mod sudoku;
use rayon::ThreadPoolBuilder;
use std::time::Instant;
use sudoku::board::SudokuBoard;
use structopt::StructOpt;
use clap::arg_enum;
use std::fs;

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
    #[structopt(short, long, default_value = "0")]
    threads: usize,
    #[structopt(short = "m", long = "millis-per-frame", default_value = "50")]
    millis: u64,

    #[structopt(short = "p", long = "path")]
    input_path: Option<String>,
    
    #[structopt(short = "s", long = "str")]
    input_str: Option<String>,

    #[structopt(short = "c", long = "compact")]
    compact: bool,
    
    #[structopt(short = "a", long = "action")]
    action: Action,
}

fn main() {
    let opt = Opt::from_args();
    if opt.threads > 0 {
        ThreadPoolBuilder::default()
            .num_threads(opt.threads)
            .build_global()
            .expect(&format!("Failed to build thread pool with {} threads.", opt.threads));
    }
   
    let input = match (&opt.input_path, &opt.input_str) {
        (None, None) => panic!("\n\nsudoku-cli error: Must specify either --input-path or --input-str\n\n"),
        (Some(_), Some(_)) => panic!("\n\nsudoku-cli error: Must specify only one of --input-path or --input-str\n\n"),
        (Some(path), _) => fs::read_to_string(&path).expect(&format!("\n\nsudoku-cli error: Failed to read from path {}\n\n", &path)),
        (_, Some(string)) => String::from(string),
    };
    
    let mut board = SudokuBoard::from(input);
    
    match opt.action {
        Action::Watch => board.watch_find_solutions(opt.millis),
        Action::Count => {
            println!("\n\n  Sudoku Board:\n\n{}", board);
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
        _ => panic!(),
    }
    
}
