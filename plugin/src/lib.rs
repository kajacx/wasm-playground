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

#[no_mangle]
pub fn add_three_pair(a: i32, b: f32) -> (i32, f32) {
    let a = a.wrapping_add(1);
    let b = b + 1.0;

    let (a, b) = unsafe { add_one_pair(a, b) };

    let a = a.wrapping_add(1);
    let b = b + 1.0;

    (a, b)
}

#[allow(improper_ctypes)]
#[link(wasm_import_module = "imported_fns")]
extern "C" {
    fn add_one_i32(number: i32) -> i32;
    fn add_one_pair(a: i32, b: f32) -> (i32, f32);
}
