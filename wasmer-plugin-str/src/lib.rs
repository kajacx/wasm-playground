#[no_mangle]
pub fn do_accounting() -> f64 {
    get_balance("account1") + get_balance("account2")
}

fn get_balance(address: &str) -> f64 {
    let mut bytes = Vec::with_capacity(address.len() + 1);
    for byte in address.as_bytes() {
        bytes.push(*byte);
    }
    bytes.push(0u8);
    unsafe { get_balance_raw(bytes.as_ref() as *const [u8] as *const u8 as usize as u32) }
}

#[link(wasm_import_module = "my_imports")]
extern "C" {
    fn get_balance_raw(address_ptr: u32) -> f64;
}
