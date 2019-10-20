// The following is the mapping of sudoku information onto a 64-bit integer:
//───────────────────────────────────────────────────────────────────────────────────────────────────────
// 0b0_______000000000_000000000_000000000___0000___0000___0000___0000___0000___0000___0000___0000___0000
// | unused |   row   |   col   |   box    | zero | one  | two  | three| four | five |  six | seven| eight

const SHIFT_ROW:   u64 = 54;
const SHIFT_COL:   u64 = 45;
const SHIFT_BOX:   u64 = 36;
const SHIFT_ZERO:  u64 = 32;
const SHIFT_ONE:   u64 = 28;
const SHIFT_TWO:   u64 = 24;
const SHIFT_THREE: u64 = 20;
const SHIFT_FOUR:  u64 = 16;
const SHIFT_FIVE:  u64 = 12;
const SHIFT_SIX:   u64 = 8;
const SHIFT_SEVEN: u64 = 4;
const SHIFT_EIGHT: u64 = 0;

const CLEAR_ZERO:  u64 =
    0b1_111111111_111111111_111111111_0000_1111_1111_1111_1111_1111_1111_1111_1111;
const CLEAR_ONE:   u64 =
    0b1_111111111_111111111_111111111_1111_0000_1111_1111_1111_1111_1111_1111_1111;
const CLEAR_TWO:   u64 =
    0b1_111111111_111111111_111111111_1111_1111_0000_1111_1111_1111_1111_1111_1111;
const CLEAR_THREE: u64 =
    0b1_111111111_111111111_111111111_1111_1111_1111_0000_1111_1111_1111_1111_1111;
const CLEAR_FOUR:  u64 =
    0b1_111111111_111111111_111111111_1111_1111_1111_1111_0000_1111_1111_1111_1111;
const CLEAR_FIVE:  u64 =
    0b1_111111111_111111111_111111111_1111_1111_1111_1111_1111_0000_1111_1111_1111;
const CLEAR_SIX:   u64 =
    0b1_111111111_111111111_111111111_1111_1111_1111_1111_1111_1111_0000_1111_1111;
const CLEAR_SEVEN: u64 =
    0b1_111111111_111111111_111111111_1111_1111_1111_1111_1111_1111_1111_0000_1111;
const CLEAR_EIGHT: u64 =
    0b1_111111111_111111111_111111111_1111_1111_1111_1111_1111_1111_1111_1111_0000;

const ONE:   u64 = 0b100000000;
const TWO:   u64 = 0b010000000;
const THREE: u64 = 0b001000000;
const FOUR:  u64 = 0b000100000;
const FIVE:  u64 = 0b000010000;
const SIX:   u64 = 0b000001000;
const SEVEN: u64 = 0b000000100;
const EIGHT: u64 = 0b000000010;
const NINE:  u64 = 0b000000001;

pub fn as_bit(x: u64) -> u64 {
    match x {
        1 => ONE,
        2 => TWO,
        3 => THREE,
        4 => FOUR,
        5 => FIVE,
        6 => SIX,
        7 => SEVEN,
        8 => EIGHT,
        9 => NINE,
        _ => panic!("as_bit(): Attempted to convert invalid number to bit."),
    }
}

pub fn as_bit_inverse(x: u64) -> u64 {
    match x {
        1 => !ONE,
        2 => !TWO,
        3 => !THREE,
        4 => !FOUR,
        5 => !FIVE,
        6 => !SIX,
        7 => !SEVEN,
        8 => !EIGHT,
        9 => !NINE,
        _ => panic!("as_bit_inverse(): Attempted to convert invalid number to bit."),
    }
}

pub const fn to_row(x: u64) -> u64 {
    x << SHIFT_ROW
}

pub const fn to_col(x: u64) -> u64 {
    x << SHIFT_COL
}

pub const fn to_box(x: u64) -> u64 {
    x << SHIFT_BOX
}

pub const fn to_row_inverse(x: u64) -> u64 {
    x << SHIFT_ROW | ((1 << SHIFT_ROW) - 1)
}

pub const fn to_col_inverse(x: u64) -> u64 {
    x << SHIFT_COL | ((1 << SHIFT_COL) - 1)
}

pub const fn to_box_inverse(x: u64) -> u64 {
    x << SHIFT_BOX | ((1 << SHIFT_BOX) - 1)
}

pub fn to_square(data: u64, col: usize) -> u64 {
    match col {
        0 => data << SHIFT_ZERO,
        1 => data << SHIFT_ONE,
        2 => data << SHIFT_TWO,
        3 => data << SHIFT_THREE,
        4 => data << SHIFT_FOUR,
        5 => data << SHIFT_FIVE,
        6 => data << SHIFT_SIX,
        7 => data << SHIFT_SEVEN,
        8 => data << SHIFT_EIGHT,
        _ => panic!("to_square(): Attempted to shift to column outside of board."),
    }
}


