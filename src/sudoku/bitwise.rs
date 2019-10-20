const ONE:   u64 = 0b0000000000000000000000000000000000000000000000000000000_100000000;
const TWO:   u64 = 0b0000000000000000000000000000000000000000000000000000000_010000000;
const THREE: u64 = 0b0000000000000000000000000000000000000000000000000000000_001000000;
const FOUR:  u64 = 0b0000000000000000000000000000000000000000000000000000000_000100000;
const FIVE:  u64 = 0b0000000000000000000000000000000000000000000000000000000_000010000;
const SIX:   u64 = 0b0000000000000000000000000000000000000000000000000000000_000001000;
const SEVEN: u64 = 0b0000000000000000000000000000000000000000000000000000000_000000100;
const EIGHT: u64 = 0b0000000000000000000000000000000000000000000000000000000_000000010;
const NINE:  u64 = 0b0000000000000000000000000000000000000000000000000000000_000000001;

const SHIFT_EIGHT: u64 = 0;
const SHIFT_SEVEN: u64 = 4;
const SHIFT_SIX:   u64 = 8;
const SHIFT_FIVE:  u64 = 12;
const SHIFT_FOUR:  u64 = 16;
const SHIFT_THREE: u64 = 20;
const SHIFT_TWO:   u64 = 24;
const SHIFT_ONE:   u64 = 28;
const SHIFT_ZERO:  u64 = 32;
const SHIFT_BOX:   u64 = 36;
const SHIFT_COL:   u64 = 45;
const SHIFT_ROW:   u64 = 54;

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

pub fn as_not_bit(x: u64) -> u64 {
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
        _ => panic!("as_not_bit(): Attempted to convert invalid number to bit."),
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

pub fn to_cell(data: u64, col: u64) -> u64 {
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
        _ => panic!("to_cell(): Attempted to shift to column outside of board."),
    }
}

pub fn value_in_cell(data: u64, col: u64) -> u64 {
    match col {
        0 => data & !CLEAR_ZERO  >> SHIFT_ZERO,
        1 => data & !CLEAR_ONE   >> SHIFT_ONE,
        2 => data & !CLEAR_TWO   >> SHIFT_TWO,
        3 => data & !CLEAR_THREE >> SHIFT_THREE,
        4 => data & !CLEAR_FOUR  >> SHIFT_FOUR,
        5 => data & !CLEAR_FIVE  >> SHIFT_FIVE,
        6 => data & !CLEAR_SIX   >> SHIFT_SIX,
        7 => data & !CLEAR_SEVEN >> SHIFT_SEVEN,
        8 => data & !CLEAR_EIGHT >> SHIFT_EIGHT,
        _ => panic!("value_in_cell(): Attempted to get value of cell that is not on board."),
    }
}

