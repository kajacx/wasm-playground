use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn compute() -> String {
    console_error_panic_hook::set_once();
    compute_it()
}

fn compute_it() -> String {
    // panic!("WHYYYY");
    "ffooo".into()
}
