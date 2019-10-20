use crate::sudoku::bitwise::{
    as_bit, to_box, to_col, as_not_bit, to_box_inverse, to_col_inverse, to_row_inverse, to_row, clear_cell,
    to_cell, value_in_cell,
};
use std::cell::Cell;
use std::fmt;

#[derive(Clone, Debug, Default)]
pub struct SudokuData(Cell<u64>);

impl SudokuData {
    pub fn clear(&self) {
        self.0.set(0);
    }

    pub fn clear_cell(&self, col: u64) {
        self.0.set(clear_cell(self.0.get(), col));
    }

    pub fn set_cell(&self, value: u64, col: u64) {
        self.0.set(self.0.get() | to_cell(value, col))
    }

    pub fn mark_in_row(&self, value: u64) {
        self.0.set(self.0.get() | to_row(as_bit(value)));
    }

    pub fn mark_in_col(&self, value: u64) {
        self.0.set(self.0.get() | to_col(as_bit(value)));
    }

    pub fn mark_in_box(&self, value: u64) {
        self.0.set(self.0.get() | to_box(as_bit(value)));
    }

    pub fn unmark_from_row(&self, value: u64) {
        self.0.set(self.0.get() & to_row_inverse(as_not_bit(value)));
    }

    pub fn unmark_from_col(&self, value: u64) {
        self.0.set(self.0.get() & to_col_inverse(as_not_bit(value)));
    }

    pub fn unmark_from_box(&self, value: u64) {
        self.0.set(self.0.get() & to_box_inverse(as_not_bit(value)));
    }

    fn format_cell(&self, col: u64) -> &str {
        match value_in_cell(self.0.get(), col) {
            0 => " ",
            1 => "1",
            2 => "2",
            3 => "3",
            4 => "4",
            5 => "5",
            6 => "6",
            7 => "7",
            8 => "8",
            9 => "9",
            _ => panic!("format_cell(): Value not in range (1..=9) found on board."),
        }
    }
}

impl fmt::Display for SudokuData {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "│ {} {} {} │ {} {} {} │ {} {} {} │",
            self.format_cell(0),
            self.format_cell(1),
            self.format_cell(2),
            self.format_cell(3),
            self.format_cell(4),
            self.format_cell(5),
            self.format_cell(6),
            self.format_cell(7),
            self.format_cell(8),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn zero_slot() {
        let data = SudokuData::default();
        data.set_cell(0b0101, 0);
        assert_eq!(
            data.0.get(),
            0b0_000000000_0000000000_000000000_0101_0000_0000_0000_0000_0000_0000_0000_0000,
        );
        data.clear_cell(0);
        assert_eq!(data.0.get(), 0);
    }

    #[test]
    fn one_slot() {
        let data = SudokuData::default();
        data.set_cell(0b0101, 1);
        assert_eq!(
            data.0.get(),
            0b0_000000000_0000000000_000000000_0000_0101_0000_0000_0000_0000_0000_0000_0000,
        );
        data.clear_cell(1);
        assert_eq!(data.0.get(), 0);
    }

    #[test]
    fn two_slot() {
        let data = SudokuData::default();
        data.set_cell(0b0101, 2);
        assert_eq!(
            data.0.get(),
            0b0_000000000_0000000000_000000000_0000_0000_0101_0000_0000_0000_0000_0000_0000,
        );
        data.clear_cell(2);
        assert_eq!(data.0.get(), 0);
    }

    #[test]
    fn three_slot() {
        let data = SudokuData::default();
        data.set_cell(0b0101, 3);
        assert_eq!(
            data.0.get(),
            0b0_000000000_0000000000_000000000_0000_0000_0000_0101_0000_0000_0000_0000_0000,
        );
        data.clear_cell(3);
        assert_eq!(data.0.get(), 0);
    }

    #[test]
    fn four_slot() {
        let data = SudokuData::default();
        data.set_cell(0b0101, 4);
        assert_eq!(
            data.0.get(),
            0b0_000000000_0000000000_000000000_0000_0000_0000_0000_0101_0000_0000_0000_0000,
        );
        data.clear_cell(4);
        assert_eq!(data.0.get(), 0);
    }

    #[test]
    fn five_slot() {
        let data = SudokuData::default();
        data.set_cell(0b0101, 5);
        assert_eq!(
            data.0.get(),
            0b0_000000000_0000000000_000000000_0000_0000_0000_0000_0000_0101_0000_0000_0000,
        );
        data.clear_cell(5);
        assert_eq!(data.0.get(), 0);
    }