pub fn value_in_square(data: u64, col: usize) -> u64 {
    match col {
        0 => (data & !CLEAR_ZERO)  >> SHIFT_ZERO,
        1 => (data & !CLEAR_ONE)   >> SHIFT_ONE,
        2 => (data & !CLEAR_TWO)   >> SHIFT_TWO,
        3 => (data & !CLEAR_THREE) >> SHIFT_THREE,
        4 => (data & !CLEAR_FOUR)  >> SHIFT_FOUR,
        5 => (data & !CLEAR_FIVE)  >> SHIFT_FIVE,
        6 => (data & !CLEAR_SIX)   >> SHIFT_SIX,
        7 => (data & !CLEAR_SEVEN) >> SHIFT_SEVEN,
        8 => (data & !CLEAR_EIGHT) >> SHIFT_EIGHT,
        _ => panic!("value_in_square(): Attempted to get value of square that is not on board."),
    }
}

pub fn zero_out_square(data: u64, col: usize) -> u64 {
    match col {
        0 => data & CLEAR_ZERO,
        1 => data & CLEAR_ONE,
        2 => data & CLEAR_TWO,
        3 => data & CLEAR_THREE,
        4 => data & CLEAR_FOUR,
        5 => data & CLEAR_FIVE,
        6 => data & CLEAR_SIX,
        7 => data & CLEAR_SEVEN,
        8 => data & CLEAR_EIGHT,
        _ => panic!("clear_square(): Attempted to clear square outside of board."),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn shifts() {
        const FOUR_SET_BITS: u64 = 0b1111;
        const NINE_SET_BITS: u64 = 0b111111111;
        assert_eq!(
            to_row(NINE_SET_BITS),
            0b0_111111111_000000000_000000000_0000_0000_0000_0000_0000_0000_0000_0000_0000,
        );
        assert_eq!(
            to_col(NINE_SET_BITS),
            0b0_000000000_111111111_000000000_0000_0000_0000_0000_0000_0000_0000_0000_0000,
        );
        assert_eq!(
            to_box(NINE_SET_BITS),
            0b0_000000000_000000000_111111111_0000_0000_0000_0000_0000_0000_0000_0000_0000,
        );
        assert_eq!(
            to_square(FOUR_SET_BITS, 0),
            0b0_000000000_000000000_000000000_1111_0000_0000_0000_0000_0000_0000_0000_0000,
        );
        assert_eq!(
            to_square(FOUR_SET_BITS, 1),
            0b0_000000000_000000000_000000000_0000_1111_0000_0000_0000_0000_0000_0000_0000,
        );
        assert_eq!(
            to_square(FOUR_SET_BITS, 2),
            0b0_000000000_000000000_000000000_0000_0000_1111_0000_0000_0000_0000_0000_0000,
        );
        assert_eq!(
            to_square(FOUR_SET_BITS, 3),
            0b0_000000000_000000000_000000000_0000_0000_0000_1111_0000_0000_0000_0000_0000,
        );
        assert_eq!(
            to_square(FOUR_SET_BITS, 4),
            0b0_000000000_000000000_000000000_0000_0000_0000_0000_1111_0000_0000_0000_0000,
        );
        assert_eq!(
            to_square(FOUR_SET_BITS, 5),
            0b0_000000000_000000000_000000000_0000_0000_0000_0000_0000_1111_0000_0000_0000,
        );
        assert_eq!(
            to_square(FOUR_SET_BITS, 6),
            0b0_000000000_000000000_000000000_0000_0000_0000_0000_0000_0000_1111_0000_0000,
        );
        assert_eq!(
            to_square(FOUR_SET_BITS, 7),
            0b0_000000000_000000000_000000000_0000_0000_0000_0000_0000_0000_0000_1111_0000,
        );
        assert_eq!(
            to_square(FOUR_SET_BITS, 8),
            0b0_000000000_000000000_000000000_0000_0000_0000_0000_0000_0000_0000_0000_1111,
        );
    }

    #[test]
    fn not_shifts() {
        const NINE_UNSET_BITS: u64 =
            0b1111111111111111111111111111111111111111111111111111111_000000000;
        assert_eq!(
            to_row_inverse(NINE_UNSET_BITS),
            0b1_000000000_111111111_111111111_1111_1111_1111_1111_1111_1111_1111_1111_1111,
        );
        assert_eq!(
            to_col_inverse(NINE_UNSET_BITS),
            0b1_111111111_000000000_111111111_1111_1111_1111_1111_1111_1111_1111_1111_1111,
        );
        assert_eq!(
            to_box_inverse(NINE_UNSET_BITS),
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
