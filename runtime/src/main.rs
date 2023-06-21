use std::error::Error;

use wasmtime::*;

const PLUGIN_BYTES: &'static [u8] =
    include_bytes!("../../plugin/target/wasm32-unknown-unknown/debug/wasmtime_plugin.wasm");

fn main() -> Result<(), Box<dyn Error>> {
    let mut store = Store::<()>::default();

    let module = Module::new(&store.engine(), PLUGIN_BYTES)?;

    let mut linker = Linker::new(store.engine());
    linker.func_wrap("imported_fns", "add_one_i32", |_: Caller<()>, val: i32| {
        val.wrapping_add(1)
    })?;
    let instance = linker.instantiate(&mut store, &module)?;

    // let instance = Instance::new(&mut store, &module, &[])?;

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

    // Call imported fn

    let add_three_i32 = instance.get_typed_func::<i32, i32>(&mut store, "add_three_i32")?;
    let returned = add_three_i32.call(&mut store, 5)?;
    println!("IT IS: {}", returned);

    Ok(())
}
