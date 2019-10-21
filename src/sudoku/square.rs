use std::{iter, fmt};

use crate::sudoku::bitwise::{as_bit, values_in_box, values_in_col, values_in_row};
use crate::sudoku::data::SudokuData;
use std::cmp::Ordering;


// 0b_____0000_____0000_____0000_____0000____
//     | options  |   row  |   col  |  value  |

const SHIFT_OPTIONS: usize = 12;
const SHIFT_ROW: usize = 8;
const SHIFT_COL: usize = 4;

#[derive(Clone)]
pub struct SudokuSquare<'a> {
    row_data: &'a SudokuData,
    col_data: &'a SudokuData,
    box_data: &'a SudokuData,
    square_data: usize,
    rcb_cache: usize,
}

impl<'a> SudokuSquare<'a> {
    pub fn new(row: usize, col: usize, row_data: &'a SudokuData, col_data: &'a SudokuData, box_data: &'a SudokuData) -> Self {
        SudokuSquare {
            row_data,
            col_data,
            box_data,
            square_data: ((row << SHIFT_ROW) | (col << SHIFT_COL)) as usize,
            rcb_cache: 0,
        }
    }

    pub fn update_data(&mut self) {
        self.rcb_cache = (values_in_row(self.row_data.data())
            | values_in_col(self.col_data.data())
            | values_in_box(self.box_data.data())) as usize;
        self.square_data &= 0b_0000_1111_1111_1111;
        self.square_data |= ((self.rcb_cache.count_ones()) << SHIFT_OPTIONS) as usize;
    }

    pub fn count_options(&self) -> u32 {
        9 - (self.square_data >> SHIFT_OPTIONS) as u32
    }

    pub fn options(&self) -> impl Iterator<Item = usize> {
        let mut start_value = 1;
        let taken = self.rcb_cache as u64;
        iter::from_fn(move || {
            for value in start_value..=9 {
                if 0 == taken & as_bit(value) {
                    start_value = value + 1;
                    return Some(value);
                }
            }
            return None;
        })
    }

    pub fn value(&self) -> usize {
        self.square_data & 0b_1111
    }

    pub fn col(&self) -> usize {
        (self.square_data >> SHIFT_COL) & 0b_1111
    }

    pub fn unmark(&self) {
        let value = self.value();
        self.row_data.unmark_from_row(value);
        self.col_data.unmark_from_col(value);
        self.box_data.unmark_from_box(value);
    }

    pub fn mark(&self) {
        let value = self.value();
        self.row_data.mark_in_row(value);
        self.col_data.mark_in_col(value);
        self.box_data.mark_in_box(value);
    }

    pub fn fill(&mut self, value: usize) {
        self.clear();
        self.square_data |= value;
        self.row_data.fill_square(value, self.col());
        self.mark();
    }

    pub fn clear(&mut self) {
        self.unmark();
        self.row_data.clear_square(self.col());
        self.square_data &= 0b_1111_1111_1111_0000;
    }
}

impl PartialEq for SudokuSquare<'_> {
    fn eq(&self, other: &Self) -> bool {
        self.square_data == other.square_data
    }
}

impl Eq for SudokuSquare<'_> {}

impl PartialOrd for SudokuSquare<'_> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for SudokuSquare<'_> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.square_data.cmp(&other.square_data)
    }
}

impl fmt::Display for SudokuSquare<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.value())
    }
}

impl fmt::Debug for SudokuSquare<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.value())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn retrieve_options_single_square() {
        let row_data = SudokuData::default();
        let col_data = SudokuData::default();
        let box_data = SudokuData::default();

        let mut square = SudokuSquare::new(0, 0, &row_data, &col_data, &box_data);
        println!("data = {}", square.square_data);
        assert_eq!(9, square.count_options());
        assert_eq!(9, square.options().count());

        square.update_data();
        assert_eq!(9, square.count_options());
        assert_eq!(9, square.options().count());

        row_data.mark_in_row(1);
        row_data.mark_in_row(3);
        col_data.mark_in_col(5);
        col_data.mark_in_col(7);
        box_data.mark_in_box(9);

        square.update_data();
        assert_eq!(4, square.count_options());
        assert_eq!(vec![2, 4, 6, 8], square.options().collect::<Vec<_>>());
    }

    #[test]
    fn retrieve_options_multi_square() {
        let row1 = SudokuData::default();
        let col1 = SudokuData::default();
        let box1 = SudokuData::default();
        let box2 = SudokuData::default();

        let mut square1 = SudokuSquare::new(0, 0,&row1, &col1, &box1);
        let mut square2 = SudokuSquare::new(0, 4, &row1, &col1, &box2);
        assert_eq!(9, square1.count_options());
        assert_eq!(9, square1.options().count());
        assert_eq!(9, square2.count_options());
        assert_eq!(9, square2.options().count());

        row1.mark_in_row(1);
        row1.mark_in_row(3);
        col1.mark_in_col(5);
        col1.mark_in_col(7);
        box1.mark_in_box(9);
        box1.mark_in_box(2);
        box2.mark_in_box(4);
        box2.mark_in_box(6);

        assert_eq!(9, square1.count_options());
        assert_eq!(9, square1.options().count());
        assert_eq!(9, square2.count_options());
        assert_eq!(9, square2.options().count());

        square1.update_data();
        square2.update_data();

        assert_eq!(vec![4, 6, 8], square1.options().collect::<Vec<_>>());
        assert_eq!(vec![2, 8, 9], square2.options().collect::<Vec<_>>());
    }

    #[test]
    fn fill_square() {
        let row = SudokuData::default();
        let col = SudokuData::default();
        let bx  = SudokuData::default();

        let mut square = SudokuSquare::new(0, 0, &row, &col, &bx);
        square.clear();
        square.update_data();
        assert_eq!(vec![1, 2, 3, 4, 5, 6, 7, 8, 9], square.options().collect::<Vec<_>>());
        square.fill(5);
        square.update_data();
        assert_eq!(vec![1, 2, 3, 4, 6, 7, 8, 9], square.options().collect::<Vec<_>>());
        square.fill(3);
        square.update_data();
        assert_eq!(vec![1, 2, 4, 5, 6, 7, 8, 9], square.options().collect::<Vec<_>>());
        square.clear();
        square.update_data();
        assert_eq!(vec![1, 2, 3, 4, 5, 6, 7, 8, 9], square.options().collect::<Vec<_>>());
    }
}
