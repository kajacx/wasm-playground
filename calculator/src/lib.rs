use wasmtime::*;

pub fn calculate_plus_three(number: i32) -> String {
    match add_three(number) {
        Ok(result) => format!("{} + 3 = {}", number, result),
        Err(err) => format!("{err:?}"),
    }
}

fn add_three(number: i32) -> Result<i32> {
    let wat = r#"
      (module
        (func $add_three (export "add_three")
          (param $p0 i32) (result i32)
          (i32.add (local.get $p0) (i32.const 3))
        )
      )
    "#;

    let mut store = Store::<()>::default();

    let module = Module::new(&store.engine(), wat.as_bytes())?;

    let instance = Instance::new(&mut store, &module, &[])?;

    let add_three_wasm = instance.get_typed_func::<i32, i32>(&mut store, "add_three")?;

    let result = add_three_wasm.call(&mut store, number)?;

    Ok(result)
}
