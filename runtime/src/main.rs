use std::error::Error;

use wasmtime::*;

const PLUGIN_BYTES: &'static [u8] =
    include_bytes!("../../plugin/target/wasm32-unknown-unknown/debug/wasmtime_plugin.wasm");

fn main() -> Result<(), Box<dyn Error>> {
    let mut store = Store::<()>::default();

    let module = Module::new(&store.engine(), PLUGIN_BYTES)?;

    let instance = Instance::new(&mut store, &module, &[])?;

    // Multiple arguments
    let add_i32 = instance.get_typed_func::<(i32, i32), i32>(&mut store, "add_i32")?;

    let returned = add_i32.call(&mut store, (5, 10))?;
    assert_eq!(returned, 5 + 10);

    // Multiple results
    let add_sub_ten_i32 =
        instance.get_typed_func::<i32, (i32, i32)>(&mut store, "add_sub_ten_i32")?;

    let res = add_sub_ten_i32.call(&mut store, 50)?;
    println!("RES: {:?}", res);
    // assert_eq!(a, 50 + 10);
    // assert_eq!(b, 50 - 10);

    Ok(())
}
