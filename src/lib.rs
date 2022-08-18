use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::Vector;
use near_sdk::{near_bindgen, env};

#[near_bindgen]
#[derive(Default, BorshDeserialize, BorshSerialize)]

//Capturing staff details
#[derive(BorshDeserialize, BorshSerialize, Debug)]
pub struct Staff {
    staff_id: String,
    name: String,
    department: String,
    roster:Vector<Roster>,
    
    
}




impl Default for Staff {
    fn default() -> Self{
        Staff { staff_id: "".to_string(), name: "".to_string(), department: "".to_string(), roster: Vector::new(b"r".to_vec()), }

    }
}


pub struct department{
    department_id: u32,
    department_name: String,
    role: String,
}

impl Default for department{
    fn default() -> Self {
        department{
            department_id: 0,
            department_name: "".to_string(),
            role: "".to_string(),
        }
    }
}

pub struct roster{
    date:String,
    attended: String,
}

#[near_bindgen]
impl Contract {
    #[init]
    pub fn new()-> Self{
        let staff:Vector<Staff> = Vector::new(b"r".to_vec());
        let departments:Vector<Department> = Vector::new(b"r".to_vec());


        Contract{staff,departments}
    }
    
}

//function to add new staff
pub fn add_new_staff(
    self:&mut self,
    staff_id: String,
    name: String,
    department: String,  
){
    let departments = &mut self.departments;
    let mut counter = 0;

    departments.iter() .for_each |department|{
        if department.department_name == staff_department{
            counter = counter+1;
        }
    };
    env::log_str(format!("===> Counter {}",counter).as_str());

 if counter > 0 {
    let staff1 = Staff {
        staff_id: staff_id.to_string(),
        name: name.to_string(),
        department: department.to_string(),
        roster: Vector::new(b"r".to_vec()),
        staff_id,
    };

    self.staff.push(&staff1);
    env::log_str("successfully added!");
 }else{
    env::log_str("error department not available!");
 }
}

pub fn get_staff_length(&self) -> u64{
    self.staff.len();
}

pub fn get_staff(&self, staff_id: String) -> Option<Staff>{
    let me:&Vector<Staff> = &self.staff;
    let todo = me.iter().find(|tod|tod.staff_id == staff_id);
    return todo;

}
//function to add a department

pub fn add_department(&mut self, department_name:String){
    let id = self.department.len() as u32;
    let new_department = Department{
        department_id: id,
        department_name: department_name.to_string(),
        role: Vector::new(b"r".to_vec()),
    };

    self.depatments.push(&new_department);
    env::log_str("department successfully added!");
}
    
pub fn get_department_length(&self)-> u64{
    self.departments.len()
}

//funtion to record staff roster
pub fn record_roster(&mut self, staff_id: String, attended:String) {
    let mut staff_count = 0;
    for (pos,mut staff) in self.staff.iter().enumerate(){
        print!("The staff at position {}: {:?}",pos,staff);
        if staff.staff_id == staff_id{
            staff_count = staff_count+1;

            let new_roster = Roster {
                date: date.to_string(),
                attended: attended.to_string(),
            };

            staff.roster.push(&new_roster);
            env::log_str("Successfully recorded!");

        }
    }

    if staff_count < 1{
        env::log_str("Invalid staff_id! \n Please try again:");
    }
}


    




#[cfg(test)]
mod tests {
    use super::*;
    use near_sdk::test_utils::{get_logs, VMContextBuilder};
   // use near_sdk::{testing_env, AccountId};

    #[test]
    fn add_department(){
        let mut department1 = Contract::new();
        department1.add_department("Radiology".to_string());
        department1.add_department("Admin".to_string());
        department1.add_department("Nursing Services".to_string());
        assert_eq!(department1.get_department_length(),3);

    }


    // TESTS HERE
}
 