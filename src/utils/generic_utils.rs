use std::num::NonZeroU32;

pub fn bit_length(num: i32) -> u32 {
    if num == 0 {
        return 1;
    }
    let abs_num = num.abs();
    32 - abs_num.leading_zeros()
}

pub fn div_rem_nonzero(a: i32, b: i32) -> (i32, i32) {
    assert!(b > 0);
    (a / b, a % b)
}