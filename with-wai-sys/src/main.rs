use wai_bindgen_wasmer::wasmer::*;

const PLUGIN_BYTES: &'static [u8] =
    include_bytes!("../../plugin/target/wasm32-unknown-unknown/release/bevy_plugin.wasm");

fn add_three(number: i32) -> i32 {
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

fn main() {
    println!("15 + 3 = {}", add_three(15));
}
