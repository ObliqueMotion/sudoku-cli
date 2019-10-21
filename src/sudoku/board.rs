use super::data::SudokuData;
use crate::sudoku::square::SudokuSquare;
use std::borrow::Borrow;
use std::{fmt, thread};
use std::iter::repeat;
use std::time::Duration;

#[derive(Clone, Debug, Default)]
pub struct SudokuBoard {
    state: [SudokuData; 9],
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

fn pop_min<'a, 'b: 'a>(v: &'a mut Vec<SudokuSquare<'b>>) -> SudokuSquare<'b> {
    let mut min = &v[0];
    let mut index = 0;
    for i in 1..v.len() {
        if min < &v[i] {
            min = &v[i];
            index = i;
        }
    }
    v.swap_remove(index)
}

impl SudokuBoard {
    pub fn insert(self, value: usize, row: usize, col: usize) -> Self {
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

    pub fn watch_find_all_solutions(&self, millis_per_frame: u64) {
        let squares = &mut self.fillable_squares();
        self.watch_find_solutions(squares, millis_per_frame);
    }

    pub fn find_all_solutions(&self) -> String {
        let squares = &mut self.fillable_squares();
        self.find_solutions(squares)
    }

    pub fn count_all_solutions(&self) -> usize {
        let squares = &mut self.fillable_squares();
        Self::count_solutions(squares)
    }

    fn watch_find_solutions(&self, squares: &mut Vec<SudokuSquare>, millis_per_frame: u64) {
        use ansi_escapes::ClearScreen;
        thread::sleep(Duration::from_millis(millis_per_frame));
        println!("{}\n{}", ClearScreen, self);
        if squares.is_empty() { return; }
        for square in squares.iter_mut() {
            square.update();
        }
        let mut square = pop_min(squares);
        for value in square.options() {
            square.fill(value);
            self.watch_find_all_solutions(millis_per_frame);
        }
        square.clear();
        squares.push(square);
        thread::sleep(Duration::from_millis(millis_per_frame));
        println!("{}\n{}", ClearScreen, self);
    }

    fn count_solutions(squares: &mut Vec<SudokuSquare>) -> usize {
        if squares.is_empty() {
            return 1;
        }
        for square in squares.iter_mut() {
            square.update();
        }
        let mut count = 0;
        let mut square = pop_min(squares);
        for value in square.options() {
            square.fill(value);
            count += Self::count_solutions(squares);
        }
        square.clear();
        squares.push(square);
        count
    }

    fn find_solutions(&self, squares: &mut Vec<SudokuSquare>) -> String {
        if squares.is_empty() {
            return self.to_string();
        }
        for square in squares.iter_mut() {
            square.update();
        }
        let mut solutions = String::new();
        let mut square = pop_min(squares);
        for value in square.options() {
            square.fill(value);
            solutions += &self.find_solutions(squares);
        }
        square.clear();
        squares.push(square);
        solutions
    }

    pub fn fillable_squares(&self) -> Vec<SudokuSquare> {
        let mut squares = Vec::with_capacity(81);
        for row in 0..9 {
            let row_data = &self.state[row];
            for col in 0..9 {
                if 0 == row_data.value_at(col) {
                    squares.push(SudokuSquare::new(
                        row,
                        col,
                        row_data,
                        &self.state[col],
                        &self.state[box_index(row, col)],
                    ));
                }
            }
        }
        squares
    }

    pub fn to_string(&self) -> String {
        let mut string = String::new();
        string.push('\n');
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
        string.push('\n');
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
