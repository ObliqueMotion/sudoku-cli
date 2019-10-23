//! A collection of bitwise functions specific to manipulating and reading data on a sudoku board.
//! ```text
//! Semantic mapping onto 64-bit integer:
//! ───────────────────────────────────────────────────────────────────────────────────────────────────────
//! 0b0_______000000000_000000000_000000000___0000___0000___0000___0000___0000___0000___0000___0000___0000
//! ───────────────────────────────────────────────────────────────────────────────────────────────────────
//! | unused |   row   |   col   |   box    | zero | one  | two  | three| four | five |  six | seven| eight
//! ───────────────────────────────────────────────────────────────────────────────────────────────────────
//!          |─── value is present if set ──|──── values represent the values in each square of a row ────|
//! ───────────────────────────────────────────────────────────────────────────────────────────────────────
//!          | 110000011 means {1, 2, 8, 9} | 
//! ───────────────────────────────────────────────────────────────────────────────────────────────────────
//! ```

const SHIFT_ROW: u64 = 54;
const SHIFT_COL: u64 = 45;
const SHIFT_BOX: u64 = 36;
const SHIFT_SQUARE: [u64; 9] = [32, 28, 24, 20, 16, 12, 8, 4, 0];

const FOUR_SET_BITS: u64 = 0b1111;
const NINE_SET_BITS: u64 = 0b111111111;

/// Clear a value by bitwise & with one of these.
const CLEAR: [u64; 9] = [
    0b1_111111111_111111111_111111111_0000_1111_1111_1111_1111_1111_1111_1111_1111,
    0b1_111111111_111111111_111111111_1111_0000_1111_1111_1111_1111_1111_1111_1111,
    0b1_111111111_111111111_111111111_1111_1111_0000_1111_1111_1111_1111_1111_1111,
    0b1_111111111_111111111_111111111_1111_1111_1111_0000_1111_1111_1111_1111_1111,
    0b1_111111111_111111111_111111111_1111_1111_1111_1111_0000_1111_1111_1111_1111,
    0b1_111111111_111111111_111111111_1111_1111_1111_1111_1111_0000_1111_1111_1111,
    0b1_111111111_111111111_111111111_1111_1111_1111_1111_1111_1111_0000_1111_1111,
    0b1_111111111_111111111_111111111_1111_1111_1111_1111_1111_1111_1111_0000_1111,
    0b1_111111111_111111111_111111111_1111_1111_1111_1111_1111_1111_1111_1111_0000,
];

/// A bit begin set represents a value being present in a row, column, or box.
const BITS: [u64; 10] = [
    0b000000000, // Padding
    0b100000000, // One
    0b010000000, // Two
    0b001000000, // Three
    0b000100000, // Four
    0b000010000, // Five
    0b000001000, // Six
    0b000000100, // Seven
    0b000000010, // Eight
    0b000000001, // Nine
];

/// Returns a set bit to mark a value as being present in a row, column, or box.  
/// ```text
/// Arg value: 7  
/// Return: 0b000000100  
/// ```
pub const fn as_bit(value: usize) -> u64 {
    BITS[value]
}

/// Returns all bits set except one to unmark a value as being present in a row, column, or box.  
/// ```text
/// Arg value: 7  
/// Return: 0b1111111111111111111111111111111111111111111111111111111_111111011  
/// ```
pub const fn as_bit_inverse(value: usize) -> u64 {
    !BITS[value]
}

/// Shifts a set of bits to the row location.  
/// ```text
/// Arg bits: 0b101010101  
/// Return: 0b0_101010101_000000000_000000000_0000_0000_0000_0000_0000_0000_0000_0000_0000  
/// ```
pub const fn shift_to_row(bits: u64) -> u64 {
    bits << SHIFT_ROW
}

/// Shifts a set of bits to the col location.  
/// ```text
/// Arg bits: 0b101010101  
/// Return: 0b0_000000000_101010101_000000000_0000_0000_0000_0000_0000_0000_0000_0000_0000  
/// ```
pub const fn shift_to_col(bits: u64) -> u64 {
    bits << SHIFT_COL
}

/// Shifts a set of bits to the box location.  
/// ```text
/// Arg bits: 0b101010101  
/// Return: 0b0_000000000_000000000_101010101_0000_0000_0000_0000_0000_0000_0000_0000_0000  
/// ```
pub const fn shift_to_box(bits: u64) -> u64 {
    bits << SHIFT_BOX
}

/// Shifts a set of bits to the row location, filling with ones from the right instead of zeros.  
/// ```text
/// Arg bits: 0b101010101  
/// Return: 0b0_101010101_111111111_111111111_1111_1111_1111_1111_1111_1111_1111_1111_1111  
/// ```
pub const fn shift_to_row_inverse(bits: u64) -> u64 {
    bits << SHIFT_ROW | ((1 << SHIFT_ROW) - 1)
}

