#[no_mangle]
pub fn add_three_i64(value: i64) -> i64 {
    // value.wrapping_add(3)
    value + 3
}
