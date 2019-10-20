use std::iter;

use crate::sudoku::bitwise::{as_bit, values_in_box, values_in_col, values_in_row};
use crate::sudoku::data::SudokuData;

#[derive(Debug)]
pub struct SudokuSquare<'a> {
    value: usize,
    row: &'a SudokuData,
    col: &'a SudokuData,
    bx: &'a SudokuData,
    rcb_cache: u64,
}

impl<'a> SudokuSquare<'a> {
    pub fn new(row: &'a SudokuData, col: &'a SudokuData, bx: &'a SudokuData) -> Self {
        SudokuSquare { value: 0, row, col, bx, rcb_cache: 0 }
    }

    pub fn update_cache(&mut self) {
        self.rcb_cache = values_in_row(self.row.data())
            | values_in_col(self.col.data())
            | values_in_box(self.bx.data());
    }

    pub fn count_options(&self) -> u32 {
        9 - self.rcb_cache.count_ones()
    }

    pub fn options(&self) -> impl Iterator<Item=usize> {
        let mut start_value = 1;
        let taken = self.rcb_cache;
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

    pub fn unmark(&self) {
        self.row.unmark_from_row(self.value);
        self.col.unmark_from_col(self.value);
        self.bx.unmark_from_box(self.value);
    }

    pub fn mark(&self) {
        self.row.mark_in_row(self.value);
        self.col.mark_in_col(self.value);
        self.bx.mark_in_box(self.value);
    }

    pub fn fill(&mut self, value: usize) {
        self.unmark();
        self.value = value;
        self.mark();
    }

    pub fn clear(&mut self) {
        self.unmark();
        self.value = 0;
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

        let mut square = SudokuSquare::new(&row_data, &col_data, &box_data);
        assert_eq!(9, square.count_options());
        assert_eq!(9, square.options().count());

        square.update_cache();
        assert_eq!(9, square.count_options());
        assert_eq!(9, square.options().count());

        row_data.mark_in_row(1);
        row_data.mark_in_row(3);
        col_data.mark_in_col(5);
        col_data.mark_in_col(7);
        box_data.mark_in_box(9);

        square.update_cache();
        assert_eq!(4, square.count_options());
        assert_eq!(vec![2, 4, 6, 8], square.options().collect::<Vec<_>>());
    }

    #[test]
    fn retrieve_options_multi_square() {
        let row1 = SudokuData::default();
        let col1 = SudokuData::default();
        let box1 = SudokuData::default();
        let box2 = SudokuData::default();

        let mut square1 = SudokuSquare::new(&row1, &col1, &box1);
        let mut square2 = SudokuSquare::new(&row1, &col1, &box2);
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

        square1.update_cache();
        square2.update_cache();

        assert_eq!(vec![4, 6, 8], square1.options().collect::<Vec<_>>());
        assert_eq!(vec![2, 8, 9], square2.options().collect::<Vec<_>>());
    }
}