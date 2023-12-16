// use crate::js_impl::*;
use wasm_bindgen::{
    convert::{FromWasmAbi, IntoWasmAbi, ReturnWasmAbi},
    prelude::*,
};

// mod js_impl;

#[wasm_bindgen]
pub async fn compute() -> String {
    console_error_panic_hook::set_once();
    // compute_it().await
    "foo".into()
}

// async fn compute_it() -> String {
//     let instance = instantiate(|mut point| {
//         point.x += 100;
//         point
//     })
//     .await;

//     instance.run();

//     let point = instance.move_point(Point { x: 50, y: 50 });

//     format!("Moved point: {point:?}")
// }

fn main() {
    let closure = Closure::<dyn Fn(i32) -> i32>::new(|i| i + 5);
    let closure = Closure::<dyn Fn(i32) -> Result<i32, JsValue>>::new(|i| Ok(i + 5));

    make_closures::<i32>();
    make_closures_fixed::<i32>();
}

fn make_closures<T: IntoWasmAbi + 'static>() {
    let closure = Closure::<dyn Fn(i32) -> T>::new(|_| todo!());
    let closure = Closure::<dyn Fn(i32) -> Result<T, JsValue>>::new(|_| todo!());
}

// But this is where it gets weird
fn make_closures_fixed<T: IntoWasmAbi + 'static>()
where
    Result<T, JsValue>: ReturnWasmAbi,
{
    let closure = Closure::<dyn Fn(i32) -> T>::new(|_| todo!()); //works
    let closure = Closure::<dyn Fn(i32) -> Result<T, JsValue>>::new(|_| todo!());
    // this now works
}

fn accept_return_wasm_abi<T: ReturnWasmAbi>() {}

fn test_it_i32() {
    accept_return_wasm_abi::<i32>(); // works
    accept_return_wasm_abi::<Result<i32, JsValue>>(); // works
}

fn test_it_into_wasm_abi<T: IntoWasmAbi + 'static>() {
    accept_return_wasm_abi::<T>(); // works
    accept_return_wasm_abi::<Result<T, JsValue>>();
    // again, works in minimal reproducible example, but not in main project
}

fn test_it_into_wasm_abi_fixed<T: IntoWasmAbi + 'static>()
where
    Result<T, JsValue>: ReturnWasmAbi, // <-- added this trait bound
{
    accept_return_wasm_abi::<T>(); // works
    accept_return_wasm_abi::<Result<T, JsValue>>();
}
