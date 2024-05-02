use example::protocol::{
    companies,
    employees::{self, HostEmployee},
};
use std::collections::HashMap;
use wasmtime::{
    component::{Component, Linker, Resource, ResourceAny},
    Config, Engine, Result, Store,
};

wasmtime::component::bindgen!({
    path: "../protocol.wit",
    world: "my-world"
});

#[derive(Default, Clone)]
struct MyCompany {
    name: String,
    max_salary: u32,
}

#[derive(Default, Clone)]
struct ResHolder<T> {
    resources: HashMap<u32, T>,
    next_id: u32,
}

impl<T> ResHolder<T> {
    fn new(&mut self, item: T) -> u32 {
        let id = self.next_id;
        self.resources.insert(id, item);
        self.next_id += 1;
        id
    }

    fn get(&self, id: u32) -> Option<&T> {
        self.resources.get(&id)
    }

    fn drop(&mut self, id: u32) -> Result<(), ()> {
        self.resources.remove(&id).map(|_| ()).ok_or(())
    }
}

#[derive(Default, Clone)]
struct State {
    companies: ResHolder<MyCompany>,
}

impl companies::HostCompany for State {
    fn new(&mut self, name: String, max_salary: u32) -> Result<Resource<companies::Company>> {
        Ok(Resource::new_own(
            self.companies.new(MyCompany { name, max_salary }),
        ))
    }

    fn get_name(&mut self, self_: Resource<companies::Company>) -> Result<String> {
        Ok(self.companies.get(self_.rep()).unwrap().name.clone())
    }

    fn get_max_salary(&mut self, self_: Resource<companies::Company>) -> Result<u32> {
        Ok(self.companies.get(self_.rep()).unwrap().max_salary)
    }

    fn drop(&mut self, rep: Resource<companies::Company>) -> Result<()> {
        self.companies.drop(rep.rep()).unwrap();
        Ok(())
    }
}

impl companies::Host for State {}

impl HostEmployee for State {
    fn drop(
        &mut self,
        rep: wasmtime::component::Resource<employees::Employee>,
    ) -> wasmtime::Result<()> {
        Ok(())
    }

    fn get_min_salary(
        &mut self,
        self_: wasmtime::component::Resource<employees::Employee>,
    ) -> wasmtime::Result<u32> {
        Ok(15_000)
    }

    fn get_name(
        &mut self,
        self_: wasmtime::component::Resource<employees::Employee>,
    ) -> wasmtime::Result<String> {
        Ok("Anon".into())
    }

    fn new(
        &mut self,
        name: String,
        min_salary: u32,
    ) -> wasmtime::Result<wasmtime::component::Resource<employees::Employee>> {
        Ok(Resource::new_own(0))
    }
}

impl employees::Host for State {}

impl MyWorldImports for State {
    fn company_roundtrip_import(
        &mut self,
        company: wasmtime::component::Resource<Company>,
    ) -> wasmtime::Result<wasmtime::component::Resource<Company>> {
        let name = &self.companies.get(company.rep()).unwrap().name;
        println!("Company roundtrip: {name}");
        Ok(company)
    }

    fn employee_roundtrip_import(
        &mut self,
        employee: wasmtime::component::Resource<Employee>,
    ) -> wasmtime::Result<wasmtime::component::Resource<Employee>> {
        println!("Employee name: ???");
        Ok(employee)
    }
}

fn main() {
    println!("Starting...");

    let mut config = Config::new();
    config.wasm_component_model(true);

    let engine = Engine::new(&config).unwrap();
    let mut store = Store::new(&engine, State::default());

    let component_bytes =
        std::fs::read("../guest-rust/target/wasm32-unknown-unknown/release/guest_rust.wasm")
            .expect("component bytes");

    let component = Component::new(&store.engine(), &component_bytes).expect("create component");

    let mut linker = Linker::new(store.engine());

    MyWorld::add_to_linker(&mut linker, |state| state).unwrap();
    let (my_world, _instance) = MyWorld::instantiate(&mut store, &component, &linker).unwrap();

    let employees = my_world.example_protocol_employees().employee();
    let employee: ResourceAny = employees
        .call_constructor(&mut store, "Mike".into(), 50_000)
        .unwrap();
    println!(
        "Employee name: {}",
        employees.call_get_name(&mut store, employee).unwrap()
    );

    let company1 = Resource::new_own(store.data_mut().companies.new(MyCompany {
        name: "Company1".into(),
        max_salary: 30_000,
    }));

    let employee = Resource::new_own(0);

    let result = my_world
        // .example_protocol_employees()
        .call_find_job(&mut store, employee, &[company1])
        .unwrap();
    println!("Find first job: {result:?}");

    let company1 = Resource::new_own(store.data_mut().companies.new(MyCompany {
        name: "Company1".into(),
        max_salary: 30_000,
    }));
    let company2 = Resource::new_own(store.data_mut().companies.new(MyCompany {
        name: "Company2".into(),
        max_salary: 60_000,
    }));

    let employee = Resource::new_own(0);

    let result = my_world
        // .example_protocol_employees()
        .call_find_job(&mut store, employee, &[company1, company2])
        .unwrap();
    println!("Find second job: {result:?}");
    println!(
        "Second job name: {}",
        store
            .data()
            .companies
            .get(result.unwrap().rep())
            .unwrap()
            .name
    );
}