/// Shifts a set of bits to the col location, filling with ones from the right instead of zeros.  
/// ```text
/// Arg bits: 0b101010101  
/// Return: 0b0_000000000_101010101_111111111_1111_1111_1111_1111_1111_1111_1111_1111_1111  
/// ```
pub const fn shift_to_col_inverse(bits: u64) -> u64 {
    bits << SHIFT_COL | ((1 << SHIFT_COL) - 1)
}

/// Shifts a set of bits to the box location, filling with ones from the right instead of zeros.  
/// ```text
/// Arg bits: 0b101010101  
/// Return: 0b0_000000000_000000000_101010101_1111_1111_1111_1111_1111_1111_1111_1111_1111  
/// ```
pub const fn shift_to_box_inverse(bits: u64) -> u64 {
    bits << SHIFT_BOX | ((1 << SHIFT_BOX) - 1)
}

/// Shifts a set of bits to a square's location.  
/// ```text
/// Arg value: 0b1001  
/// Arg col: 4  
/// Return: 0b0_000000000_000000000_000000000_0000_0000_0000_1001_0000_0000_0000_0000_0000  
/// ```
pub const fn shift_to_square(value: usize, col: usize) -> u64 {
    (value as u64) << SHIFT_SQUARE[col]
}

/// Returns the set of bits that represent the values present in a given row.  
/// ```text
/// Arg data: 0b0_110010011_000000000_000000000_0000_0000_0000_0000_0000_0000_0000_0000_0000  
/// Return: 0b110010011  
/// ```
pub const fn values_in_row(data: u64) -> u64 {
    (data >> SHIFT_ROW) & NINE_SET_BITS
}

/// Returns the set of bits that represent the values present in a given column.  
/// ```text
/// Arg data: 0b0_000000000_110010011_000000000_0000_0000_0000_0000_0000_0000_0000_0000_0000  
/// Return: 0b110010011  
/// ```
pub const fn values_in_col(data: u64) -> u64 {
    (data >> SHIFT_COL) & NINE_SET_BITS
}

/// Returns the set of bits that represent the values present in a given box.  
/// ```text
/// Arg data: 0b0_000000000_000000000_110010011_0000_0000_0000_0000_0000_0000_0000_0000_0000  
/// Return: 0b110010011  
/// ```
pub const fn values_in_box(data: u64) -> u64 {
    (data >> SHIFT_BOX) & NINE_SET_BITS
}

/// Returns the value in a square's location.  
/// ```text
/// Arg data: 0b0001_0010_0011_0100_0101_0110_0111_1000_1001  
/// Arg col: 7  
/// Return: 0b0111  
/// ```
pub const fn value_in_square(data: u64, col: usize) -> u64 {
    (data >> SHIFT_SQUARE[col]) & FOUR_SET_BITS
}

