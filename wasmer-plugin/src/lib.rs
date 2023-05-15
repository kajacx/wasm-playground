use std::sync::Mutex;

static COUNT: Mutex<u32> = Mutex::new(0);

#[no_mangle]
pub fn increment() -> u32 {
    let mut lock = COUNT.lock().unwrap();
    let new_value = *lock + 1;
    *lock = new_value;
    new_value
}
