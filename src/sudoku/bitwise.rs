use std::cell::Cell;

const ONE: u64 = 0b0000000000000000000000000000000000000000000000000000000_100000000;
const TWO: u64 = 0b0000000000000000000000000000000000000000000000000000000_010000000;
const THREE: u64 = 0b0000000000000000000000000000000000000000000000000000000_001000000;
const FOUR: u64 = 0b0000000000000000000000000000000000000000000000000000000_000100000;
const FIVE: u64 = 0b0000000000000000000000000000000000000000000000000000000_000010000;
const SIX: u64 = 0b0000000000000000000000000000000000000000000000000000000_000001000;
const SEVEN: u64 = 0b0000000000000000000000000000000000000000000000000000000_000000100;
const EIGHT: u64 = 0b0000000000000000000000000000000000000000000000000000000_000000010;
const NINE: u64 = 0b0000000000000000000000000000000000000000000000000000000_000000001;

const NOT_ONE: u64 = !ONE;
const NOT_TWO: u64 = !TWO;
const NOT_THREE: u64 = !THREE;
const NOT_FOUR: u64 = !FOUR;
const NOT_FIVE: u64 = !FIVE;
const NOT_SIX: u64 = !SIX;
const NOT_SEVEN: u64 = !SEVEN;
const NOT_EIGHT: u64 = !EIGHT;
const NOT_NINE: u64 = !NINE;

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
        1 => NOT_ONE,
        2 => NOT_TWO,
        3 => NOT_THREE,
        4 => NOT_FOUR,
        5 => NOT_FIVE,
        6 => NOT_SIX,
        7 => NOT_SEVEN,
        8 => NOT_EIGHT,
        9 => NOT_NINE,
        _ => panic!("as_not_bit(): Attempted to convert invalid number to bit."),
    }
}

pub const fn as_row(x: u64) -> u64 {
    x << ROW_SHIFT
}

pub const fn as_not_row(x: u64) -> u64 {
    x << ROW_SHIFT | ((1 << ROW_SHIFT) - 1)
}

pub const fn as_col(x: u64) -> u64 {
    x << COL_SHIFT
}

pub const fn as_not_col(x: u64) -> u64 {
    x << COL_SHIFT | ((1 << COL_SHIFT) - 1)
}

pub const fn as_box(x: u64) -> u64 {
    x << BOX_SHIFT
}

pub const fn as_not_box(x: u64) -> u64 {
    x << BOX_SHIFT | ((1 << BOX_SHIFT) - 1)
}

pub const fn as_zero(x: u64) -> u64 {
    x << ZERO_SHIFT
}

pub const fn clear_zero(x: u64) -> u64 {
    x & CLEAR_ZERO
}

pub const fn as_one(x: u64) -> u64 {
    x << ONE_SHIFT
}

pub const fn clear_one(x: u64) -> u64 {
    x & CLEAR_ONE
}

pub const fn as_two(x: u64) -> u64 {
    x << TWO_SHIFT
}

pub const fn clear_two(x: u64) -> u64 {
    x & CLEAR_TWO
}

pub const fn as_three(x: u64) -> u64 {
    x << THREE_SHIFT
}

pub const fn clear_three(x: u64) -> u64 {
    x & CLEAR_THREE
}

pub const fn as_four(x: u64) -> u64 {
    x << FOUR_SHIFT
}

pub const fn clear_four(x: u64) -> u64 {
    x & CLEAR_FOUR
}

pub const fn as_five(x: u64) -> u64 {
    x << FIVE_SHIFT
}

pub const fn clear_five(x: u64) -> u64 {
    x & CLEAR_FIVE
}

pub const fn as_six(x: u64) -> u64 {
    x << SIX_SHIFT
}

pub const fn clear_six(x: u64) -> u64 {
    x & CLEAR_SIX
}

pub const fn as_seven(x: u64) -> u64 {
    x << SEVEN_SHIFT
}

pub const fn clear_seven(x: u64) -> u64 {
    x & CLEAR_SEVEN
}

pub const fn as_eight(x: u64) -> u64 {
    x << EIGHT_SHIFT
}

pub const fn clear_eight(x: u64) -> u64 {
    x & CLEAR_EIGHT
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
            as_zero(FOUR_SET_BITS),
            0b0_000000000_000000000_000000000_1111_0000_0000_0000_0000_0000_0000_0000_0000,
        );
        assert_eq!(
            as_one(FOUR_SET_BITS),
            0b0_000000000_000000000_000000000_0000_1111_0000_0000_0000_0000_0000_0000_0000,
        );
        assert_eq!(
            as_two(FOUR_SET_BITS),
            0b0_000000000_000000000_000000000_0000_0000_1111_0000_0000_0000_0000_0000_0000,
        );
        assert_eq!(
            as_three(FOUR_SET_BITS),
            0b0_000000000_000000000_000000000_0000_0000_0000_1111_0000_0000_0000_0000_0000,
        );
        assert_eq!(
            as_four(FOUR_SET_BITS),
            0b0_000000000_000000000_000000000_0000_0000_0000_0000_1111_0000_0000_0000_0000,
        );
        assert_eq!(
            as_five(FOUR_SET_BITS),
            0b0_000000000_000000000_000000000_0000_0000_0000_0000_0000_1111_0000_0000_0000,
        );
        assert_eq!(
            as_six(FOUR_SET_BITS),
            0b0_000000000_000000000_000000000_0000_0000_0000_0000_0000_0000_1111_0000_0000,
        );
        assert_eq!(
            as_seven(FOUR_SET_BITS),
            0b0_000000000_000000000_000000000_0000_0000_0000_0000_0000_0000_0000_1111_0000,
        );
        assert_eq!(
            as_eight(FOUR_SET_BITS),
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
            clear_zero(ALL_SET_BITS),
            0b1_111111111_111111111_111111111_0000_1111_1111_1111_1111_1111_1111_1111_1111,
        );
        assert_eq!(
            clear_one(ALL_SET_BITS),
            0b1_111111111_111111111_111111111_1111_0000_1111_1111_1111_1111_1111_1111_1111,
        );
        assert_eq!(
            clear_two(ALL_SET_BITS),
            0b1_111111111_111111111_111111111_1111_1111_0000_1111_1111_1111_1111_1111_1111,
        );
        assert_eq!(
            clear_three(ALL_SET_BITS),
            0b1_111111111_111111111_111111111_1111_1111_1111_0000_1111_1111_1111_1111_1111,
        );
        assert_eq!(
            clear_four(ALL_SET_BITS),
            0b1_111111111_111111111_111111111_1111_1111_1111_1111_0000_1111_1111_1111_1111,
        );
        assert_eq!(
            clear_five(ALL_SET_BITS),
            0b1_111111111_111111111_111111111_1111_1111_1111_1111_1111_0000_1111_1111_1111,
        );
        assert_eq!(
            clear_six(ALL_SET_BITS),
            0b1_111111111_111111111_111111111_1111_1111_1111_1111_1111_1111_0000_1111_1111,
        );
        assert_eq!(
            clear_seven(ALL_SET_BITS),
            0b1_111111111_111111111_111111111_1111_1111_1111_1111_1111_1111_1111_0000_1111,
        );
        assert_eq!(
            clear_eight(ALL_SET_BITS),
            0b1_111111111_111111111_111111111_1111_1111_1111_1111_1111_1111_1111_1111_0000,
        );
    }
}
