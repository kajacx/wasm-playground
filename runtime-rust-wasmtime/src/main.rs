use std::{collections::HashMap, error::Error};
use wasmtime::{
    component::{Component, Linker, Resource},
    Config, Engine, Result, Store,
};

wasmtime::component::bindgen!({
    path: "../protocol.wit",
    world: "my-world"
});

#[derive(Debug, Default, Clone)]
struct State {
    buildings: HashMap<u32, String>,
    next_id: u32,
}

impl HostBuilding for State {
    fn new(&mut self, name: String) -> Result<Resource<Building>> {
        let id = self.next_id;
        self.buildings.insert(id, name);
        self.next_id += 1;
        Ok(Resource::new_own(id))
    }

    fn get_name(&mut self, building: Resource<Building>) -> Result<String> {
        let building = self.buildings.get(&building.rep()).expect("get building");
        Ok(building.clone())
    }

    fn drop(&mut self, building: Resource<Building>) -> Result<()> {
        self.buildings
            .remove(&building.rep())
            .expect("remove building");
        Ok(())
    }
}

impl MyWorldImports for State {
    fn import_point(&mut self, mut point: Point) -> wasmtime::Result<Point> {
        point.x += 100;
        Ok(point)
    }
}

impl example::protocol::host_imports::Host for State {
    fn print_line(&mut self, msg: String) -> wasmtime::Result<()> {
        println!("Printing in host: {msg}");
        Ok(())
    }
}

impl inline_imports::Host for State {
    fn add_one(&mut self, num: i32) -> wasmtime::Result<i32> {
        Ok(num + 1)
    }
}

// cSpell::disable-next-line
impl singlewordimports::Host for State {
    fn sub_one(&mut self, num: i32) -> wasmtime::Result<i32> {
        Ok(num - 1)
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut config = Config::new();
    config.wasm_component_model(true);

    let engine = Engine::new(&config)?;
    let mut store = Store::new(&engine, State::default());

    let component_bytes =
        std::fs::read("../guest-rust/target/wasm32-unknown-unknown/release/guest_rust.wasm")
            .expect("component bytes");

    let component = Component::new(&store.engine(), &component_bytes).expect("create component");

    let mut linker = Linker::new(store.engine());

    MyWorld::add_to_linker(&mut linker, |state| state)?;
    let (my_world, _instance) = MyWorld::instantiate(&mut store, &component, &linker)?;

    my_world
        .example_protocol_guest_exports()
        .call_run(&mut store)?;

    println!(
        "Point: {:?}",
        my_world.call_move_point(&mut store, Point { x: 50, y: 50 })
    );

    let result = my_world.inline_exports().call_add_three(&mut store, 5)?;
    println!("5 + 3 = {result}");

    // cSpell::disable-next-line
    let result = my_world.singlewordexports().call_sub_three(&mut store, 5)?;
    println!("5 - 3 = {result}");

    let result = my_world.call_export_flags(&mut store)?;
    println!("flags: {result:?}");

    let result = my_world.call_export_many_flags(&mut store)?;
    println!(
        "many flags: {result:?}, {:?}",
        ManyFlags::F00 | ManyFlags::F01
    );

    let result = my_world.call_get_new_building(&mut store, "building name")?;
    println!("BUILDING: {result:?}");

    let result = my_world.call_get_buildings_name(&mut store, result)?;
    println!("BUILDING NAME: {result:?}");

    let result = my_world
        .example_protocol_guest_exports()
        .call_get_guest_resource(&mut store)?;
    println!("GR: {result:?}");

    let resource = my_world.example_protocol_guest_exports().guest_resource();

    let result = resource.call_get_name(&mut store, result)?;
    println!("GR name: {result:?}");

    Ok(())
}
