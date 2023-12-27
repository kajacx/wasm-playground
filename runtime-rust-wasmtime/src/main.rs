use bytes::Bytes;
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

impl example::protocol::types::Host for State {}

#[derive(Debug, Clone)]
struct OutStream(Arc<Mutex<Vec<u8>>>);

#[async_trait::async_trait]
impl Subscribe for OutStream {
    async fn ready(&mut self) {}
}

impl HostOutputStream for OutStream {
    fn write(&mut self, buf: bytes::Bytes) -> StreamResult<()> {
        let len = buf.len();
        self.0.try_lock().unwrap().extend(buf);
        StreamResult::Ok(())
    }

    fn flush(&mut self) -> StreamResult<()> {
        StreamResult::Ok(())
    }

    fn check_write(&mut self) -> StreamResult<usize> {
        StreamResult::Ok(usize::MAX)
    }
}

impl StdoutStream for OutStream {
    fn stream(&self) -> Box<(dyn wasmtime_wasi::preview2::HostOutputStream + 'static)> {
        Box::new((*self).clone()) // TODO: this is probably wrong
    }

    fn isatty(&self) -> bool {
        false
    }
}

#[derive(Debug, Clone)]
struct InStream(Vec<u8>, usize);

#[async_trait::async_trait]
impl Subscribe for InStream {
    async fn ready(&mut self) {}
}

// NOT NEEDED?
impl HostInputStream for InStream {
    fn read(&mut self, size: usize) -> StreamResult<Bytes> {
        let start = self.1;
        let len = size.min(self.0.len() - self.1);
        self.1 += len as usize;
        if size > 0 && len == 0 {
            StreamResult::Err(StreamError::Closed)
        } else {
            StreamResult::Ok(Bytes::copy_from_slice(&self.0[start..start + len]))
        }
    }
}

impl StdinStream for InStream {
    fn stream(&self) -> Box<(dyn wasmtime_wasi::preview2::HostInputStream + 'static)> {
        Box::new((*self).clone()) // TODO: this is probably wrong
    }

    fn isatty(&self) -> bool {
        false
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

    let table = Table::new();
    let wasi = WasiCtxBuilder::new()
        // .stdout(out)
        // .stdin(in_)
        // .wall_clock(FakeClock)
        // .secure_random(random)
        // .set_monotonic_clock(FakeClock)
        // .set_secure_random_to_custom_generator(random)
        // .set_
        // .inherit_stdin()
        .build();

    let engine = Engine::new(&config)?;
    let mut store = Store::new(&engine, State { table, wasi });

    let component_bytes =
        std::fs::read("../plugin-wasi/target/wasm32-wasi/release/plugin_wasi.wasm")
            //std::fs::read("../plugin-wasi/target/wasm32-wasi/release/wasi_components_guest.wasm")
            .expect("component bytes");

    let component = Component::new(&store.engine(), &component_bytes).expect("create component");

    let mut linker = Linker::new(store.engine());

    // linker.

    // do NOT use these ...
    // wasi::cli_base::stdin::add_to_linker(&mut linker, |data| data)?;
    // wasi::cli_base::stdout::add_to_linker(&mut linker, |data| data)?;
    // wasi::cli_base::stderr::add_to_linker(&mut linker, |data| data)?;

    // Use this instead
    // command::add_to_linker(&mut linker)?;
    command::add_to_linker(&mut linker)?;

    MyWorld::add_to_linker(&mut linker, |state| state)?;
    let (my_world, _instance) = MyWorld::instantiate_async(&mut store, &component, &linker).await?;

    my_world.call_say_hello(&mut store).await?;

    println!(
        "BYTES ss: {:?}",
        std::str::from_utf8(&*out_bytes.try_lock().unwrap())
    );

    println!("LINE: {:?}", my_world.call_read_line(&mut store).await?);
    println!("LINE: {:?}", my_world.call_read_line(&mut store).await?);
    println!("LINE: {:?}", my_world.call_read_line(&mut store).await?);

    println!("5 + 3 = {:?}", my_world.call_add_three(&mut store, 5).await);

    Ok(())
}
