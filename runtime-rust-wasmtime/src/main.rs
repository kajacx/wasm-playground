use std::{
    error::Error,
    sync::{Arc, Mutex},
};
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

impl example::protocol::types::Host for State {}

struct OutStream(Arc<Mutex<Vec<u8>>>);

#[async_trait::async_trait]
impl OutputStream for OutStream {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    async fn writable(&self) -> Result<()> {
        Ok(())
    }

    async fn write(&mut self, buf: &[u8]) -> Result<u64> {
        self.0.try_lock().unwrap().extend(buf);
        Ok(buf.len() as u64)
    }
}

struct InStream(Vec<u8>, usize);

#[async_trait::async_trait]
impl InputStream for InStream {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    async fn readable(&self) -> Result<()> {
        Ok(())
    }

    async fn read(&mut self, buf: &mut [u8]) -> Result<(u64, bool)> {
        let len = buf.len().min(self.0.len() - self.1);
        (&mut buf[..len]).copy_from_slice(&self.0[self.1..(len + self.1)]);
        self.1 += len as usize;
        Ok((len as _, self.1 == self.0.len()))
    }

    async fn num_ready_bytes(&self) -> Result<u64> {
        Ok((self.0.len() - self.1) as _)
    }
}

struct FakeClock;

impl HostWallClock for FakeClock {
    fn now(&self) -> std::time::Duration {
        std::time::Duration::from_secs(0)
    }

    fn resolution(&self) -> std::time::Duration {
        std::time::Duration::from_nanos(1)
    }
}

impl HostMonotonicClock for FakeClock {
    fn now(&self) -> u64 {
        0
    }
    fn resolution(&self) -> u64 {
        1
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let mut config = Config::new();
    config.wasm_component_model(true);
    config.async_support(true);

    let out_bytes = Arc::new(Mutex::new(Vec::<u8>::new()));
    let out = OutStream(out_bytes.clone());

    let in_ = InStream("Hello world!\n".to_string().into_bytes(), 0);

    let mut table = Table::new();
    let wasi = WasiCtxBuilder::new()
        .set_stdout(out)
        .set_stdin(in_)
        .set_wall_clock(FakeClock)
        // .set_monotonic_clock(FakeClock)
        // .set_secure_random_to_custom_generator(random)
        // .set_
        // .inherit_stdin()
        .build(&mut table)?;

    let engine = Engine::new(&config)?;
    let mut store = Store::new(&engine, State { table, wasi });

    let component_bytes =
        std::fs::read("../plugin-wasi/target/wasm32-wasi/release/plugin_wasi.wasm")
            .expect("component bytes");

    let component = Component::new(&store.engine(), &component_bytes).expect("create component");

    let mut linker = Linker::new(store.engine());

    // linker.

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

    println!(
        "BYTES: {:?}",
        std::str::from_utf8(&*out_bytes.try_lock().unwrap())
    );

    println!("LINE: {:?}", my_world.call_read_line(&mut store).await?);
    println!("LINE: {:?}", my_world.call_read_line(&mut store).await?);
    println!("LINE: {:?}", my_world.call_read_line(&mut store).await?);

    Ok(())
}
