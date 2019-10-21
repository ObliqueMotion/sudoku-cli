use super::data::SudokuData;
use crate::sudoku::bitwise::as_bit;
use std::borrow::Borrow;
use std::iter::repeat;
use std::time::Duration;
use std::{fmt, iter, thread};

//                Sudoku Board
//     c0  c1  c2  c3  c4  c5  c6  c7  c8
//    ╔═══════════╦═══════════╦═══════════╗
// r0 ║           ║           ║           ║
// r1 ║   Box 0   ║   Box 1   ║   Box 2   ║
// r2 ║           ║           ║           ║
//    ╠═══════════╬═══════════╬═══════════╣
// r3 ║           ║           ║           ║
// r4 ║   Box 3   ║   Box 4   ║   Box 5   ║
// r5 ║           ║           ║           ║
//    ╠═══════════╬═══════════╬═══════════╣
// r6 ║           ║           ║           ║
// r7 ║   Box 6   ║   Box 7   ║   Box 8   ║
// r8 ║           ║           ║           ║
//    ╚═══════════╩═══════════╩═══════════╝

/// A struct that represents a sudoku board. The board's state consists of 9 SudokuData structs.
/// The board design is compact so that it can be trivially copied into another thread.
/// ──────────────────────────────────────────────────────────────────────────────────────────
/// board[0] contains if a value present in row[0], col[0], box[0], and all the values in row[0]
/// board[1] contains if a value present in row[1], col[1], box[1], and all the values in row[1]
/// board[2] contains if a value present in row[2], col[2], box[2], and all the values in row[2]
/// board[3] contains if a value present in row[3], col[3], box[3], and all the values in row[3]
/// board[4] contains if a value present in row[4], col[4], box[4], and all the values in row[4]
/// board[5] contains if a value present in row[5], col[5], box[5], and all the values in row[5]
/// board[6] contains if a value present in row[6], col[6], box[6], and all the values in row[6]
/// board[7] contains if a value present in row[7], col[7], box[7], and all the values in row[7]
/// board[8] contains if a value present in row[8], col[8], box[8], and all the values in row[8]
#[derive(Clone, Copy, Debug, Default)]
pub struct SudokuBoard {
    board: [SudokuData; 9],
}

/// A sudoku square represents a value that is in a particular (row, col, box).
/// A square's location is fully determined by (row, col) alone,
/// but the box is important information for validation.
#[derive(Clone)]
pub struct SudokuSquare(usize, usize, usize);

impl SudokuSquare {
    /// Creates a new square.
    pub fn new(row: usize, col: usize, bx: usize) -> Self {
        SudokuSquare(row, col, bx)
    }
}

/// Given a row and a column, returns which box that square is in.
fn box_index(row: usize, col: usize) -> usize {
    match row {
        0 | 1 | 2 => match col {
            0 | 1 | 2 => 0,
            3 | 4 | 5 => 1,
            _________ => 2,
        },
        3 | 4 | 5 => match col {
            0 | 1 | 2 => 3,
            3 | 4 | 5 => 4,
            _________ => 5,
        },
        _________ => match col {
            0 | 1 | 2 => 6,
            3 | 4 | 5 => 7,
            _________ => 8,
        },
    }
}

impl SudokuBoard {
    /// Marks a square's value as being present in its row, col, and box.
    fn mark(&mut self, square: &SudokuSquare) {
        let value = self.value_at(square) as usize;
        let &SudokuSquare(row, col, bx) = square;
        self.board[row].mark_in_row(value);
        self.board[col].mark_in_col(value);
        self.board[bx].mark_in_box(value);
    }

    /// Unmarks a square's value from being present in its row, col, and box.
    fn unmark(&mut self, square: &SudokuSquare) {
        let value = self.value_at(square) as usize;
        let &SudokuSquare(row, col, bx) = square;
        self.board[row].unmark_from_row(value);
        self.board[col].unmark_from_col(value);
        self.board[bx].unmark_from_box(value);
    }

    /// Fills a square with a given value, overwriting the previous value.
    fn fill(&mut self, square: &SudokuSquare, value: usize) {
        self.unmark(square);
        let &SudokuSquare(row, col, _) = square;
        self.board[row].fill_square(value, col);
        self.mark(square);
    }

    /// Inserts a new value onto the board at a given (row, col).
    pub fn insert(mut self, value: usize, row: usize, col: usize) -> Self {
        if value == 0 {
            return self;
        }
        assert!((1..=9).contains(&value));
        assert!((0..=8).contains(&row));
        assert!((0..=8).contains(&col));
        let square = &SudokuSquare::new(row, col, box_index(row, col));
        self.fill(square, value);
        self
    }

