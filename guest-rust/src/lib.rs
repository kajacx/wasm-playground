wit_bindgen::generate!({
    path: "../protocol.wit",
    world: "my-world",
});

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

impl Guest for MyEmployees {
    fn find_job(employee: Employee, companies: Vec<Company>) -> Option<Company> {
        companies
            .into_iter()
            .find(|company| employee.get_min_salary() <= company.get_max_salary())
    }

    fn find_employee(company: Company, employees: Vec<Employee>) -> Option<Employee> {
        employees
            .into_iter()
            .find(|employee| employee.get_min_salary() <= company.get_max_salary())
    }

    fn company_roundtrip_export(company: Company) -> Company {
        company_roundtrip_import(company)
    }

    fn employee_roundtrip_export(employee: Employee) -> Employee {
        employee_roundtrip_import(employee)
    }
}

export!(MyEmployees);