pub fn clear_cell(data: u64, col: u64) -> u64 {
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
        _ => panic!("clear_cell(): Attempted to clear cell outside of board."),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn shifts() {
        const NINE_SET_BITS: u64 =
            0b0000000000000000000000000000000000000000000000000000000_111111111;
        const FOUR_SET_BITS: u64 =
            0b000000000000000000000000000000000000000000000000000000000000_1111;
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
            to_cell(FOUR_SET_BITS, 0),
            0b0_000000000_000000000_000000000_1111_0000_0000_0000_0000_0000_0000_0000_0000,
        );
        assert_eq!(
            to_cell(FOUR_SET_BITS, 1),
            0b0_000000000_000000000_000000000_0000_1111_0000_0000_0000_0000_0000_0000_0000,
        );
        assert_eq!(
            to_cell(FOUR_SET_BITS, 2),
            0b0_000000000_000000000_000000000_0000_0000_1111_0000_0000_0000_0000_0000_0000,
        );
        assert_eq!(
            to_cell(FOUR_SET_BITS, 3),
            0b0_000000000_000000000_000000000_0000_0000_0000_1111_0000_0000_0000_0000_0000,
        );
        assert_eq!(
            to_cell(FOUR_SET_BITS, 4),
            0b0_000000000_000000000_000000000_0000_0000_0000_0000_1111_0000_0000_0000_0000,
        );
        assert_eq!(
            to_cell(FOUR_SET_BITS, 5),
            0b0_000000000_000000000_000000000_0000_0000_0000_0000_0000_1111_0000_0000_0000,
        );
        assert_eq!(
            to_cell(FOUR_SET_BITS, 6),
            0b0_000000000_000000000_000000000_0000_0000_0000_0000_0000_0000_1111_0000_0000,
        );
        assert_eq!(
            to_cell(FOUR_SET_BITS, 7),
            0b0_000000000_000000000_000000000_0000_0000_0000_0000_0000_0000_0000_1111_0000,
        );
        assert_eq!(
            to_cell(FOUR_SET_BITS, 8),
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
            clear_cell(ALL_SET_BITS, 0),
            0b1_111111111_111111111_111111111_0000_1111_1111_1111_1111_1111_1111_1111_1111,
        );
        assert_eq!(
            clear_cell(ALL_SET_BITS, 1),
            0b1_111111111_111111111_111111111_1111_0000_1111_1111_1111_1111_1111_1111_1111,
        );
        assert_eq!(
            clear_cell(ALL_SET_BITS, 2),
            0b1_111111111_111111111_111111111_1111_1111_0000_1111_1111_1111_1111_1111_1111,
        );
        assert_eq!(
            clear_cell(ALL_SET_BITS, 3),
            0b1_111111111_111111111_111111111_1111_1111_1111_0000_1111_1111_1111_1111_1111,
        );
        assert_eq!(
            clear_cell(ALL_SET_BITS, 4),
            0b1_111111111_111111111_111111111_1111_1111_1111_1111_0000_1111_1111_1111_1111,
        );
        assert_eq!(
            clear_cell(ALL_SET_BITS, 5),
            0b1_111111111_111111111_111111111_1111_1111_1111_1111_1111_0000_1111_1111_1111,
        );
        assert_eq!(
            clear_cell(ALL_SET_BITS, 6),
            0b1_111111111_111111111_111111111_1111_1111_1111_1111_1111_1111_0000_1111_1111,
        );
        assert_eq!(
            clear_cell(ALL_SET_BITS, 7),
            0b1_111111111_111111111_111111111_1111_1111_1111_1111_1111_1111_1111_0000_1111,
        );
        assert_eq!(
            clear_cell(ALL_SET_BITS, 8),
            0b1_111111111_111111111_111111111_1111_1111_1111_1111_1111_1111_1111_1111_0000,
        );
    }

    #[test]
    fn isolations() {
        const ALL_BITS_SET: u64 =
            0b1_111111111_111111111_111111111_1111_1111_1111_1111_1111_1111_1111_1111_1111;
        const FOUR_BITS_SET: u64 =
            0b0_000000000_000000000_000000000_0000_0000_0000_0000_0000_0000_0000_0000_1111;

        assert_eq!(FOUR_BITS_SET, value_in_cell(ALL_BITS_SET, 0));
        assert_eq!(FOUR_BITS_SET, value_in_cell(ALL_BITS_SET, 1));
        assert_eq!(FOUR_BITS_SET, value_in_cell(ALL_BITS_SET, 2));
        assert_eq!(FOUR_BITS_SET, value_in_cell(ALL_BITS_SET, 3));
        assert_eq!(FOUR_BITS_SET, value_in_cell(ALL_BITS_SET, 4));
        assert_eq!(FOUR_BITS_SET, value_in_cell(ALL_BITS_SET, 5));
        assert_eq!(FOUR_BITS_SET, value_in_cell(ALL_BITS_SET, 6));
        assert_eq!(FOUR_BITS_SET, value_in_cell(ALL_BITS_SET, 7));
        assert_eq!(FOUR_BITS_SET, value_in_cell(ALL_BITS_SET, 8));
    }
}
