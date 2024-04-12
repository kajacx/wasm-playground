use std::error::Error;

use wasmtime::*;
use wasmtime_wasi::{sync::WasiCtxBuilder, WasiCtx};

const PLUGIN_BYTES: &'static [u8] =
    include_bytes!("../../plugin/target/wasm32-unknown-unknown/debug/wasmtime_plugin.wasm");

const WASI_BYTES: &'static [u8] =
    include_bytes!("../../plugin-wasi/target/wasm32-wasi/debug/wasmtime_plugin.wasm");

fn main() -> Result<(), Box<dyn Error>> {
    run_wasm()?;
    run_wasi()?;

    Ok(())
}

#[allow(warnings)]
fn run_wasm() -> Result<(), Box<dyn Error>> {
    let mut store = Store::<()>::default();

    let module = Module::new(&store.engine(), PLUGIN_BYTES)?;

    let mut linker = Linker::new(store.engine());
    // linker.func_wrap("imported_fns", "add_one_i32", |_: Caller<()>, val: i32| {
    //     val.wrapping_add(1)
    // })?;
    linker.func_wrap(
        "imported_fns",
        "add_one_pair",
        |_: Caller<()>, a: i32, b: f32| (a.wrapping_add(1), b + 1.0),
    )?;

    linker.func_new(
        "imported_fns",
        "add_one_i32",
        FuncType::new([ValType::I32], [ValType::I32]),
        |_caller, args, rets| match args[0] {
            Val::I32(val) => {
                rets[0] = Val::I32(val + 1);
                Ok(())
            }
            _ => panic!(),
        },
    )?;

    let instance = linker.instantiate(&mut store, &module)?;

    // instance.exports(&mut store).
    let memory = instance.get_memory(&mut store, "memory").unwrap();
    memory.write(&mut store, 50, &[8, 9]);
    memory.read(&mut store, 90, &mut [8]);

    let bytes = vec![0u8; 3];
    let bytes = bytes.into_boxed_slice();
    let addr = Box::leak(bytes);
    let add = addr as *mut [u8] as *mut u8;

    unsafe {
        std::ptr::read(add);
        // std::ptr::write(dst, src)
    }

    // instance.get
    // Call imported fn
    // let add_three_i32 = instance.get_typed_func::<i32, i32>(&mut store, "add_three_i32")?;

    // let returned = add_three_i32.call(&mut store, 5)?;
    let add_three_i32 = instance
        .get_func(&mut store, "add_three_i32")
        .expect("get add_three_i32");
    let mut results = [Val::I32(0)];
    add_three_i32.call(&mut store, &[Val::I32(5)], &mut results)?;
    println!("Add three: {results:?}");
    // assert_eq!(returned, 5 + 3);

    // Multiple arguments
    let add_i32 = instance.get_typed_func::<(i32, i32), i32>(&mut store, "add_i32")?;

    let returned = add_i32.call(&mut store, (5, 10))?;
    println!("Add: {returned:?}");
    assert_eq!(returned, 5 + 10);

    // Multiple results
    let add_sub_ten_i32 =
        instance.get_typed_func::<i32, (i32, i32)>(&mut store, "add_sub_ten_i32")?;

    let returned = add_sub_ten_i32.call(&mut store, 50)?;
    println!("Add sub ten: {returned:?}");
    assert_eq!(returned, (50 + 10, 50 - 10));

    // Pair of numbers
    let add_three_pair =
        instance.get_typed_func::<(i32, f32), (i32, f32)>(&mut store, "add_three_pair")?;

    let returned = add_three_pair.call(&mut store, (5, 15.5))?;
    println!("Add three pair: {returned:?}");
    assert_eq!(returned, (5 + 3, 15.5 + 3.0));

    Ok(())
}

fn run_wasi() -> Result<(), Box<dyn Error>> {
    let wasi = WasiCtxBuilder::new().inherit_stdio().build();

    let mut store = Store::new(&Engine::default(), wasi);

    let module = Module::new(&store.engine(), WASI_BYTES)?;

    let mut linker = Linker::<WasiCtx>::new(store.engine());
    linker.func_wrap("imported_fns", "add_one_i32", |_: Caller<_>, val: i32| {
        val.wrapping_add(1)
    })?;

    wasmtime_wasi::add_to_linker(&mut linker, |data| data)?;

    let instance = linker.instantiate(&mut store, &module)?;

    // Call imported fn
    let add_three_i32 = instance.get_typed_func::<i32, i32>(&mut store, "add_three_i32")?;

    let returned = add_three_i32.call(&mut store, 5)?;
    println!("Add three WASI: {returned:?}");
    assert_eq!(returned, 5 + 3);

    Ok(())
}
