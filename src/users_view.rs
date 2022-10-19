use crate::*;

#[near_bindgen]
impl Contract {

    pub fn get_users (&self, offset:Option<usize>, limit:Option<usize>) -> Vec<User> {
        
        let u = self.users.values_as_vector()
        .iter()
        .skip(offset.unwrap_or(0))
        .take(limit.unwrap_or(10))
        .collect::<Vec<User>>();
        
        return u; 
    }


    pub fn get_user(&self, user_id : String ) -> Option<User>{

        return self.users.get(&user_id);
    }

    pub fn users_count(&self) -> usize {
        self.users.len() as usize 
    }


    pub fn has_user(&self, user_id : &String ) -> bool{
        self.users.get(&user_id).is_some()
    }


}