use crate::sudoku::bitwise;
use crate::sudoku::bitwise::{
    as_bit, as_bit_inverse, shift_to_box, shift_to_box_inverse, shift_to_col, shift_to_col_inverse,
    shift_to_row, shift_to_row_inverse, shift_to_square, value_in_square, zero_out_square,
};
use std::fmt;

static OUTPUT: [&str; 10] = [" ", "1", "2", "3", "4", "5", "6", "7", "8", "9"];

#[derive(Clone, Debug, Default)]
pub struct SudokuData(u64);

impl SudokuData {
    pub fn data(&self) -> u64 {
        self.0
    }

    pub fn clear(&mut self) {
        self.0 = 0;
    }

    pub fn clear_square(&mut self, col: usize) {
        self.0 = zero_out_square(self.0, col);
    }

    pub fn fill_square(&mut self, value: usize, col: usize) {
        self.clear_square(col);
        self.0 = self.0 | shift_to_square(value, col)
    }

    pub fn mark_in_row(&mut self, value: usize) {
        self.0 = self.0 | shift_to_row(as_bit(value));
    }

    pub fn mark_in_col(&mut self, value: usize) {
        self.0 = self.0 | shift_to_col(as_bit(value));
    }

    pub fn mark_in_box(&mut self, value: usize) {
        self.0 = self.0 | shift_to_box(as_bit(value));
    }

    pub fn unmark_from_row(&mut self, value: usize) {
        self.0 = self.0 & shift_to_row_inverse(as_bit_inverse(value));
    }

    pub fn unmark_from_col(&mut self, value: usize) {
        self.0 = self.0 & shift_to_col_inverse(as_bit_inverse(value));
    }

    pub fn unmark_from_box(&mut self, value: usize) {
        self.0 = self.0 & shift_to_box_inverse(as_bit_inverse(value));
    }

    pub fn values_in_row(&self) -> u64 {
        bitwise::values_in_row(self.0)
    }

    pub fn values_in_col(&self) -> u64 {
        bitwise::values_in_col(self.0)
    }

    pub fn values_in_box(&self) -> u64 {
        bitwise::values_in_box(self.0)
    }

    pub fn value_at(&self, col: usize) -> u64 {
        value_in_square(self.0, col)
    }

    fn format_square(&self, col: usize) -> &str {
        OUTPUT[value_in_square(self.0, col) as usize]
    }

    pub fn to_string(&self) -> String {
        format!(
            "  ║ {} │ {} │ {} ║ {} │ {} │ {} ║ {} │ {} │ {} ║\n",
            self.format_square(0),
            self.format_square(1),
            self.format_square(2),
            self.format_square(3),
            self.format_square(4),
            self.format_square(5),
            self.format_square(6),
            self.format_square(7),
            self.format_square(8),
        )
    }

    pub fn to_string_compact(&self) -> String {
        let mut string = String::with_capacity(9);
        for i in 0..=8 {
            string.push_str(self.format_square(i));
        }
        string
    }
}

impl fmt::Display for SudokuData {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "║ {} │ {} │ {} ║ {} │ {} │ {} ║ {} │ {} │ {} ║",
            self.format_square(0),
            self.format_square(1),
            self.format_square(2),
            self.format_square(3),
            self.format_square(4),
            self.format_square(5),
            self.format_square(6),
            self.format_square(7),
            self.format_square(8),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn zero_slot() {
        let data = SudokuData::default();
        data.fill_square(0b0101, 0);
        assert_eq!(
            data.0,
            0b0_000000000_0000000000_000000000_0101_0000_0000_0000_0000_0000_0000_0000_0000,
        );
        data.clear_square(0);
        assert_eq!(data.0, 0);
    }

    #[test]
    fn one_slot() {
        let data = SudokuData::default();
        data.fill_square(0b0101, 1);
        assert_eq!(
            data.0,
            0b0_000000000_0000000000_000000000_0000_0101_0000_0000_0000_0000_0000_0000_0000,
        );
        data.clear_square(1);
        assert_eq!(data.0, 0);
    }

    #[test]
    fn two_slot() {
        let data = SudokuData::default();
        data.fill_square(0b0101, 2);
        assert_eq!(
            data.0,
            0b0_000000000_0000000000_000000000_0000_0000_0101_0000_0000_0000_0000_0000_0000,
        );
        data.clear_square(2);
        assert_eq!(data.0, 0);
    }

    #[test]
    fn three_slot() {
        let data = SudokuData::default();
        data.fill_square(0b0101, 3);
        assert_eq!(
            data.0,
            0b0_000000000_0000000000_000000000_0000_0000_0000_0101_0000_0000_0000_0000_0000,
        );
        data.clear_square(3);
        assert_eq!(data.0, 0);
    }

    #[test]
    fn four_slot() {
        let data = SudokuData::default();
        data.fill_square(0b0101, 4);
        assert_eq!(
            data.0,
            0b0_000000000_0000000000_000000000_0000_0000_0000_0000_0101_0000_0000_0000_0000,
        );
        data.clear_square(4);
        assert_eq!(data.0, 0);
    }

