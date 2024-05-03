wit_bindgen::generate!({
    path: "../protocol.wit",
    world: "resources",
    with: {
        // Throws error: unused mapping
        //"example:protocol/employees/employee": MyEmployee
    }
});

use example::protocol::companies::Company;

// This has wrong function signature, but can call host
// use example::protocol::employees::Employee;

// This works for the function signature, but cannot call host
use exports::example::protocol::employees::Employee;

struct MyEmployees;

impl exports::example::protocol::employees::Guest for MyEmployees {
    type Employee = MyEmployee;
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

impl exports::example::protocol::guest_fns::Guest for MyEmployees {
    fn company_roundtrip(company: Company) -> Company {
        let _name = company.get_name();
        example::protocol::host_fns::company_roundtrip(company)
    }

    // Host call wants example::protocol::employees::Employee
    // But this function signature wants
    fn employee_roundtrip(employee: Employee) -> Employee {
        #[allow(unused, unreachable_code)]
        if false {
            let mut employee: example::protocol::employees::Employee = todo!();
            let _name = employee.get_name();
            employee = example::protocol::host_fns::employee_roundtrip(employee);
        }

        let _name = &employee.get::<MyEmployee>().name;
        employee

        // TODO: this unfortunately doesn't work
        // example::protocol::host_fns::employee_roundtrip(employee)
    }
}

export!(MyEmployees);
