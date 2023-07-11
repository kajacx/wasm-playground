use wasm_bindgen::prelude::*;

static PLUGIN_BYTES: &'static [u8] =
    include_bytes!("../../target/wasm32-unknown-unknown/debug/component.zip");

#[wasm_bindgen]
pub fn calculate(number: i32) -> String {
    calculator::calculate_plus_three(PLUGIN_BYTES, number)
}
