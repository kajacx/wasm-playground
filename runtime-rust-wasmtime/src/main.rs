use example::protocol::{companies, employees};
use std::{collections::HashMap, sync::Arc};
use wasmtime::{
    component::{Component, Linker, Resource},
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
struct MyEmployee {
    name: String,
    min_salary: u32,
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
    employees: ResHolder<MyEmployee>,
    world: Option<Arc<MyWorld>>,
}

impl MyWorldImports for State {
    fn can_employee_work_at(
        &mut self,
        employee: Resource<Employee>,
        company: Resource<Company>,
    ) -> Result<bool> {
        let company = self.companies.get(company.rep()).unwrap();
        let employee = self.employees.get(employee.rep()).unwrap();
        Ok(employee.min_salary <= company.max_salary)
    }
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

impl employees::HostEmployee for State {
    fn new(&mut self, name: String, min_salary: u32) -> Result<Resource<employees::Employee>> {
        Ok(Resource::new_own(
            self.employees.new(MyEmployee { name, min_salary }),
        ))
    }

    fn get_name(&mut self, self_: Resource<employees::Employee>) -> Result<String> {
        Ok(self.employees.get(self_.rep()).unwrap().name.clone())
    }

    fn get_min_salary(
        &mut self,
        self_: wasmtime::component::Resource<employees::Employee>,
    ) -> Result<u32> {
        Ok(self.employees.get(self_.rep()).unwrap().min_salary)
    }

    fn drop(&mut self, rep: wasmtime::component::Resource<employees::Employee>) -> Result<()> {
        self.employees.drop(rep.rep()).unwrap();
        Ok(())
    }
}

impl employees::Host for State {}

fn main() {
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
    let my_world = Arc::new(my_world);
    store.data_mut().world = Some(my_world.clone());

    let employees = my_world.example_protocol_employees().employee();
    let employee = employees
        .call_constructor(&mut store, "Mike".into(), 50_000)
        .unwrap();
    println!("What can this be used for? {employee:?}");
    println!(
        "Employee name: {}",
        employees.call_get_name(&mut store, employee).unwrap()
    );

    let employee = Resource::new_own(store.data_mut().employees.new(MyEmployee {
        name: "Mike".into(),
        min_salary: 50_000,
    }));

    let company1 = Resource::new_own(store.data_mut().companies.new(MyCompany {
        name: "Company1".into(),
        max_salary: 30_000,
    }));

    let result = my_world
        .call_find_job(
            &mut store,
            Resource::new_borrow(employee.rep()),
            &[company1],
        )
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

    let result = my_world
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
