use wasmtime::{
    component::{Component, Linker, Resource, ResourceAny, ResourceTable},
    Config, Engine, Result, Store,
};

wasmtime::component::bindgen!({
    path: "../protocol.wit",
    world: "resources",
    with: {
        "example:protocol/companies/company": MyCompany
    }
});

use example::protocol::employees::Employee;

#[derive(Default, Clone)]
pub struct MyCompany {
    name: String,
    max_salary: u32,
}

#[derive(Default)]
struct State {
    resources: ResourceTable,
}

impl example::protocol::companies::HostCompany for State {
    fn new(&mut self, name: String, max_salary: u32) -> Result<Resource<MyCompany>> {
        Ok(self.resources.push(MyCompany { name, max_salary })?)
    }

    fn get_name(&mut self, self_: Resource<MyCompany>) -> Result<String> {
        Ok(self.resources.get(&self_)?.name.clone())
    }

    fn get_max_salary(&mut self, self_: Resource<MyCompany>) -> Result<u32> {
        Ok(self.resources.get(&self_)?.max_salary)
    }

    fn drop(&mut self, rep: Resource<MyCompany>) -> Result<()> {
        self.resources.delete(rep)?;
        Ok(())
    }
}

impl example::protocol::host_fns::Host for State {
    fn company_roundtrip(&mut self, company: Resource<MyCompany>) -> Result<Resource<MyCompany>> {
        println!("Name: {}", self.resources.get(&company).unwrap().name);
        Ok(company)
    }

    fn employee_roundtrip(&mut self, employee: Resource<Employee>) -> Result<Resource<Employee>> {
        // TODO: how to get employee name here?
        // println!("Name: {}", self.resources.get(&employee).unwrap().???);
        Ok(employee)
    }
}

impl example::protocol::companies::Host for State {}

impl example::protocol::employees::HostEmployee for State {
    fn new(
        &mut self,
        _name: String,
        _min_salary: u32,
    ) -> wasmtime::Result<wasmtime::component::Resource<Employee>> {
        todo!()
    }

    fn get_name(
        &mut self,
        _self_: wasmtime::component::Resource<Employee>,
    ) -> wasmtime::Result<String> {
        todo!()
    }

    fn get_min_salary(
        &mut self,
        _self_: wasmtime::component::Resource<Employee>,
    ) -> wasmtime::Result<u32> {
        todo!()
    }

    fn drop(&mut self, _rep: wasmtime::component::Resource<Employee>) -> wasmtime::Result<()> {
        todo!()
    }
}

impl example::protocol::employees::Host for State {}

fn main() {
    println!("Starting...");

    let mut config = Config::new();
    config.wasm_component_model(true);

    let engine = Engine::new(&config).unwrap();
    let mut store = Store::new(&engine, State::default());

    let component_bytes =
        std::fs::read("../guest/target/wasm32-unknown-unknown/debug/guest.wasm").unwrap();

    let component = Component::new(&store.engine(), &component_bytes).unwrap();

    let mut linker = Linker::new(store.engine());

    Resources::add_to_linker(&mut linker, |state| state).unwrap();
    let (my_world, _instance) = Resources::instantiate(&mut store, &component, &linker).unwrap();

    let employees = my_world.example_protocol_employees().employee();
    let employee: ResourceAny = employees
        .call_constructor(&mut store, "Mike".into(), 50_000)
        .unwrap();
    println!(
        "Employee name: {}",
        employees.call_get_name(&mut store, employee).unwrap()
    );

    let result = my_world
        .example_protocol_guest_fns()
        .call_employee_roundtrip(&mut store, employee)
        .unwrap();
    println!(
        "Employee name after roundtrip: {}",
        employees.call_get_name(&mut store, result).unwrap()
    );

    let company = store
        .data_mut()
        .resources
        .push(MyCompany {
            name: "Company1".into(),
            max_salary: 30_000,
        })
        .unwrap();
    println!(
        "Company name: {}",
        store.data().resources.get(&company).unwrap().name
    );

    let result = my_world
        .example_protocol_guest_fns()
        .call_company_roundtrip(&mut store, company)
        .unwrap();
    println!(
        "Company name after roundabout: {}",
        store.data().resources.get(&result).unwrap().name
    );

    // "Must" manually drop returned company, dropping the Resource handle does not delete the company from the table
    store.data_mut().resources.delete(result).unwrap();

    println!("All done.")
}
