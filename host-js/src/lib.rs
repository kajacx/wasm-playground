use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn calculate(number: i32) -> String {
    calculator::calculate_plus_three(number)
}
