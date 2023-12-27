
use wasmtime::{
    component::{Component, Linker},
    Config, Engine, Store,
};

use wasmtime_wasi::preview2::*;

wasmtime::component::bindgen!({
    path: "../guest-no-imports/wit/world.wit",
    world: "example",
    async: true,
});

struct State {
    table: Table,
    wasi: WasiCtx,
}

impl WasiView for State {
    fn table(&self) -> &Table {
        &self.table
    }
    fn table_mut(&mut self) -> &mut Table {
        &mut self.table
    }
    fn ctx(&self) -> &WasiCtx {
        &self.wasi
    }
    fn ctx_mut(&mut self) -> &mut WasiCtx {
        &mut self.wasi
    }
}

#[tokio::main]
async fn main()  {
    let mut config = Config::new();
    config.wasm_component_model(true);
    config.async_support(true);
    
    let table = Table::new();
    let wasi = WasiCtxBuilder::new().inherit_stdout().build();

    let engine = Engine::new(&config).unwrap();
    let mut store = Store::new(&engine, State { table, wasi });

    let component_bytes =
        std::fs::read("../guest-no-imports/target/wasm32-wasi/release/guest_no_imports.wasm")
            .expect("component bytes");

    let component = Component::new(&store.engine(), &component_bytes).expect("create component");

    let mut linker = Linker::new(store.engine());
    command::add_to_linker(&mut linker).unwrap();

    let (my_world, _instance) = Example::instantiate_async(&mut store, &component, &linker).await.unwrap();

    let result = my_world.call_hello_world(&mut store).await.unwrap();

    println!("Returned on host: {}", result);

}
