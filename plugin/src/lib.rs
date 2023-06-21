#[no_mangle]
pub fn add_i32(a: i32, b: i32) -> i32 {
    a.wrapping_add(b)
}

#[no_mangle]
pub fn add_sub_ten_i32(number: i32) -> (i32, i32) {
    (number.wrapping_add(10), number.wrapping_sub(10))
}

#[no_mangle]
pub fn add_three_i32(number: i32) -> i32 {
    unsafe { add_one_i32(number.wrapping_add(1)).wrapping_add(1) }
}

#[link(wasm_import_module = "imported_fns")]
extern "C" {
    fn add_one_i32(number: i32) -> i32;
}