/// Zeros out the value in a square's location.  
/// ```text
/// Arg data: 0b0001_0010_0011_0100_0101_0110_0111_1000_1001  
/// Arg col: 7  
/// Return: 0b0001_0010_0011_0100_0101_0110_0000_1000_1001  
/// ```
pub const fn zero_out_square(data: u64, col: usize) -> u64 {
    data & CLEAR[col]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn shifts() {
        const FOUR_SET_BITS: usize = 0b1111;
        const NINE_SET_BITS: u64 = 0b111111111;
        assert_eq!(
            shift_to_row(NINE_SET_BITS),
            0b0_111111111_000000000_000000000_0000_0000_0000_0000_0000_0000_0000_0000_0000,
        );
        assert_eq!(
            shift_to_col(NINE_SET_BITS),
            0b0_000000000_111111111_000000000_0000_0000_0000_0000_0000_0000_0000_0000_0000,
        );
        assert_eq!(
            shift_to_box(NINE_SET_BITS),
            0b0_000000000_000000000_111111111_0000_0000_0000_0000_0000_0000_0000_0000_0000,
        );
        assert_eq!(
            shift_to_square(FOUR_SET_BITS, 0),
            0b0_000000000_000000000_000000000_1111_0000_0000_0000_0000_0000_0000_0000_0000,
        );
        assert_eq!(
            shift_to_square(FOUR_SET_BITS, 1),
            0b0_000000000_000000000_000000000_0000_1111_0000_0000_0000_0000_0000_0000_0000,
        );
        assert_eq!(
            shift_to_square(FOUR_SET_BITS, 2),
            0b0_000000000_000000000_000000000_0000_0000_1111_0000_0000_0000_0000_0000_0000,
        );
        assert_eq!(
            shift_to_square(FOUR_SET_BITS, 3),
            0b0_000000000_000000000_000000000_0000_0000_0000_1111_0000_0000_0000_0000_0000,
        );
        assert_eq!(
            shift_to_square(FOUR_SET_BITS, 4),
            0b0_000000000_000000000_000000000_0000_0000_0000_0000_1111_0000_0000_0000_0000,
        );
        assert_eq!(
            shift_to_square(FOUR_SET_BITS, 5),
            0b0_000000000_000000000_000000000_0000_0000_0000_0000_0000_1111_0000_0000_0000,
        );
        assert_eq!(
            shift_to_square(FOUR_SET_BITS, 6),
            0b0_000000000_000000000_000000000_0000_0000_0000_0000_0000_0000_1111_0000_0000,
        );
        assert_eq!(
            shift_to_square(FOUR_SET_BITS, 7),
            0b0_000000000_000000000_000000000_0000_0000_0000_0000_0000_0000_0000_1111_0000,
        );
        assert_eq!(
            shift_to_square(FOUR_SET_BITS, 8),
            0b0_000000000_000000000_000000000_0000_0000_0000_0000_0000_0000_0000_0000_1111,
        );
    }

    #[test]
    fn inverse_shifts() {
        const NINE_UNSET_BITS: u64 =
            0b1111111111111111111111111111111111111111111111111111111_000000000;
        assert_eq!(
            shift_to_row_inverse(NINE_UNSET_BITS),
            0b1_000000000_111111111_111111111_1111_1111_1111_1111_1111_1111_1111_1111_1111,
        );
        assert_eq!(
            shift_to_col_inverse(NINE_UNSET_BITS),
            0b1_111111111_000000000_111111111_1111_1111_1111_1111_1111_1111_1111_1111_1111,
        );
        assert_eq!(
            shift_to_box_inverse(NINE_UNSET_BITS),
            0b1_111111111_111111111_000000000_1111_1111_1111_1111_1111_1111_1111_1111_1111,
        );
    }

    #[test]
    fn clears() {
        const ALL_SET_BITS: u64 =
            0b1_111111111_111111111_111111111_1111_1111_1111_1111_1111_1111_1111_1111_1111;
        assert_eq!(
            zero_out_square(ALL_SET_BITS, 0),
            0b1_111111111_111111111_111111111_0000_1111_1111_1111_1111_1111_1111_1111_1111,
        );
        assert_eq!(
            zero_out_square(ALL_SET_BITS, 1),
            0b1_111111111_111111111_111111111_1111_0000_1111_1111_1111_1111_1111_1111_1111,
        );
        assert_eq!(
            zero_out_square(ALL_SET_BITS, 2),
            0b1_111111111_111111111_111111111_1111_1111_0000_1111_1111_1111_1111_1111_1111,
        );
        assert_eq!(
            zero_out_square(ALL_SET_BITS, 3),
            0b1_111111111_111111111_111111111_1111_1111_1111_0000_1111_1111_1111_1111_1111,
        );
        assert_eq!(
            zero_out_square(ALL_SET_BITS, 4),
            0b1_111111111_111111111_111111111_1111_1111_1111_1111_0000_1111_1111_1111_1111,
        );
        assert_eq!(
            zero_out_square(ALL_SET_BITS, 5),
            0b1_111111111_111111111_111111111_1111_1111_1111_1111_1111_0000_1111_1111_1111,
        );
        assert_eq!(
            zero_out_square(ALL_SET_BITS, 6),
            0b1_111111111_111111111_111111111_1111_1111_1111_1111_1111_1111_0000_1111_1111,
        );
        assert_eq!(
            zero_out_square(ALL_SET_BITS, 7),
            0b1_111111111_111111111_111111111_1111_1111_1111_1111_1111_1111_1111_0000_1111,
        );
        assert_eq!(
            zero_out_square(ALL_SET_BITS, 8),
            0b1_111111111_111111111_111111111_1111_1111_1111_1111_1111_1111_1111_1111_0000,
        );
    }

    #[test]
    fn retrievals() {
        const DATA: u64 =
            0b0_100101001_101010101_110010011_0001_0010_0011_0100_0101_0110_0111_1000_1001;
        assert_eq!(1, value_in_square(DATA, 0));
        assert_eq!(2, value_in_square(DATA, 1));
        assert_eq!(3, value_in_square(DATA, 2));
        assert_eq!(4, value_in_square(DATA, 3));
        assert_eq!(5, value_in_square(DATA, 4));
        assert_eq!(6, value_in_square(DATA, 5));
        assert_eq!(7, value_in_square(DATA, 6));
        assert_eq!(8, value_in_square(DATA, 7));
        assert_eq!(9, value_in_square(DATA, 8));
        assert_eq!(0b100101001, values_in_row(DATA));
        assert_eq!(0b101010101, values_in_col(DATA));
        assert_eq!(0b110010011, values_in_box(DATA));
    }
}
