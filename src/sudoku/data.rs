use std::cell::Cell;

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

pub const fn as_row(x: u64) -> u64 {
    x << ROW_SHIFT
}

pub const fn as_col(x: u64) -> u64 {
    x << COL_SHIFT
}

pub const fn as_box(x: u64) -> u64 {
    x << BOX_SHIFT
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
