wit_bindgen::generate!({
    path: "../protocol.wit",
    world: "my-world",
});

struct MyEmployees;

impl exports::example::protocol::employees::Guest for MyEmployees {
    type Employee = MyEmployee;

    fn find_job(
        employee: exports::example::protocol::employees::EmployeeBorrow,
        companies: wit_bindgen::rt::vec::Vec<exports::example::protocol::employees::Company>,
    ) -> Option<exports::example::protocol::employees::Company> {
        companies
            .into_iter()
            .find(|company| employee.get::<MyEmployee>().min_salary <= company.get_max_salary())
    }
}

pub struct MyEmployee {
    name: String,
    min_salary: u32,
}

impl exports::example::protocol::employees::GuestEmployee for MyEmployee {
    fn new(name: String, min_salary: u32) -> Self {
        Self { name, min_salary }
    }

    fn get_name(&self) -> String {
        self.name.clone()
    }

    fn get_min_salary(&self) -> u32 {
        self.min_salary
    }
}

export!(MyEmployees);
