use wasmer::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let wasm_bytes = include_bytes!(
        "../../wasmer-plugin/target/wasm32-unknown-unknown/debug/wasmer_plugin.wasm"
    )
    .as_ref();

    // Create the store
    let mut store = Store::new(Cranelift::default());

    println!("Compiling module...");
    // Let's compile the Wasm module.
    let module = Module::new(&store, wasm_bytes)?;

    // Create an empty import object.
    let import_object = imports! {};

    println!("Instantiating module...");
    // Let's instantiate the Wasm module.
    let instance1 = Instance::new(&mut store, &module, &import_object)?;
    let increment1 = instance1
        .exports
        .get_typed_function::<(), u32>(&store, "increment")
        .unwrap();

    let instance2 = Instance::new(&mut store, &module, &import_object)?;
    let increment2 = instance2
        .exports
        .get_typed_function::<(), u32>(&store, "increment")
        .unwrap();

    assert_eq!(increment1.call(&mut store).unwrap(), 1);
    assert_eq!(increment1.call(&mut store).unwrap(), 2);
    assert_eq!(increment2.call(&mut store).unwrap(), 1);
    assert_eq!(increment2.call(&mut store).unwrap(), 2);
    assert_eq!(increment2.call(&mut store).unwrap(), 3);
    assert_eq!(increment2.call(&mut store).unwrap(), 4);
    assert_eq!(increment1.call(&mut store).unwrap(), 3);

    Ok(())
}
