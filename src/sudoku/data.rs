use super::bitwise;
use std::cell::Cell;

#[derive(Clone, Debug, Default)]
pub struct SudokuData(Cell<u64>);

impl SudokuData {
    pub fn new() -> Self {
        SudokuData(Cell::new(0))
    }

    pub fn clear(&self) {
        self.0.set(0);
    }

    pub fn clear_zero_slot(&self) {
        self.0.set(bitwise::clear_zero_slot(self.0.get()));
    }

    pub fn clear_one_slot(&self) {
        self.0.set(bitwise::clear_one_slot(self.0.get()))
    }

    pub fn clear_two_slot(&self) {
        self.0.set(bitwise::clear_two_slot(self.0.get()))
    }

    pub fn clear_three_slot(&self) {
        self.0.set(bitwise::clear_three_slot(self.0.get()))
    }

    pub fn clear_four_slot(&self) {
        self.0.set(bitwise::clear_four_slot(self.0.get()))
    }

    pub fn clear_five_slot(&self) {
        self.0.set(bitwise::clear_five_slot(self.0.get()))
    }

    pub fn clear_six_slot(&self) {
        self.0.set(bitwise::clear_six_slot(self.0.get()))
    }

    pub fn clear_seven_slot(&self) {
        self.0.set(bitwise::clear_seven_slot(self.0.get()))
    }

    pub fn clear_eight_slot(&self) {
        self.0.set(bitwise::clear_eight_slot(self.0.get()))
    }

    pub fn set_zero_slot(&self, x: u64) {
        self.0.set(self.0.get() | bitwise::to_zero_slot(x));
    }

    pub fn set_one_slot(&self, x: u64) {
        self.0.set(self.0.get() | bitwise::to_one_slot(x));
    }

    pub fn set_two_slot(&self, x: u64) {
        self.0.set(self.0.get() | bitwise::to_two_slot(x));
    }

    pub fn set_three_slot(&self, x: u64) {
        self.0.set(self.0.get() | bitwise::to_three_slot(x));
    }

    pub fn set_four_slot(&self, x: u64) {
        self.0.set(self.0.get() | bitwise::to_four_slot(x));
    }

    pub fn set_five_slot(&self, x: u64) {
        self.0.set(self.0.get() | bitwise::to_five_slot(x));
    }

    pub fn set_six_slot(&self, x: u64) {
        self.0.set(self.0.get() | bitwise::to_six_slot(x));
    }

    pub fn set_seven_slot(&self, x: u64) {
        self.0.set(self.0.get() | bitwise::to_seven_slot(x));
    }

    pub fn set_eight_slot(&self, x: u64) {
        self.0.set(self.0.get() | bitwise::to_eight_slot(x));
    }

    pub fn add_to_row(&self, x: u64) {
        self.0
            .set(self.0.get() | bitwise::as_row(bitwise::as_bit(x)));
    }

    pub fn add_to_col(&self, x: u64) {
        self.0
            .set(self.0.get() | bitwise::as_col(bitwise::as_bit(x)));
    }

    pub fn add_to_box(&self, x: u64) {
        self.0
            .set(self.0.get() | bitwise::as_box(bitwise::as_bit(x)));
    }

    pub fn remove_from_row(&self, x: u64) {
        self.0
            .set(self.0.get() & bitwise::as_not_row(bitwise::as_not_bit(x)));
    }

    pub fn remove_from_col(&self, x: u64) {
        self.0
            .set(self.0.get() & bitwise::as_not_col(bitwise::as_not_bit(x)));
    }

