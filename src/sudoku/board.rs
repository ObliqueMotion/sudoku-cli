use super::data::SudokuData;
use crate::sudoku::bitwise::{as_bit, values_in_box, values_in_col, values_in_row};
use std::borrow::Borrow;
use std::iter::repeat;
use std::time::Duration;
use std::{fmt, iter, thread};

#[derive(Clone, Debug, Default)]
pub struct SudokuBoard {
    state: [SudokuData; 9],
}

#[derive(Clone)]
pub struct SudokuSquare(usize, usize, usize);

impl SudokuSquare {
    pub fn new(row: usize, col: usize, bx: usize) -> Self {
        SudokuSquare(row, col, bx)
    }
}

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
    pub fn insert(mut self, value: usize, row: usize, col: usize) -> Self {
        if value == 0 {
            return self;
        }
        assert!((1..=9).contains(&value));
        assert!((0..=8).contains(&row));
        assert!((0..=8).contains(&col));
        let bx = box_index(row, col);
        self.state[row].fill_square(value, col);
        self.state[row].mark_in_row(value);
        self.state[col].mark_in_col(value);
        self.state[bx].mark_in_box(value);
        self
    }

    fn mark(&mut self, square: &SudokuSquare) {
        let value = self.value_at(square) as usize;
        let &SudokuSquare(row, col, bx) = square;
        self.state[row].mark_in_row(value);
        self.state[col].mark_in_col(value);
        self.state[bx].mark_in_box(value);
    }

    fn unmark(&mut self, square: &SudokuSquare) {
        let value = self.value_at(square) as usize;
        let &SudokuSquare(row, col, bx) = square;
        self.state[row].unmark_from_row(value);
        self.state[col].unmark_from_col(value);
        self.state[bx].unmark_from_box(value);
    }

    fn fill(&mut self, square: &SudokuSquare, value: usize) {
        self.unmark(square);
        let &SudokuSquare(row, col, _) = square;
        self.state[row].fill_square(value, col);
        self.mark(square);
    }

    fn clear(&mut self, square: &SudokuSquare) {
        self.unmark(square);
        let &SudokuSquare(row, col, _) = square;
        self.state[row].clear_square(col);
    }

    fn value_at(&self, &SudokuSquare(row, col, _): &SudokuSquare) -> u64 {
        self.state[row].value_at(col)
    }

    fn options(&self, &SudokuSquare(row, col, bx): &SudokuSquare) -> u64 {
        values_in_row(self.state[row].data())
            | values_in_col(self.state[col].data())
            | values_in_box(self.state[bx].data())
    }

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

    fn count_options(&self, square: &SudokuSquare) -> u32 {
        9 - self.options(square).count_ones()
    }

    pub fn watch_find_all_solutions(&mut self, millis_per_frame: u64) {
        let squares = &mut self.fillable_squares();
        self.watch_find_solutions(squares, millis_per_frame);
    }

    pub fn find_all_solutions(&mut self) -> String {
        let squares = &mut self.fillable_squares();
        self.find_solutions(squares)
    }

    pub fn find_all_solutions_compact(&mut self) -> String {
        let squares = &mut self.fillable_squares();
        self.find_solutions_compact(squares)
    }

    pub fn count_all_solutions(&mut self) -> usize {
        let squares = &mut self.fillable_squares();
        self.count_solutions(squares)
    }

    fn watch_find_solutions(&mut self, squares: &mut Vec<SudokuSquare>, millis_per_frame: u64) {
        use ansi_escapes::ClearScreen;
        thread::sleep(Duration::from_millis(millis_per_frame));
        println!("{}\n{}", ClearScreen, self);
        if squares.is_empty() {
            return;
        }
        let square = self.next_square(squares);
        for value in self.options_iter(&square) {
            self.fill(&square, value);
            self.watch_find_all_solutions(millis_per_frame);
        }
        self.clear(&square);
        squares.push(square);
        thread::sleep(Duration::from_millis(millis_per_frame));
        println!("{}\n{}", ClearScreen, self);
    }

    fn count_solutions(&mut self, squares: &mut Vec<SudokuSquare>) -> usize {
        if squares.is_empty() {
            return 1;
        }
        let mut count = 0;
        let square = self.next_square(squares);
        self.options_iter(&square).for_each(|value| {
            self.fill(&square, value);
            count += self.count_solutions(squares);
        });
        self.clear(&square);
        squares.push(square);
        count
    }

    fn find_solutions(&mut self, squares: &mut Vec<SudokuSquare>) -> String {
        if squares.is_empty() {
            return self.to_string();
        }
        let mut solutions = String::new();
        let square = self.next_square(squares);
        for value in self.options_iter(&square) {
            self.fill(&square, value);
            solutions += &self.find_solutions(squares);
        }
        self.clear(&square);
        squares.push(square);
        solutions
    }

    fn find_solutions_compact(&mut self, squares: &mut Vec<SudokuSquare>) -> String {
        if squares.is_empty() {
            return self.to_string_compact();
        }
        let mut solutions = String::new();
        let square = self.next_square(squares);
        for value in self.options_iter(&square) {
            self.fill(&square, value);
            solutions += &self.find_solutions_compact(squares);
        }
        self.clear(&square);
        squares.push(square);
        solutions
    }

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

    pub fn fillable_squares(&self) -> Vec<SudokuSquare> {
        let mut squares = Vec::with_capacity(81);
        for row in 0..9 {
            let row_data = &self.state[row];
            for col in 0..9 {
                if 0 == row_data.value_at(col) {
                    squares.push(SudokuSquare::new(row, col, box_index(row, col)));
                }
            }
        }
        squares
    }

    pub fn to_string(&self) -> String {
        let mut string = String::new();
        string.push_str("\n");
        string.push_str("  ╔═══════════╦═══════════╦═══════════╗\n");
        string.push_str(&self.state[0].to_string());
        string.push_str("  ║───┼───┼───║───┼───┼───║───┼───┼───║\n");
        string.push_str(&self.state[1].to_string());
        string.push_str("  ║───┼───┼───║───┼───┼───║───┼───┼───║\n");
        string.push_str(&self.state[2].to_string());
        string.push_str("  ╠═══════════╬═══════════╬═══════════╣\n");
        string.push_str(&self.state[3].to_string());
        string.push_str("  ║───┼───┼───║───┼───┼───║───┼───┼───║\n");
        string.push_str(&self.state[4].to_string());
        string.push_str("  ║───┼───┼───║───┼───┼───║───┼───┼───║\n");
        string.push_str(&self.state[5].to_string());
        string.push_str("  ╠═══════════╬═══════════╬═══════════╣\n");
        string.push_str(&self.state[6].to_string());
        string.push_str("  ║───┼───┼───║───┼───┼───║───┼───┼───║\n");
        string.push_str(&self.state[7].to_string());
        string.push_str("  ║───┼───┼───║───┼───┼───║───┼───┼───║\n");
        string.push_str(&self.state[8].to_string());
        string.push_str("  ╚═══════════╩═══════════╩═══════════╝\n");
        string.push_str("\n");
        string
    }

    pub fn to_string_compact(&self) -> String {
        let mut string = String::with_capacity(82);
        for i in 0..=8 {
            string.push_str(&self.state[i].to_string_compact());
        }
        string.push_str("\n");
        string
    }
}

