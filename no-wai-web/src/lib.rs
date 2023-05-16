#![feature(core_intrinsics)]
#![feature(structural_match)]
#![feature(no_coverage)]
#![feature(derive_clone_copy)]

use wasm_bindgen::prelude::*;
use wasmer::*;

const PLUGIN_BYTES: &'static [u8] =
    include_bytes!("../../plugin/target/wasm32-unknown-unknown/release/bevy_plugin.wasm");

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub fn add_three(number: i32) -> i32 {
    console_error_panic_hook::set_once();

    let (mut store, module) = create_store();

    let instance =
        Instance::new(&mut store, &module, &imports! {}).expect("should create instance");

    let add_three = instance
        .exports
        .get_function("add_three")
        .expect("should get function")
        .typed::<i32, i32>(&store)
        .expect("should cast to typed");

    add_three
        .call(&mut store, number)
        .expect("should add three")
}

fn create_store() -> (Store, Module) {
    let store = Store::new(Engine::default());
    //Store::new()

    // let bytes = std::fs::read("../../plugin/target/wasm32-unknown-unknown/debug/bevy_plugin.wasm")
    //     .expect("should read bytes");
    let bytes = PLUGIN_BYTES;
    let module = Module::new(&store, &bytes).expect("should create module");

    (store, module)
}
