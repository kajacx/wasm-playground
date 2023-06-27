use std::error::Error;
use wasmtime::component::*;

wasmtime::component::bindgen!("my-world");

#[derive(Debug, Default, Clone)]
struct State {}

impl MyWorldImports for State {
    fn print(&mut self, msg: String) -> wasmtime::Result<()> {
        println!("Printing in host: {msg}");
        Ok(())
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut store = wasmtime::Store::<State>::default();

    let component = Component::new(&store.engine(), &[])?;

    let mut linker = Linker::new(store.engine());

    MyWorld::add_to_linker(&mut linker, |state| state)?;
    let (my_world, _instance) = MyWorld::instantiate(&mut store, &component, &linker)?;

    my_world.call_run(&mut store)?;

    Ok(())
}
