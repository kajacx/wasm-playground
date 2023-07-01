use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn compute() -> String {
    compute_it()
}

fn compute_it() -> String {
    "ffooo".into()
}
