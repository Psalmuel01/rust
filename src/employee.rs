struct Employee {
    name: String,
    salary: f64,
    id: u32,
    employee_type: EmployeeType,
}

#[derive(Debug)]
enum EmployeeType {
    JuniorEngineer,
    SeniorEngineer,
}

impl Employee {
    fn new(name: String, id: u32, employee_type: EmployeeType) -> Self {
        let salary = match employee_type {
            EmployeeType::JuniorEngineer => 50000.0,
            EmployeeType::SeniorEngineer => 60000.0,
        };
        Employee {
            name,
            salary,
            id,
            employee_type,
        }
    }

    fn add_salary(&mut self, salary: f64) {
        self.salary += salary;
    }
}

pub fn main() {
    let mut employee1 = Employee::new(String::from("John Doe"), 1001, EmployeeType::JuniorEngineer);
    println!("Initial salary: {}", employee1.salary);

    employee1.add_salary(1000.0);
    println!("Updated salary: {}", employee1.salary);

    let employee2 = Employee::new(String::from("Jane Doe"), 1002, EmployeeType::SeniorEngineer);
    println!("Employee 2 salary: {}", employee2.salary);
}
