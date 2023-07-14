use crate::js_impl::*;
use wasm_bindgen::prelude::*;

mod js_impl;

#[wasm_bindgen]
pub async fn compute() -> String {
    console_error_panic_hook::set_once();
    compute_it().await
}

async fn compute_it() -> String {
    let instance = instantiate(|mut point| {
        point.x += 100;
        point
    })
    .await;

    instance.run();

    let point = instance.move_point(Point { x: 50, y: 50 });

    format!("Moved point: {point:?}")
}
