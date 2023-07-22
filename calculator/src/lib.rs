use wasm_bridge::component::*;
use wasm_bridge::{Config, Engine, Result, Store};

static COMPONENT_BYTES: &'static [u8] =
    include_bytes!("../../target/wasm32-unknown-unknown/debug/guest.wasm");

wasm_bridge::component::bindgen!({
    path: "../protocol.wit",
    world: "calculator",
});

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
    let mut store = Store::new(&engine, ());

    let component = Component::new(&store.engine(), COMPONENT_BYTES)?;

    let linker = Linker::new(store.engine());

    let (instance, _) = Calculator::instantiate(&mut store, &component, &linker)?;

    Ok(instance.call_add_three(&mut store, number)?)
}
