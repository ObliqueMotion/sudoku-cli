const ONE: u64 = 0b0000000000000000000000000000000000000000000000000000000_100000000;
const TWO: u64 = 0b0000000000000000000000000000000000000000000000000000000_010000000;
const THREE: u64 = 0b0000000000000000000000000000000000000000000000000000000_001000000;
const FOUR: u64 = 0b0000000000000000000000000000000000000000000000000000000_000100000;
const FIVE: u64 = 0b0000000000000000000000000000000000000000000000000000000_000010000;
const SIX: u64 = 0b0000000000000000000000000000000000000000000000000000000_000001000;
const SEVEN: u64 = 0b0000000000000000000000000000000000000000000000000000000_000000100;
const EIGHT: u64 = 0b0000000000000000000000000000000000000000000000000000000_000000010;
const NINE: u64 = 0b0000000000000000000000000000000000000000000000000000000_000000001;

const ROW_SHIFT: u64 = 54;
const COL_SHIFT: u64 = 45;
const BOX_SHIFT: u64 = 36;
const ZERO_SHIFT: u64 = 32;
const ONE_SHIFT: u64 = 28;
const TWO_SHIFT: u64 = 24;
const THREE_SHIFT: u64 = 20;
const FOUR_SHIFT: u64 = 16;
const FIVE_SHIFT: u64 = 12;
const SIX_SHIFT: u64 = 8;
const SEVEN_SHIFT: u64 = 4;
const EIGHT_SHIFT: u64 = 0;

const CLEAR_ZERO: u64 =
    0b1_111111111_111111111_111111111_0000_1111_1111_1111_1111_1111_1111_1111_1111;
const CLEAR_ONE: u64 =
    0b1_111111111_111111111_111111111_1111_0000_1111_1111_1111_1111_1111_1111_1111;
const CLEAR_TWO: u64 =
    0b1_111111111_111111111_111111111_1111_1111_0000_1111_1111_1111_1111_1111_1111;
const CLEAR_THREE: u64 =
    0b1_111111111_111111111_111111111_1111_1111_1111_0000_1111_1111_1111_1111_1111;
const CLEAR_FOUR: u64 =
    0b1_111111111_111111111_111111111_1111_1111_1111_1111_0000_1111_1111_1111_1111;
const CLEAR_FIVE: u64 =
    0b1_111111111_111111111_111111111_1111_1111_1111_1111_1111_0000_1111_1111_1111;
const CLEAR_SIX: u64 =
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

pub const fn as_row(x: u64) -> u64 {
    x << ROW_SHIFT
}

pub const fn as_col(x: u64) -> u64 {
    x << COL_SHIFT
}

pub const fn as_box(x: u64) -> u64 {
    x << BOX_SHIFT
}

pub const fn as_not_row(x: u64) -> u64 {
    x << ROW_SHIFT | ((1 << ROW_SHIFT) - 1)
}

pub const fn as_not_col(x: u64) -> u64 {
    x << COL_SHIFT | ((1 << COL_SHIFT) - 1)
}

pub const fn as_not_box(x: u64) -> u64 {
    x << BOX_SHIFT | ((1 << BOX_SHIFT) - 1)
}

pub fn to_cell(data: u64, col: u64) -> u64 {
    match col {
        0 => data << ZERO_SHIFT,
        1 => data << ONE_SHIFT,
        2 => data << TWO_SHIFT,
        3 => data << THREE_SHIFT,
        4 => data << FOUR_SHIFT,
        5 => data << FIVE_SHIFT,
        6 => data << SIX_SHIFT,
        7 => data << SEVEN_SHIFT,
        8 => data << EIGHT_SHIFT,
        _ => panic!("to_cell(): Attempted to shift to column outside of board."),
    }
}

pub fn value_in_cell(data: u64, col: u64) -> u64 {
    match col {
        0 => data & !CLEAR_ZERO >> ZERO_SHIFT,
        1 => data & !CLEAR_ONE >> ONE_SHIFT,
        2 => data & !CLEAR_TWO >> TWO_SHIFT,
        3 => data & !CLEAR_THREE >> THREE_SHIFT,
        4 => data & !CLEAR_FOUR >> FOUR_SHIFT,
        5 => data & !CLEAR_FIVE >> FIVE_SHIFT,
        6 => data & !CLEAR_SIX >> SIX_SHIFT,
        7 => data & !CLEAR_SEVEN >> SEVEN_SHIFT,
        8 => data & !CLEAR_EIGHT >> EIGHT_SHIFT,
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
            as_row(NINE_SET_BITS),
            0b0_111111111_000000000_000000000_0000_0000_0000_0000_0000_0000_0000_0000_0000,
        );
        assert_eq!(
            as_col(NINE_SET_BITS),
            0b0_000000000_111111111_000000000_0000_0000_0000_0000_0000_0000_0000_0000_0000,
        );
        assert_eq!(
            as_box(NINE_SET_BITS),
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
            as_not_row(NINE_UNSET_BITS),
            0b1_000000000_111111111_111111111_1111_1111_1111_1111_1111_1111_1111_1111_1111,
        );
        assert_eq!(
            as_not_col(NINE_UNSET_BITS),
            0b1_111111111_000000000_111111111_1111_1111_1111_1111_1111_1111_1111_1111_1111,
        );
        assert_eq!(
            as_not_box(NINE_UNSET_BITS),
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
