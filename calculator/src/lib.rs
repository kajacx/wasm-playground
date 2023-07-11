use anyhow::Result;
use wasm_bridge::component::*;
use wasm_bridge::{Config, Engine, Store};

wasm_bridge::component::bindgen!({
    path: "../protocol.wit",
    world: "calculator",
});

pub fn calculate_plus_three(component_bytes: &[u8], number: i32) -> String {
    match add_three(component_bytes, number) {
        Ok(result) => format!("{} + 3 = {}", number, result),
        Err(err) => format!("{err:?}"),
    }
}

fn add_three(component_bytes: &[u8], number: i32) -> Result<i32> {
    let mut config = Config::new();
    config.wasm_component_model(true);

    let engine = Engine::new(&config)?;
    let mut store = Store::new(&engine, ());

    let component = Component::new(&store.engine(), &component_bytes)?;

    let linker = Linker::new(store.engine());
    let (instance, _) = Calculator::instantiate(&mut store, &component, &linker)?;

    Ok(instance.call_add(&mut store, number, 3)?)
}