    /// Clears a square's value from the board.
    fn clear(&mut self, square: &SudokuSquare) {
        self.unmark(square);
        let &SudokuSquare(row, col, _) = square;
        self.board[row].clear_square(col);
    }

    /// Retrieves the value at a given square.
    fn value_at(&self, &SudokuSquare(row, col, _): &SudokuSquare) -> u64 {
        self.board[row].value_at(col)
    }

    /// Returns a set of bits representing the options for a given square.
    /// For example, if 0b101010101 is returned, this means that
    /// { 1, 3, 5, 7, 9 } are already present in the row/col/box and that
    /// { 2, 4, 6, 8 } are available options.
    fn options(&self, &SudokuSquare(row, col, bx): &SudokuSquare) -> u64 {
        self.board[row].values_in_row()
            | self.board[col].values_in_col()
            | self.board[bx].values_in_box()
    }

    /// Returns an iterator over every value that is an available option for this square.
    /// For example, if { 1, 3, 5, 7, 9 } are already present in this square's row/col/box,
    /// then this will return an iterator over { 2 } -> { 4 } -> { 6 } -> { 8 } -> None
    fn options_iter(&self, square: &SudokuSquare) -> impl Iterator<Item = usize> {
        let mut start_value = 1;
        let options = self.options(square);
        iter::from_fn(move || {
            for value in start_value..=9 {
                if 0 == options & as_bit(value) {
                    start_value = value + 1;
                    return Some(value);
                }
            }
            return None;
        })
    }

    /// Returns the count of available options for this square.
    /// For example, if { 1, 3, 5, 7, 9 } are already present in this square's row/col/box,
    /// then this will return 4, because { 2, 4, 6, 8 } are all available options.
    fn count_options(&self, square: &SudokuSquare) -> u32 {
        9 - self.options(square).count_ones()
    }

    /// Count the number of solutions for this board.
    pub fn count_solutions(&mut self) -> usize {
        let squares = &mut self.fillable_squares();
        self.count_all_solutions(squares)
    }

    /// Count the number of solutions for this board.
    fn count_all_solutions(&mut self, squares: &mut Vec<SudokuSquare>) -> usize {
        if squares.is_empty() {
            return 1;
        }
        let mut count = 0;
        let square = self.next_square(squares);
        self.options_iter(&square).for_each(|value| {
            self.fill(&square, value);
            count += self.count_all_solutions(squares);
        });
        self.clear(&square);
        squares.push(square);
        count
    }

    /// watch the board find solutions in the terminal.
    pub fn watch_find_solutions(&mut self, millis_per_frame: u64) {
        let squares = &mut self.fillable_squares();
        self.watch_find_all_solutions(squares, millis_per_frame);
    }

    /// Watch the board find solutions in the terminal.
    fn watch_find_all_solutions(&mut self, squares: &mut Vec<SudokuSquare>, millis_per_frame: u64) {
        use ansi_escapes::ClearScreen;
        thread::sleep(Duration::from_millis(millis_per_frame));
        println!("{}\n{}", ClearScreen, self);
        if squares.is_empty() {
            return;
        }
        let square = self.next_square(squares);
        for value in self.options_iter(&square) {
            self.fill(&square, value);
            self.watch_find_all_solutions(squares, millis_per_frame);
        }
        self.clear(&square);
        squares.push(square);
        thread::sleep(Duration::from_millis(millis_per_frame));
        println!("{}\n{}", ClearScreen, self);
    }

    /// Find all solutions and return each solved board in a String.
    pub fn find_solutions(&mut self) -> String {
        let squares = &mut self.fillable_squares();
        self.find_all_solutions(squares)
    }

    /// Find all solutions and return each solved board in a String.
    fn find_all_solutions(&mut self, squares: &mut Vec<SudokuSquare>) -> String {
        if squares.is_empty() {
            return self.to_string();
        }
        let mut solutions = String::new();
        let square = self.next_square(squares);
        for value in self.options_iter(&square) {
            self.fill(&square, value);
            solutions += &self.find_all_solutions(squares);
        }
        self.clear(&square);
        squares.push(square);
        solutions
    }

    /// Find all solutions and return each solved board as a compact string of 81 contiguous digits (1..=9)
    pub fn find_solutions_compact(&mut self) -> String {
        let squares = &mut self.fillable_squares();
        self.find_all_solutions_compact(squares)
    }

