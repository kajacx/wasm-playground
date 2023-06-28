use std::error::Error;
use wasmtime::{
    component::{Component, Linker},
    Config, Engine, Store,
};

wasmtime::component::bindgen!({
    path: "../protocol.wit",
    world: "my-world"
});

#[derive(Debug, Default, Clone)]
struct State {}

impl MyWorldImports for State {
    fn print(&mut self, msg: String) -> wasmtime::Result<()> {
        println!("Printing in host: {msg}");
        Ok(())
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut config = Config::new();
    config.wasm_component_model(true);

    let engine = Engine::new(&config)?;
    let mut store = Store::new(&engine, State::default());

    // let bytes =
    //     // std::fs::read("../plugin-rust/target/wasm32-wasi/debug/plugin_rust.wasm").expect("bytres");
    // std::fs::read("../plugin-rust/target/wasm32-unknown-unknown/debug/plugin_rust.wasm").expect("bytres");
    // // let adapter_bytes =
    // //     std::fs::read("../plugin-rust/wasi_snapshot_preview1.wasm").expect("adapter");

    // let component_bytes = ComponentEncoder::default()
    //     .module(&bytes)?
    //     .validate(true)
    //     // .adapter("wasm_to_component", &adapter_bytes)?
    //     .encode()?;

    let component_bytes =
        std::fs::read("../plugin-rust/target/wasm32-unknown-unknown/debug/component.wasm")
            .expect("component bytes");

    let component = Component::new(&store.engine(), &component_bytes).expect("create component");

    let mut linker = Linker::new(store.engine());

    MyWorld::add_to_linker(&mut linker, |state| state)?;
    let (my_world, _instance) = MyWorld::instantiate(&mut store, &component, &linker)?;

    my_world.call_run(&mut store)?;

    Ok(())
}
