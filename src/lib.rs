use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::{LookupMap, Vector};
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::{env, near_bindgen, AccountId};
 
//Capturing staff details
 
#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Debug)]
#[serde(crate = "near_sdk::serde")]
 
pub struct Staff {
    account: AccountId,
    staff_id: String,
    name: String,
    department: String,
}
 
#[near_bindgen]
#[derive(Default, BorshDeserialize, BorshSerialize, Serialize, Deserialize, Clone, Debug)]
#[serde(crate = "near_sdk::serde")]
pub struct Department {
    department_id: u32,
    department_name: String,
    role: String,
}
 
#[near_bindgen]
#[derive(Default, BorshDeserialize, BorshSerialize, Serialize, Deserialize, Clone, Debug)]
#[serde(crate = "near_sdk::serde")]
pub struct Roster {
    date: u64,
    attended: String,
    staff_id: String,
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
    staff: Vector<Staff>,
    roster: LookupMap<AccountId, Vector<Roster>>,
    departments: Vector<Department>,
}
 
impl Default for App {
    fn default() -> Self {
        App {
            departments: Vector::new(b"r".to_vec()),
            staff: Vector::new(b"r".to_vec()),
            roster: LookupMap::new(b"r".to_vec()),
        }
    }
}
 
#[near_bindgen]
impl App {
    //function to add new staff
    pub fn add_new_staff(&mut self, name: String, department: String) -> String {
        let departments = &mut self.departments;
        let mut counter = 0;
 
        departments.iter().for_each(|department_args| {
            if department_args.department_name == department {
                counter = counter + 1;
            }
        });
        env::log_str(format!("===> Counter {}", counter).as_str());
 
        let id = &env::random_seed();
        let s = format!("{:?}", id);
        return if counter > 0 {
            let staff1 = Staff {
                account: env::current_account_id(),
                staff_id: s,
                name: name.to_string(),
                department: department.to_string(),
            };
 
            self.staff.push(&staff1);
            env::log_str("successfully added!");
 
            format!("{:?}", id)
        } else {
            env::log_str("error department not available!");
            "error".to_owned()
        };
    }
 
    pub fn get_staff_length(&self) {
        self.staff.len();
    }
 
    pub fn get_staff(&self, staff_id: String) -> Option<Staff> {
        // return self.staff.ge
        // let mut staff: Option<&Staff> = None;
        let mut index: Option<usize> = None;
        for (i, elem) in self.staff.iter().enumerate() {
            if elem.staff_id == staff_id {
                index = Some(i);
            }
        }
        match index {
            Some(k) => {
                let size = k as u64;
                self.staff.get(size)
            }
            None => {
                env::log_str("Woops index is 0");
                None
            }
        }
    }
    //function to add a department
 
    pub fn add_department(&mut self, department_name: String) {
        let id = (self.departments.len() + 1) as u32;
        let new_department = Department {
            department_id: id,
            department_name: department_name.to_string(),
            role: "".to_owned(),
        };
 
        self.departments.push(&new_department);
        env::log_str("department successfully added!");
    }
 
    pub fn get_department_length(&self) -> u64 {
        self.departments.len() as u64
    }
 
    //funtion to record staff roster
    pub fn record_roster(&mut self, staff_id: String, attended: String) {
        // get roaster for a staff
 
        let exist_roaster = self.roster.get(&env::current_account_id());
 
        let new_roster = Roster {
            date: env::block_timestamp(),
            staff_id: staff_id,
            attended: attended.to_string(),
        };
 
        match exist_roaster {
            Some(mut k) => {
                k.push(&new_roster);
 
                self.roster.insert(&env::current_account_id(), &k);
            }
            None => {
                let mut tmp = Vector::new(b"r".to_vec());
                tmp.push(&new_roster);
                self.roster.insert(&env::current_account_id(), &tmp);
            }
        }
    }
}
 
#[cfg(test)]
mod tests {
    use super::*;
    use near_sdk::test_utils::{ VMContextBuilder};
    use near_sdk::{ testing_env,AccountId};
 
 
    fn get_context(predecessor: AccountId) -> VMContextBuilder {
        let mut builder = VMContextBuilder::new();
        builder.predecessor_account_id(predecessor);
        builder
    }
 
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
 
        let elvis = AccountId::new_unchecked("elvis.testnet".to_string());
        let context = get_context(elvis.clone());
        testing_env!(context.build());
        let mut app = App::default();
        app.add_department("Radiology".to_string());
 
        let result = app.add_new_staff(
            "elvis".to_string(),
            "Radiology".to_string(),
            
        );
 
        assert_ne!(result, "error".to_owned());
        assert_eq!(app.staff.len(), 1);
 
        
    }
    #[test]
    #[should_panic]
    fn record_roster() {
 
        let elvis = AccountId::new_unchecked("elvis.testnet".to_string());
        let context = get_context(elvis.clone());
 
        testing_env!(context.build());
        let mut department1 = App::default();
        department1.add_department("Radiology".to_string());
 
       let staff_id = department1.add_new_staff(
            "elvis".to_string(),
            "Radiology".to_string(),
        );
 
        assert_ne!(staff_id, "error".to_owned());
 
        department1.record_roster(
            staff_id,
            "kenn".to_string(),
            
        );
 
        match department1.roster.get(&elvis){
            Some(v) => {
                assert_eq!(v.len(), 1);
 
            }
            None => {
                env::panic_str("Staff not found!")
            }
        };
    }
}