impl fmt::Display for SudokuBoard {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "  ╔═══════════╦═══════════╦═══════════╗")?;
        writeln!(f, "  {}", self.state[0])?;
        writeln!(f, "  ║───┼───┼───║───┼───┼───║───┼───┼───║")?;
        writeln!(f, "  {}", self.state[1])?;
        writeln!(f, "  ║───┼───┼───║───┼───┼───║───┼───┼───║")?;
        writeln!(f, "  {}", self.state[2])?;
        writeln!(f, "  ╠═══════════╬═══════════╬═══════════╣")?;
        writeln!(f, "  {}", self.state[3])?;
        writeln!(f, "  ║───┼───┼───║───┼───┼───║───┼───┼───║")?;
        writeln!(f, "  {}", self.state[4])?;
        writeln!(f, "  ║───┼───┼───║───┼───┼───║───┼───┼───║")?;
        writeln!(f, "  {}", self.state[5])?;
        writeln!(f, "  ╠═══════════╬═══════════╬═══════════╣")?;
        writeln!(f, "  {}", self.state[6])?;
        writeln!(f, "  ║───┼───┼───║───┼───┼───║───┼───┼───║")?;
        writeln!(f, "  {}", self.state[7])?;
        writeln!(f, "  ║───┼───┼───║───┼───┼───║───┼───┼───║")?;
        writeln!(f, "  {}", self.state[8])?;
        writeln!(f, "  ╚═══════════╩═══════════╩═══════════╝")
    }
}

impl<B: Borrow<str>> From<B> for SudokuBoard {
    fn from(input: B) -> Self {
        input
            .borrow()
            .trim()
            .chars()
            .filter(|c| !c.is_whitespace())
            .filter_map(|c| c.to_digit(10).or(Some(0)))
            .zip(board_indices())
            .fold(SudokuBoard::default(), |board, (value, (row, col))| {
                board.insert(value as usize, row, col)
            })
    }
}

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
}
