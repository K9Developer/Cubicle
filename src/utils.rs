pub fn bit_length(num: i32) -> u32 {
    if num == 0 {
        return 1;
    }
    let abs_num = num.abs();
    32 - abs_num.leading_zeros()
}
