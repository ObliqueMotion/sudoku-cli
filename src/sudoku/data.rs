use super::bitwise;
use std::cell::Cell;
use std::fmt;

#[derive(Clone, Debug, Default)]
pub struct SudokuData(Cell<u64>);

impl SudokuData {
    pub fn clear(&self) {
        self.0.set(0);
    }

    pub fn clear_cell(&self, col: u64) {
        self.0.set(bitwise::clear_cell(self.0.get(), col));
    }

    pub fn set_cell(&self, x: u64, col: u64) {
        self.0.set(self.0.get() | bitwise::to_col(x, col))
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

    fn format_cell(&self, col: u64) -> &str {
       let value = bitwise::value_at(self.0.get(), col);
        match value {
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
            _ => panic!("format_cell(): Value not in range (1..=9) found on board.")
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
        let data = SudokuData::default();
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
        let data = SudokuData::default();
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
