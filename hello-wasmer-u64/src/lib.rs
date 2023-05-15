use wasm_bindgen::prelude::*;
use wasmer::*;

#[wasm_bindgen]
pub fn test_add_three_i64() {
    console_error_panic_hook::set_once();

    let (mut store, instance) = instantiate(
        r#"(module
            (func $add_one_i64 (import "host" "add_one_i64") (param i64) (result i64))
            (func (export "add_three_i64") (param i64) (result i64)
                (i64.add (call $add_one_i64 (i64.add (local.get 0) (i64.const 1))) (i64.const 1))
            )
        )"#,
        |store| {
            imports! {
                "host" => {
                    "add_one_i64" => Function::new_typed(store, |value: i64| value.wrapping_add(1)),
                },
            }
        },
    );

    let add_three_i64 = instance
        .exports
        .get_typed_function::<i64, i64>(&store, "add_three_i64")
        .expect("should get add_three_i64 export");

    let mut numbers = Vec::<i64>::new();
    numbers.extend(-4..=4);
    numbers.extend((i64::MAX - 4)..=i64::MAX);
    numbers.extend((i64::MIN)..=(i64::MIN + 4));

    for number in numbers {
        let wasm_result = add_three_i64
            .call(&mut store, number)
            .expect("should call add_three_i64");
        let compare_result = number.wrapping_add(3);

        assert_eq!(wasm_result, compare_result)
    }
}

fn instantiate(module: &str, imports: impl FnOnce(&mut Store) -> Imports) -> (Store, Instance) {
    // let mut store = Store::new()
    let mut store = Store::default();

    let module = Module::new(&store, module).expect("should load module");

    let imports = imports(&mut store);
    let instance = Instance::new(&mut store, &module, &imports).expect("should create instance");

    (store, instance)
}
