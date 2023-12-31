use example::protocol::companies;

wit_bindgen::generate!({
    path: "../protocol.wit",
    world: "my-world",
    exports: {
        world: MyGuest,
        "example:protocol/employees/employee": MyEmployee,
    }
});

pub struct MyEmployee(String, u32);

impl exports::example::protocol::employees::GuestEmployee for MyEmployee {
    fn new(name: String, min_salary: u32) -> Self {
        Self(name, min_salary)
    }

    fn get_name(&self) -> String {
        self.0.clone()
    }

    fn get_min_salary(&self) -> u32 {
        self.1
    }
}

struct MyGuest;

impl Guest for MyGuest {
    fn find_job(
        _employee: &Employee,
        _companies: Vec<&companies::Company>,
    ) -> Option<companies::Company> {
        Option::None
    }
}
