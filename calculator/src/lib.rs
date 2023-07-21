use wasm_bridge::component::*;
use wasm_bridge::{Config, Engine, Result, Store};

static COMPONENT_BYTES: &'static [u8] =
    include_bytes!("../../target/wasm32-unknown-unknown/debug/component.wasm");

wasm_bridge::component::bindgen!({
    path: "../protocol.wit",
    world: "calculator",
});

struct Imports;

impl CalculatorImports for Imports {
    fn add_one(&mut self, num: i32) -> Result<i32> {
        Ok(num + 1)
    }
}

pub fn calculate_plus_three(number: i32) -> String {
    match add_three(number) {
        Ok(result) => format!("{} + 3 = {}", number, result),
        Err(err) => format!("Error: {err:?}"),
    }
}

fn add_three(number: i32) -> Result<i32> {
    let mut config = Config::new();
    config.wasm_component_model(true);

    let engine = Engine::new(&config)?;
    let mut store = Store::new(&engine, Imports);

    let component = Component::new(&store.engine(), COMPONENT_BYTES)?;

    let mut linker = Linker::new(store.engine());
    Calculator::add_to_linker(&mut linker, |data| data)?;

    let (instance, _) = Calculator::instantiate(&mut store, &component, &linker)?;

    Ok(instance.call_add_three(&mut store, number)?)
}