    #[test]
    fn six_slot() {
        let data = SudokuData::default();
        data.set_cell(0b0101, 6);
        assert_eq!(
            data.0.get(),
            0b0_000000000_0000000000_000000000_0000_0000_0000_0000_0000_0000_0101_0000_0000,
        );
        data.clear_cell(6);
        assert_eq!(data.0.get(), 0);
    }

    #[test]
    fn seven_slot() {
        let data = SudokuData::default();
        data.set_cell(0b0101, 7);
        assert_eq!(
            data.0.get(),
            0b0_000000000_0000000000_000000000_0000_0000_0000_0000_0000_0000_0000_0101_0000,
        );
        data.clear_cell(7);
        assert_eq!(data.0.get(), 0);
    }

    #[test]
    fn eight_slot() {
        let data = SudokuData::default();
        data.set_cell(0b0101, 8);
        assert_eq!(
            data.0.get(),
            0b0_000000000_0000000000_000000000_0000_0000_0000_0000_0000_0000_0000_0000_0101,
        );
        data.clear_cell(8);
        assert_eq!(data.0.get(), 0);
    }

    #[test]
    fn row_manipulation() {
        let data = SudokuData::default();
        data.mark_in_row(1);
        data.mark_in_row(3);
        data.mark_in_row(5);
        data.mark_in_row(7);
        data.mark_in_row(9);
        println!("{:b}", data.0.get());
        assert_eq!(
            data.0.get(),
            0b0_101010101_000000000_000000000_0000_0000_0000_0000_0000_0000_0000_0000_0000,
        );
        data.mark_in_row(2);
        data.mark_in_row(4);
        data.mark_in_row(6);
        data.mark_in_row(8);
        assert_eq!(
            data.0.get(),
            0b0_111111111_000000000_000000000_0000_0000_0000_0000_0000_0000_0000_0000_0000,
        );
        data.unmark_from_row(8);
        data.unmark_from_row(6);
        data.unmark_from_row(4);
        data.unmark_from_row(2);
        assert_eq!(
            data.0.get(),
            0b0_101010101_000000000_000000000_0000_0000_0000_0000_0000_0000_0000_0000_0000,
        );
        data.unmark_from_row(9);
        data.unmark_from_row(7);
        data.unmark_from_row(5);
        data.unmark_from_row(3);
        data.unmark_from_row(1);
        assert_eq!(data.0.get(), 0);
    }

    #[test]
    fn col_manipulation() {
        let data = SudokuData::default();
        data.mark_in_col(1);
        data.mark_in_col(3);
        data.mark_in_col(5);
        data.mark_in_col(7);
        data.mark_in_col(9);
        assert_eq!(
            data.0.get(),
            0b0_000000000_101010101_000000000_0000_0000_0000_0000_0000_0000_0000_0000_0000,
        );
        data.mark_in_col(2);
        data.mark_in_col(4);
        data.mark_in_col(6);
        data.mark_in_col(8);
        assert_eq!(
            data.0.get(),
            0b0_000000000_111111111_000000000_0000_0000_0000_0000_0000_0000_0000_0000_0000,
        );
        data.unmark_from_col(8);
        data.unmark_from_col(6);
        data.unmark_from_col(4);
        data.unmark_from_col(2);
        assert_eq!(
            data.0.get(),
            0b0_000000000_101010101_000000000_0000_0000_0000_0000_0000_0000_0000_0000_0000,
        );
        data.unmark_from_col(9);
        data.unmark_from_col(7);
        data.unmark_from_col(5);
        data.unmark_from_col(3);
        data.unmark_from_col(1);
        assert_eq!(data.0.get(), 0);
    }

    #[test]
    fn box_manipulation() {
        let data = SudokuData::default();
        data.mark_in_box(1);
        data.mark_in_box(3);
        data.mark_in_box(5);
        data.mark_in_box(7);
        data.mark_in_box(9);
        assert_eq!(
            data.0.get(),
            0b0_000000000_000000000_101010101_0000_0000_0000_0000_0000_0000_0000_0000_0000,
        );
        data.mark_in_box(2);
        data.mark_in_box(4);
        data.mark_in_box(6);
        data.mark_in_box(8);
        assert_eq!(
            data.0.get(),
            0b0_000000000_000000000_111111111_0000_0000_0000_0000_0000_0000_0000_0000_0000,
        );
        data.unmark_from_box(8);
        data.unmark_from_box(6);
        data.unmark_from_box(4);
        data.unmark_from_box(2);
        assert_eq!(
            data.0.get(),
            0b0_000000000_000000000_101010101_0000_0000_0000_0000_0000_0000_0000_0000_0000,
        );
        data.unmark_from_box(9);
        data.unmark_from_box(7);
        data.unmark_from_box(5);
        data.unmark_from_box(3);
        data.unmark_from_box(1);
        assert_eq!(data.0.get(), 0);
    }
}
