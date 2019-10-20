// The following is the mapping of sudoku information onto a 64-bit integer:
//───────────────────────────────────────────────────────────────────────────────────────────────────────
// 0b0_______000000000_000000000_000000000___0000___0000___0000___0000___0000___0000___0000___0000___0000
// | unused |   row   |   col   |   box    | zero | one  | two  | three| four | five |  six | seven| eight

const SHIFT_ROW: u64 = 54;
const SHIFT_COL: u64 = 45;
const SHIFT_BOX: u64 = 36;
const SHIFT_SQUARE: [u64;9] = [ 32, 28, 24, 20, 16, 12, 8, 4, 0 ];

const CLEAR: [u64;9] = [
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

const BITS: [u64;10] = [
    0b000000000,
    0b100000000,
    0b010000000,
    0b001000000,
    0b000100000,
    0b000010000,
    0b000001000,
    0b000000100,
    0b000000010,
    0b000000001,
];

pub const fn as_bit(x: usize) -> u64 {
    BITS[x]
}

pub const fn as_bit_inverse(x: usize) -> u64 {
    !BITS[x]
}

pub const fn shift_to_row(x: u64) -> u64 {
    x << SHIFT_ROW
}

pub const fn shift_to_col(x: u64) -> u64 {
    x << SHIFT_COL
}

pub const fn shift_to_box(x: u64) -> u64 {
    x << SHIFT_BOX
}

pub const fn shift_to_row_inverse(x: u64) -> u64 {
    x << SHIFT_ROW | ((1 << SHIFT_ROW) - 1)
}

pub const fn shift_to_col_inverse(x: u64) -> u64 {
    x << SHIFT_COL | ((1 << SHIFT_COL) - 1)
}

pub const fn shift_to_box_inverse(x: u64) -> u64 {
    x << SHIFT_BOX | ((1 << SHIFT_BOX) - 1)
}

pub const fn shift_to_square(value: usize, col: usize) -> u64 {
    (value as u64) << SHIFT_SQUARE[col]
}


pub const fn value_in_square(data: u64, col: usize) -> u64 {
    (data & !CLEAR[col]) >> SHIFT_SQUARE[col]
}

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
    fn not_shifts() {
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
    fn isolations() {
        const COLUMNS: u64 = 0b0001_0010_0011_0100_0101_0110_0111_1000_1001;
        assert_eq!(1, value_in_square(COLUMNS, 0));
        assert_eq!(2, value_in_square(COLUMNS, 1));
        assert_eq!(3, value_in_square(COLUMNS, 2));
        assert_eq!(4, value_in_square(COLUMNS, 3));
        assert_eq!(5, value_in_square(COLUMNS, 4));
        assert_eq!(6, value_in_square(COLUMNS, 5));
        assert_eq!(7, value_in_square(COLUMNS, 6));
        assert_eq!(8, value_in_square(COLUMNS, 7));
        assert_eq!(9, value_in_square(COLUMNS, 8));
    }
}
