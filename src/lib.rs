use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::Vector;
use near_sdk::{env, near_bindgen};

//Capturing staff details

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct Staff {
    staff_id: String,
    name: String,
    department: String,
    roster: Vector<Roster>,
}

impl Default for Staff {
    fn default() -> Self {
        Staff {
            staff_id: "".to_string(),
            name: "".to_string(),
            department: "".to_string(),
            roster: Vector::new(b"r".to_vec()),
        }
    }
}

#[near_bindgen]
#[derive(Default, BorshDeserialize, BorshSerialize)]
pub struct Department {
    department_id: u32,
    department_name: String,
    role: String,
}

#[near_bindgen]
#[derive(Default, BorshDeserialize, BorshSerialize)]
pub struct Roster {
    date: String,
    attended: String,
}

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct Data {
    staff: Vector<Staff>,
    departments: Vector<Department>,
}

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct App {
    staff: Vec<Staff>,
    departments: Vec<Department>,
}

impl Default for App {
    fn default() -> Self {
        App {
            departments: vec![], // Vector::new(b"r".to_vec()),
            staff: vec![],       //
        }
    }
}

#[near_bindgen]
impl App {
    // #[init]
    // pub fn new() -> Self {
    //     let staff: Vec<Staff> =  vec![];//Vector::new(b"r".to_vec());
    //     let departments: Vector<Department> = Vector::new(b"r".to_vec());

    //     App {
    //         staff: staff,
    //         departments: departments,
    //     }
    // }

    //function to add new staff
    pub fn add_new_staff(&mut self, staff_id: String, name: String, department: String) -> String {
        let departments = &mut self.departments;
        let mut counter = 0;

        departments.iter().for_each(|department_args| {
            if department_args.department_name == department {
                counter = counter + 1;
            }
        });
        env::log_str(format!("===> Counter {}", counter).as_str());

        return if counter > 0 {
            let staff1 = Staff {
                staff_id: staff_id.to_string(),
                name: name.to_string(),
                department: department.to_string(),
                roster: Vector::new(b"r".to_vec()),
            };

            self.staff.push(staff1);
            env::log_str("successfully added!");
            "okay".to_owned()
        } else {
            env::log_str("error department not available!");
            "error".to_owned()
        };
    }

    pub fn get_staff_length(&self) {
        self.staff.len();
    }

    pub fn get_staff(&self, staff_id: String) -> Option<&Staff> {
        // let me: &Vector<Staff> = &self.staff;
        // let todo = me.iter().find(|tod| tod.staff_id == staff_id);
        // return todo;
        let mut staff: Option<&Staff> = None;
        let mut index: Option<usize> = None;
        for (i, elem) in self.staff.iter().enumerate() {
            if elem.staff_id == staff_id {
                index = Some(i);
            }
        }
        match index {
            Some(k) => {
                staff = self.staff.get(k);
            }
            None => env::log_str("Woops index is 0"),
        }

        return staff;
    }
    //function to add a department

    pub fn add_department(&mut self, department_name: String) {
        let id = (self.departments.len() + 1) as u32;
        let new_department = Department {
            department_id: id,
            department_name: department_name.to_string(),
            role: "".to_owned(), //Vector::new(b"r".to_vec()),
        };

        self.departments.push(new_department);
        env::log_str("department successfully added!");
    }

    pub fn get_department_length(&self) -> u64 {
        self.departments.len() as u64
    }

    //funtion to record staff roster
    pub fn record_roster(&mut self, staff_id: String, attended: String) {
        let mut staff_count = 0;
        for (pos, staff) in self.staff.iter_mut().enumerate() {
            print!("The staff at position {}: {:?}", pos, staff.name);

            if staff.staff_id == staff_id {
                staff_count = staff_count + 1;

                let new_roster = Roster {
                    date: "date".to_string(),
                    attended: attended.to_string(),
                };

                staff.roster.push(&new_roster);
                env::log_str("Successfully recorded!");
            }
        }

        if staff_count < 1 {
            env::log_str("Invalid staff_id! \n Please try again:");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    //  use near_sdk::test_utils::{get_logs, VMContextBuilder};
    // use near_sdk::{testing_env, AccountId};

    #[test]
    fn add_department() {
        let mut department1 = App::default();
        department1.add_department("Radiology".to_string());
        department1.add_department("Admin".to_string());
        department1.add_department("Nursing Services".to_string());
        assert_eq!(department1.get_department_length(), 3);
    }

    #[test]
    fn get_staff() {
        let mut app = App::default();
        app.add_department("Radiology".to_string());

        let result = app.add_new_staff(
            "123".to_string(),
            "kenn".to_string(),
            "Radiology".to_string(),
        );

        assert_eq!(result, "okay");
        assert_eq!(app.staff.len(), 1);

        let rs = app.get_staff("123".to_string());

        match rs {
            Some(k) => {
                assert_eq!(k.name, "kenn".to_string())
            }
            None => {
                panic!("staff should not be none")
            }
        }
    }
#[test]
#[should_panic]
fn record_roster(){

    let mut department1 = App::default();
    department1.add_department("Radiology".to_string());
    

    department1.add_new_staff(
        "123".to_string(),
        "kenn".to_string(),
        "Radiology".to_string(), 
    );

    department1.record_roster(
        "123".to_string(),
            "kenn".to_string(),
            // "Radiology".to_string(),
    );

    match department1.staff.get(0) {
        Some(v)=>{
          assert_eq!(v.roster.len(), 1);  
        }
        None =>{
            panic!("Staff not found!")
        } 
    };
}

}