    pub fn remove_from_box(&self, x: u64) {
        self.0
            .set(self.0.get() & bitwise::as_not_box(bitwise::as_not_bit(x)));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn zero_slot() {
        let data = SudokuData::new();
        data.set_zero_slot(0b0101);
        assert_eq!(
            data.0.get(),
            0b0_000000000_0000000000_000000000_0101_0000_0000_0000_0000_0000_0000_0000_0000,
        );
        data.clear_zero_slot();
        assert_eq!(data.0.get(), 0);
    }

    #[test]
    fn one_slot() {
        let data = SudokuData::new();
        data.set_one_slot(0b0101);
        assert_eq!(
            data.0.get(),
            0b0_000000000_0000000000_000000000_0000_0101_0000_0000_0000_0000_0000_0000_0000,
        );
        data.clear_one_slot();
        assert_eq!(data.0.get(), 0);
    }

    #[test]
    fn two_slot() {
        let data = SudokuData::new();
        data.set_two_slot(0b0101);
        assert_eq!(
            data.0.get(),
            0b0_000000000_0000000000_000000000_0000_0000_0101_0000_0000_0000_0000_0000_0000,
        );
        data.clear_two_slot();
        assert_eq!(data.0.get(), 0);
    }

    #[test]
    fn three_slot() {
        let data = SudokuData::new();
        data.set_three_slot(0b0101);
        assert_eq!(
            data.0.get(),
            0b0_000000000_0000000000_000000000_0000_0000_0000_0101_0000_0000_0000_0000_0000,
        );
        data.clear_three_slot();
        assert_eq!(data.0.get(), 0);
    }

    #[test]
    fn four_slot() {
        let data = SudokuData::new();
        data.set_four_slot(0b0101);
        assert_eq!(
            data.0.get(),
            0b0_000000000_0000000000_000000000_0000_0000_0000_0000_0101_0000_0000_0000_0000,
        );
        data.clear_four_slot();
        assert_eq!(data.0.get(), 0);
    }

    #[test]
    fn five_slot() {
        let data = SudokuData::new();
        data.set_five_slot(0b0101);
        assert_eq!(
            data.0.get(),
            0b0_000000000_0000000000_000000000_0000_0000_0000_0000_0000_0101_0000_0000_0000,
        );
        data.clear_five_slot();
        assert_eq!(data.0.get(), 0);
    }

    #[test]
    fn six_slot() {
        let data = SudokuData::new();
        data.set_six_slot(0b0101);
        assert_eq!(
            data.0.get(),
            0b0_000000000_0000000000_000000000_0000_0000_0000_0000_0000_0000_0101_0000_0000,
        );
        data.clear_six_slot();
        assert_eq!(data.0.get(), 0);
    }

    #[test]
    fn seven_slot() {
        let data = SudokuData::new();
        data.set_seven_slot(0b0101);
        assert_eq!(
            data.0.get(),
            0b0_000000000_0000000000_000000000_0000_0000_0000_0000_0000_0000_0000_0101_0000,
        );
        data.clear_seven_slot();
        assert_eq!(data.0.get(), 0);
    }

    #[test]
    fn eight_slot() {
        let data = SudokuData::new();
        data.set_eight_slot(0b0101);
        assert_eq!(
            data.0.get(),
            0b0_000000000_0000000000_000000000_0000_0000_0000_0000_0000_0000_0000_0000_0101,
        );
        data.clear_eight_slot();
        assert_eq!(data.0.get(), 0);
    }

    #[test]
    fn row_manipulation() {
        let data = SudokuData::new();
        data.add_to_row(1);
        data.add_to_row(3);
        data.add_to_row(5);
        data.add_to_row(7);
        data.add_to_row(9);
        println!("{:b}", data.0.get());
        assert_eq!(
            data.0.get(),
            0b0_101010101_000000000_000000000_0000_0000_0000_0000_0000_0000_0000_0000_0000,
        );
        data.add_to_row(2);
        data.add_to_row(4);
        data.add_to_row(6);
        data.add_to_row(8);
        assert_eq!(
            data.0.get(),
            0b0_111111111_000000000_000000000_0000_0000_0000_0000_0000_0000_0000_0000_0000,
        );
        data.remove_from_row(8);
        data.remove_from_row(6);
        data.remove_from_row(4);
        data.remove_from_row(2);
        assert_eq!(
            data.0.get(),
            0b0_101010101_000000000_000000000_0000_0000_0000_0000_0000_0000_0000_0000_0000,
        );
        data.remove_from_row(9);
        data.remove_from_row(7);
        data.remove_from_row(5);
        data.remove_from_row(3);
        data.remove_from_row(1);
        assert_eq!(data.0.get(), 0);
    }

    #[test]
    fn col_manipulation() {
        let data = SudokuData::new();
        data.add_to_col(1);
        data.add_to_col(3);
        data.add_to_col(5);
        data.add_to_col(7);
        data.add_to_col(9);
        assert_eq!(
            data.0.get(),
            0b0_000000000_101010101_000000000_0000_0000_0000_0000_0000_0000_0000_0000_0000,
        );
        data.add_to_col(2);
        data.add_to_col(4);
        data.add_to_col(6);
        data.add_to_col(8);
        assert_eq!(
            data.0.get(),
            0b0_000000000_111111111_000000000_0000_0000_0000_0000_0000_0000_0000_0000_0000,
        );
        data.remove_from_col(8);
        data.remove_from_col(6);
        data.remove_from_col(4);
        data.remove_from_col(2);
        assert_eq!(
            data.0.get(),
            0b0_000000000_101010101_000000000_0000_0000_0000_0000_0000_0000_0000_0000_0000,
        );
        data.remove_from_col(9);
        data.remove_from_col(7);
        data.remove_from_col(5);
        data.remove_from_col(3);
        data.remove_from_col(1);
        assert_eq!(data.0.get(), 0);
    }

    #[test]
    fn box_manipulation() {
        let data = SudokuData::new();
        data.add_to_box(1);
        data.add_to_box(3);
        data.add_to_box(5);
        data.add_to_box(7);
        data.add_to_box(9);
        assert_eq!(
            data.0.get(),
            0b0_000000000_000000000_101010101_0000_0000_0000_0000_0000_0000_0000_0000_0000,
        );
        data.add_to_box(2);
        data.add_to_box(4);
        data.add_to_box(6);
        data.add_to_box(8);
        assert_eq!(
            data.0.get(),
            0b0_000000000_000000000_111111111_0000_0000_0000_0000_0000_0000_0000_0000_0000,
        );
        data.remove_from_box(8);
        data.remove_from_box(6);
        data.remove_from_box(4);
        data.remove_from_box(2);
        assert_eq!(
            data.0.get(),
            0b0_000000000_000000000_101010101_0000_0000_0000_0000_0000_0000_0000_0000_0000,
        );
        data.remove_from_box(9);
        data.remove_from_box(7);
        data.remove_from_box(5);
        data.remove_from_box(3);
        data.remove_from_box(1);
        assert_eq!(data.0.get(), 0);
    }
}