    /// Find all solutions and return each solved board as a compact string of 81 contiguous digits (1..=9)
    fn find_all_solutions_compact(&mut self, squares: &mut Vec<SudokuSquare>) -> String {
        if squares.is_empty() {
            return self.to_string_compact();
        }
        let mut solutions = String::new();
        let square = self.next_square(squares);
        for value in self.options_iter(&square) {
            self.fill(&square, value);
            solutions += &self.find_all_solutions_compact(squares);
        }
        self.clear(&square);
        squares.push(square);
        solutions
    }

    /// Returns the next best square in which to try a value, removing it from the vector.
    /// That is the first encountered square if only 1 option.
    /// Or else any square that is tied for the least number of options.
    fn next_square(&self, v: &mut Vec<SudokuSquare>) -> SudokuSquare {
        let mut index = 0;
        let mut min_options = self.count_options(&v[0]);
        for i in 1..v.len() {
            if min_options == 1 {
                break;
            }
            let curr_options = self.count_options(&v[i]);
            if curr_options < min_options {
                min_options = curr_options;
                index = i;
            }
        }
        v.swap_remove(index)
    }

    /// Returns a vector of all fillable squares on the board.
    pub fn fillable_squares(&self) -> Vec<SudokuSquare> {
        let mut squares = Vec::with_capacity(81);
        for row in 0..9 {
            let row_data = &self.board[row];
            for col in 0..9 {
                if 0 == row_data.value_at(col) {
                    squares.push(SudokuSquare::new(row, col, box_index(row, col)));
                }
            }
        }
        squares
    }

    /// Returns a string representation of the board.
    pub fn to_string(&self) -> String {
        let mut string = String::new();
        string.push_str("\n");
        string.push_str("  ╔═══════════╦═══════════╦═══════════╗\n");
        string.push_str(&self.board[0].to_string());
        string.push_str("  ║───┼───┼───║───┼───┼───║───┼───┼───║\n");
        string.push_str(&self.board[1].to_string());
        string.push_str("  ║───┼───┼───║───┼───┼───║───┼───┼───║\n");
        string.push_str(&self.board[2].to_string());
        string.push_str("  ╠═══════════╬═══════════╬═══════════╣\n");
        string.push_str(&self.board[3].to_string());
        string.push_str("  ║───┼───┼───║───┼───┼───║───┼───┼───║\n");
        string.push_str(&self.board[4].to_string());
        string.push_str("  ║───┼───┼───║───┼───┼───║───┼───┼───║\n");
        string.push_str(&self.board[5].to_string());
        string.push_str("  ╠═══════════╬═══════════╬═══════════╣\n");
        string.push_str(&self.board[6].to_string());
        string.push_str("  ║───┼───┼───║───┼───┼───║───┼───┼───║\n");
        string.push_str(&self.board[7].to_string());
        string.push_str("  ║───┼───┼───║───┼───┼───║───┼───┼───║\n");
        string.push_str(&self.board[8].to_string());
        string.push_str("  ╚═══════════╩═══════════╩═══════════╝\n");
        string.push_str("\n");
        string
    }

    /// Returns a compact string representation of the board: 81 contiguous digits (1..=9)
    pub fn to_string_compact(&self) -> String {
        let mut string = String::with_capacity(83);
        for i in 0..=8 {
            string.push_str(&self.board[i].to_string_compact());
        }
        string.push_str("\n");
        string
    }
}

/// Displays the board in a traditional representation.
impl fmt::Display for SudokuBoard {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "  ╔═══════════╦═══════════╦═══════════╗")?;
        writeln!(f, "  {}", self.board[0])?;
        writeln!(f, "  ║───┼───┼───║───┼───┼───║───┼───┼───║")?;
        writeln!(f, "  {}", self.board[1])?;
        writeln!(f, "  ║───┼───┼───║───┼───┼───║───┼───┼───║")?;
        writeln!(f, "  {}", self.board[2])?;
        writeln!(f, "  ╠═══════════╬═══════════╬═══════════╣")?;
        writeln!(f, "  {}", self.board[3])?;
        writeln!(f, "  ║───┼───┼───║───┼───┼───║───┼───┼───║")?;
        writeln!(f, "  {}", self.board[4])?;
        writeln!(f, "  ║───┼───┼───║───┼───┼───║───┼───┼───║")?;
        writeln!(f, "  {}", self.board[5])?;
        writeln!(f, "  ╠═══════════╬═══════════╬═══════════╣")?;
        writeln!(f, "  {}", self.board[6])?;
        writeln!(f, "  ║───┼───┼───║───┼───┼───║───┼───┼───║")?;
        writeln!(f, "  {}", self.board[7])?;
        writeln!(f, "  ║───┼───┼───║───┼───┼───║───┼───┼───║")?;
        writeln!(f, "  {}", self.board[8])?;
        writeln!(f, "  ╚═══════════╩═══════════╩═══════════╝")
    }
}

