pub mod sudoku;
use sudoku::board::SudokuBoard;
use sudoku::data::SudokuData;

static HARD_PUZZLE: &str = include_str!("../puzzles/hard2");

fn main() {
    let board = SudokuBoard::default()
        .insert(5, 0, 3)
        .insert(7, 3, 1)
        .insert(2, 5, 7);
    println!("\n{}", board);

    let board2 = SudokuBoard::from(HARD_PUZZLE);
    println!("\n{}", board2);

    let data1 = SudokuData::default();
    let data2 = SudokuData::default();
    let data3 = SudokuData::default();

    let vector = vec![&data1, &data2, &data3];

    vector.iter().for_each(|d| d.fill_square(5, 2));

    for d in &vector {
        dbg!(d);
    }

    dbg!(&data1);
    dbg!(&data2);
    dbg!(&data3);

    for d in &vector {
        dbg!(d);
    }

}
