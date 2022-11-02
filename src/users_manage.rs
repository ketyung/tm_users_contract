use crate::*;

// Implement the contract structure
#[near_bindgen]
impl Contract {
    fn check_if_authorized() {

        if env::current_account_id() != env::signer_account_id() {
            env::panic_str("Unauthorized!");
        }
    }
}

// Implement the contract structure
#[near_bindgen]
impl Contract {

    
    pub fn signup_user (&mut self, first_name : String, last_name : String,
    profile_image : Option<String>, email : Option<String>, mobile_number : Option<String> ){
        self.add_user(env::signer_account_id().to_string(), first_name, last_name, profile_image, email,mobile_number);
    }

    pub(crate) fn add_user (&mut self, id : String,  first_name : String, last_name : String, 
        profile_image : Option<String>,
        email : Option<String>,
        mobile_number : Option<String> ){

        if self.users.get(&id).is_some() {
            env::panic_str(format!("User {} already exists",id).as_str());
        }    

       // env::log_str(format!("profile_img:{:?}", profile_image).as_str());

        env::log_str(format!("storage.usage.b4:{}", env::storage_usage()).as_str());


      
        let date = env::block_timestamp();
        let user = User {
            id :id.clone(),
            first_name : Some(first_name),
            last_name : Some(last_name),
            email : email,
            mobile_number : mobile_number,
            profile_image : profile_image, 
            date_created : Some(date),
            date_updated : Some(date), 
        };


        //println!("sizeofuser:{}", std::mem::size_of::<User>());

        self.users.insert(&id, &user);

        env::log_str(format!("storage.usage.af:{}", env::storage_usage()).as_str());
        
    }


}


#[near_bindgen]
impl Contract {


    pub fn update_user (&mut self, first_name : Option<String>, last_name : Option<String>,
        profile_image : Option<String>, email : Option<String>, mobile_number : Option<String>) {

        let id = env::signer_account_id().to_string();

        let user = self.users.get(&id); 

        if user.is_some () {

            let mut wuser  = user.unwrap();
            wuser.id = id.clone(); 
            // only update field if they're not none (undefined from the client)
            if first_name.is_some() {
                wuser.first_name = first_name;
            }

            if last_name.is_some(){
                wuser.last_name = last_name;
            }

            if email.is_some() {
                wuser.email = email;
            }

            if mobile_number.is_some() {
                wuser.mobile_number = mobile_number;
            }

            wuser.profile_image = profile_image;
            
            wuser.date_updated = Some(env::block_timestamp());
            self.users.remove(&id);
            self.users.insert(&id, &wuser);

        }   
        else {
            env::panic_str(format!("User {} not found", id).as_str());
        } 
    }

}


#[near_bindgen]
impl Contract{

    pub fn remove_user (&mut self, user_id : String) {

        Self::check_if_authorized();

        let user = self.users.get(&user_id);
        if user.is_none() {

            env::panic_str(format!("User {} not found", user_id).as_str());
     
        }

        self.users.remove(&user_id);
    }


    pub fn remove_all_users (&mut self) {
        
        Self::check_if_authorized();

        self.users.clear();
    }

}