/// Creates a board from a string of integers.
/// Ignores whitespace and treats non-digits as a blank square.
/// Example Board String #1:
/// .75.....42139.5.7...8.7...9..2417...4...6...1...8324..3...9.7...5.3.46988.....31.
/// Example Board String #2:
/// -  -  -  -  -  -  -  -  -
/// -  -  -  -  -  3  -  8  5
/// -  -  1  -  2  -  -  -  -
/// -  -  -  5  -  7  -  -  -
/// -  -  4  -  -  -  1  -  -
/// -  9  -  -  -  -  -  -  -
/// 5  -  -  -  -  -  -  7  3
/// -  -  2  -  1  -  -  -  -
/// -  -  -  -  4  -  -  -  9
impl<B: Borrow<str>> From<B> for SudokuBoard {
    fn from(input: B) -> Self {
        input
            .borrow()
            .chars()
            .filter(|c| !c.is_whitespace())
            .filter_map(|c| c.to_digit(10).or(Some(0)))
            .zip(board_indices())
            .fold(SudokuBoard::default(), |board, (value, (row, col))| {
                board.insert(value as usize, row, col)
            })
    }
}

/// Returns an iterator over all 81 of the board's (row, col) indices from (0, 0) to (8, 8)
fn board_indices() -> impl Iterator<Item = (usize, usize)> {
    repeat(0)
        .take(9)
        .chain(repeat(1).take(9))
        .chain(repeat(2).take(9))
        .chain(repeat(3).take(9))
        .chain(repeat(4).take(9))
        .chain(repeat(5).take(9))
        .chain(repeat(6).take(9))
        .chain(repeat(7).take(9))
        .chain(repeat(8).take(9))
        .zip((0..9).cycle())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn fillable_squares() {
        let board = SudokuBoard::from(
            "--------------3-85--1-2-------5-7-----4---1---9-------5------73--2-1--------4---9",
        );
        let squares = board.fillable_squares();
        assert_eq!(81 - 17, squares.len());
    }

    #[test]
    fn count_solutions() {
        let mut board = SudokuBoard::from(
            "--------------3-85--1-2-------5-7-----4---1---9-------5------73--2-1--------4---9",
        );
        assert_eq!(1, board.count_solutions());
        let mut board = SudokuBoard::from(
            ".75.....42139.5.7...8.....9..241....4...........8.24..3...9.7...5.3..6988.....31.",
        );
        assert_eq!(35, board.count_solutions());
        let mut board = SudokuBoard::from(
            "
            -  -  -  -  -  -  -  -  -
            -  -  -  -  -  3  -  8  5
            -  -  1  -  2  -  -  -  -
            -  -  -  5  -  7  -  -  -
            -  -  4  -  -  -  1  -  -
            -  9  -  -  -  -  -  -  -
            5  -  -  -  -  -  -  7  3
            -  -  2  -  1  -  -  -  -
            -  -  -  -  4  -  -  -  9
        ",
        );
        assert_eq!(1, board.count_solutions());
    }

    #[test]
    fn find_solutions() {
        let mut board = SudokuBoard::from(
            ".75.....4.1...5.7...8.7...9..2417...4.......1...8.24..3...9.7...5.3.4..88.....31.",
        );
        let expected_solutions = vec![
            "675983124913245876248671539562417983487539261139862457326198745751324698894756312",
            "675983124913245876248671539582417693437569281169832457326198745751324968894756312",
            "675983124913245876248671539582417693467539281139862457326198745751324968894756312",
            "675983124913245876248671539582417963437569281169832457326198745751324698894756312",
            "675983124913245876248671539582417963467539281139862457326198745751324698894756312",
            "975683124213945876648271539562417983487539261139862457326198745751324698894756312",
            "975683124213945876648271539582417693437569281169832457326198745751324968894756312",
            "975683124213945876648271539582417693467539281139862457326198745751324968894756312",
            "975683124213945876648271539582417963437569281169832457326198745751324698894756312",
            "975683124213945876648271539582417963467539281139862457326198745751324698894756312",
        ];
        let solutions = board.find_solutions_compact();
        for solution in &expected_solutions {
            assert!(solutions.contains(solution));
        }
    }
}
