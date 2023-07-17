use std::error::Error;
use wasmtime::{
    component::{Component, Linker},
    Config, Engine, Result, Store,
};

use wasmtime_wasi::preview2::*;

wasmtime::component::bindgen!({
    path: "../protocol.wit",
    world: "my-world",
    async: true,
    with: {
       "wasi:cli-base/stdin": wasi::cli_base::stdin,
       "wasi:cli-base/stdout": wasi::cli_base::stdout,
       "wasi:cli-base/stderr": wasi::cli_base::stderr,
    }
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

#[async_trait::async_trait]
impl MyWorldImports for State {
    async fn import_point(&mut self, mut point: Point) -> Result<Point> {
        point.x += 100;
        Ok(point)
    }

    async fn print(&mut self, msg: String) -> Result<()> {
        println!("From sys host: {msg}");
        Ok(())
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let mut config = Config::new();
    config.wasm_component_model(true);
    config.async_support(true);

    let mut table = Table::new();
    let wasi = WasiCtxBuilder::new().inherit_stdio().build(&mut table)?;

    let engine = Engine::new(&config)?;
    let mut store = Store::new(&engine, State { table, wasi });

    let component_bytes =
        std::fs::read("../plugin-wasi/target/wasm32-wasi/release/plugin_wasi.wasm")
            .expect("component bytes");

    let component = Component::new(&store.engine(), &component_bytes).expect("create component");

    let mut linker = Linker::new(store.engine());

    // do NOT use these ...
    // wasi::cli_base::stdin::add_to_linker(&mut linker, |data| data)?;
    // wasi::cli_base::stdout::add_to_linker(&mut linker, |data| data)?;
    // wasi::cli_base::stderr::add_to_linker(&mut linker, |data| data)?;

    // Use this instead
    wasi::command::add_to_linker(&mut linker)?;

    MyWorld::add_to_linker(&mut linker, |state| state)?;
    let (my_world, _instance) = MyWorld::instantiate_async(&mut store, &component, &linker).await?;

    println!(
        "Point: {:?}",
        my_world
            .call_move_point(&mut store, Point { x: 50, y: 50 })
            .await
    );

    my_world.call_say_hello(&mut store).await?;

    Ok(())
}
