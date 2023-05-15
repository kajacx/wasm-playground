use wasmer::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let wasm_bytes = include_bytes!(
        "../../wasmer-plugin/target/wasm32-unknown-unknown/debug/wasmer2_plugin.wasm"
    )
    .as_ref();

    // Use Singlepass compiler with the default settings
    let compiler = Singlepass::default();
    let engine = Universal::new(compiler).engine();

    // Create the store
    let store = Store::new(&engine);

    println!("Compiling module...");
    // Let's compile the Wasm module.
    let module = Module::new(&store, wasm_bytes)?;

    fn add_one_i32(a: i32) -> i32 {
        a + 1
    }
    let add_one_i32_native = Function::new_native(&store, add_one_i32);

    fn add_one_f32(a: f32) -> f32 {
        a + 1.0
    }
    let add_one_f32_native = Function::new_native(&store, add_one_f32);

    // Create an empty import object.
    let import_object = imports! {
        "my_imports" => {
            "add_one_i32" => add_one_i32_native,
            "add_one_f32" => add_one_f32_native,
            "imported_takes_bool" => Function::new_native(&store, |arg: u8| {
                println!("Host takes bool: {arg}");
            }),
            "imported_takes_u8" => Function::new_native(&store, |arg: u8| {
                println!("Host takes u8: {arg}");
            }),
            "imported_returns_bool" => Function::new_native(&store, || {
                2
            }),
            "imported_returns_u8" => Function::new_native(&store, || {
                8
            }),
        }
    };

    println!("Instantiating module...");
    // Let's instantiate the Wasm module.
    let instance = Instance::new(&module, &import_object)?;

    let add_three_i32 = instance.exports.get_function("add_three_i32")?;

    println!("Calling `add_three_i32` function...");
    let results = add_three_i32.call(&[Value::I32(5)])?;

    println!("Results: {:?}", results);
    assert_eq!(results.to_vec(), vec![Value::I32(8)]);

    let add_three_f32 = instance.exports.get_function("add_three_f32")?;

    println!("Calling `add_three_f32` function...");
    let results = add_three_f32.call(&[Value::F32(5.5)])?;

    println!("Results: {:?}", results);
    //assert_eq!(results.to_vec(), vec![Value::F32(8.5)]);

    let exported_returns_bool = instance.exports.get_function("exported_returns_bool")?;

    println!("Calling `exported_returns_bool` function...");
    let results = exported_returns_bool.call(&[])?;

    println!("Results: {:?}", results);
    assert_eq!(results.to_vec(), vec![Value::I32(0)]);

    Ok(())
}
