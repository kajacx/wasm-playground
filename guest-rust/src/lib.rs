use example::protocol::companies;

wit_bindgen::generate!({
    path: "../protocol.wit",
    world: "my-world",
    exports: {
        world: MyGuest,
        "example:protocol/employees/employee": MyEmployee,
    }
});

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

struct MyGuest;

impl Guest for MyGuest {
    fn find_job(
        employee: &Employee,
        companies: Vec<companies::Company>,
    ) -> Option<companies::Company> {
        companies
            .into_iter()
            .find(|company| can_employee_work_at(employee, &company))
    }
}