    #[test]
    fn five_slot() {
        let data = SudokuData::default();
        data.fill_square(0b0101, 5);
        assert_eq!(
            data.0,
            0b0_000000000_0000000000_000000000_0000_0000_0000_0000_0000_0101_0000_0000_0000,
        );
        data.clear_square(5);
        assert_eq!(data.0, 0);
    }

    #[test]
    fn six_slot() {
        let data = SudokuData::default();
        data.fill_square(0b0101, 6);
        assert_eq!(
            data.0,
            0b0_000000000_0000000000_000000000_0000_0000_0000_0000_0000_0000_0101_0000_0000,
        );
        data.clear_square(6);
        assert_eq!(data.0, 0);
    }

    #[test]
    fn seven_slot() {
        let data = SudokuData::default();
        data.fill_square(0b0101, 7);
        assert_eq!(
            data.0,
            0b0_000000000_0000000000_000000000_0000_0000_0000_0000_0000_0000_0000_0101_0000,
        );
        data.clear_square(7);
        assert_eq!(data.0, 0);
    }

    #[test]
    fn eight_slot() {
        let data = SudokuData::default();
        data.fill_square(0b0101, 8);
        assert_eq!(
            data.0,
            0b0_000000000_0000000000_000000000_0000_0000_0000_0000_0000_0000_0000_0000_0101,
        );
        data.clear_square(8);
        assert_eq!(data.0, 0);
    }

    #[test]
    fn mark_rows() {
        let data = SudokuData::default();
        data.mark_in_row(1);
        data.mark_in_row(3);
        data.mark_in_row(5);
        data.mark_in_row(7);
        data.mark_in_row(9);
        println!("{:b}", data.0);
        assert_eq!(
            data.0,
            0b0_101010101_000000000_000000000_0000_0000_0000_0000_0000_0000_0000_0000_0000,
        );
        data.mark_in_row(2);
        data.mark_in_row(4);
        data.mark_in_row(6);
        data.mark_in_row(8);
        assert_eq!(
            data.0,
            0b0_111111111_000000000_000000000_0000_0000_0000_0000_0000_0000_0000_0000_0000,
        );
        data.unmark_from_row(8);
        data.unmark_from_row(6);
        data.unmark_from_row(4);
        data.unmark_from_row(2);
        assert_eq!(
            data.0,
            0b0_101010101_000000000_000000000_0000_0000_0000_0000_0000_0000_0000_0000_0000,
        );
        data.unmark_from_row(9);
        data.unmark_from_row(7);
        data.unmark_from_row(5);
        data.unmark_from_row(3);
        data.unmark_from_row(1);
        assert_eq!(data.0, 0);
    }

    #[test]
    fn mark_cols() {
        let data = SudokuData::default();
        data.mark_in_col(1);
        data.mark_in_col(3);
        data.mark_in_col(5);
        data.mark_in_col(7);
        data.mark_in_col(9);
        assert_eq!(
            data.0,
            0b0_000000000_101010101_000000000_0000_0000_0000_0000_0000_0000_0000_0000_0000,
        );
        data.mark_in_col(2);
        data.mark_in_col(4);
        data.mark_in_col(6);
        data.mark_in_col(8);
        assert_eq!(
            data.0,
            0b0_000000000_111111111_000000000_0000_0000_0000_0000_0000_0000_0000_0000_0000,
        );
        data.unmark_from_col(8);
        data.unmark_from_col(6);
        data.unmark_from_col(4);
        data.unmark_from_col(2);
        assert_eq!(
            data.0,
            0b0_000000000_101010101_000000000_0000_0000_0000_0000_0000_0000_0000_0000_0000,
        );
        data.unmark_from_col(9);
        data.unmark_from_col(7);
        data.unmark_from_col(5);
        data.unmark_from_col(3);
        data.unmark_from_col(1);
        assert_eq!(data.0, 0);
    }

    #[test]
    fn mark_boxes() {
        let data = SudokuData::default();
        data.mark_in_box(1);
        data.mark_in_box(3);
        data.mark_in_box(5);
        data.mark_in_box(7);
        data.mark_in_box(9);
        assert_eq!(
            data.0,
            0b0_000000000_000000000_101010101_0000_0000_0000_0000_0000_0000_0000_0000_0000,
        );
        data.mark_in_box(2);
        data.mark_in_box(4);
        data.mark_in_box(6);
        data.mark_in_box(8);
        assert_eq!(
            data.0,
            0b0_000000000_000000000_111111111_0000_0000_0000_0000_0000_0000_0000_0000_0000,
        );
        data.unmark_from_box(8);
        data.unmark_from_box(6);
        data.unmark_from_box(4);
        data.unmark_from_box(2);
        assert_eq!(
            data.0,
            0b0_000000000_000000000_101010101_0000_0000_0000_0000_0000_0000_0000_0000_0000,
        );
        data.unmark_from_box(9);
        data.unmark_from_box(7);
        data.unmark_from_box(5);
        data.unmark_from_box(3);
        data.unmark_from_box(1);
        assert_eq!(data.0, 0);
    }
}
