use wasmer::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let wasm_bytes = include_bytes!(
        "../../wasmer-plugin/target/wasm32-unknown-unknown/debug/wasmer_plugin.wasm"
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

    // Create an empty import object.
    let import_object = imports! {
        "my_imports" => {
            "add_one_i32" => add_one_i32_native,
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

    Ok(())
}
