use std::sync::Mutex;

static COUNT: Mutex<u32> = Mutex::new(0);

#[no_mangle]
pub fn increment() -> u32 {
    let mut lock = COUNT.lock().unwrap();
    let new_value = *lock + 1;
    *lock = new_value;
    new_value
}

#[no_mangle]
pub fn add_three_i32(arg: i32) -> i32 {
    unsafe { add_one_i32(arg + 1) + 1 }
}

#[link(wasm_import_module = "my_imports")]
extern "C" {
    fn add_one_i32(arg: i32) -> i32;
}
