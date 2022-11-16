use crate::*;


#[near_bindgen]
impl Contract {

    #[payable]
    pub fn ticket_mint (&mut self, collection_id : CollectionId, token_id : TokenId,
    ticket_image : String,ticket_type : Option<TicketType>, extra  : Option<String>) {
    
        
        self.ticket_mint_check_signup(collection_id, token_id, ticket_image, ticket_type,
        extra); 
    
    }

}

#[near_bindgen]
impl Contract {

    /**
     * Reserve function for implementation of if a ticket collection required 
     * the check if the buyer has signed up
     */
    #[allow(dead_code)]
    #[payable]
    fn ticket_mint_check_signup (&mut self, collection_id : CollectionId, token_id : TokenId,
        ticket_image : String,ticket_type : Option<TicketType>, extra  : Option<String>){


        collections_contract::ext(Self::get_collections_contract_id(self.collections_contract_id.clone()))
        .with_static_gas(Gas(5*TGAS))
        .get_collection(collection_id.clone())
        .then(
            Self::ext(env::current_account_id())
            .with_static_gas(Gas(CALLBACK_TGAS))
            .with_attached_deposit(env::attached_deposit())
            .after_getting_collection_callback(collection_id, token_id,
            ticket_image, ticket_type, extra)
            .as_return()
        );

    
    }

    #[private]
    pub fn after_getting_collection_callback(&self,
    collection_id : CollectionId, token_id : TokenId,
    ticket_image : String,ticket_type : Option<TicketType>, extra  : Option<String>,
    #[callback_result] call_result: Result<Collection, PromiseError>) 
    {

        if call_result.is_err() {

            env::panic_str("Error!@x");
        }

     
        let collection : Collection = call_result.unwrap();

        if collection.attributes.is_some() {

            let uw_attribs = collection.attributes.unwrap();

            let req_to_sign_up = Attribute {
                name : AttributeType::BuyerRequiredToSignUp,
                value : "none".to_string(),
            };

            let index = uw_attribs.iter().position(|a| *a == req_to_sign_up);

            if index.is_some() {

                if uw_attribs[index.unwrap()].value == "true".to_string() {

                    if !self.has_user(&env::signer_account_id().to_string()) {

                        env::panic_str(format!("User {} does NOT exist!",
                        env::signer_account_id()).as_str());
                    }
            
                }
            }
            
        }

        collections_contract::ext(Self::get_collections_contract_id(self.collections_contract_id.clone()))
        .with_static_gas(Gas(5*TGAS))
        .with_attached_deposit(env::attached_deposit())
        .ticket_mint(collection_id, token_id, ticket_image, ticket_type, extra, 
        None).as_return();
   
        
      
    }

}

#[near_bindgen]
impl Contract {

    /*
    Reserved function for the check of if buyer has signed up when minting a ticket
    */
    #[allow(dead_code)]
    fn ticket_mint_force_signup (&mut self, collection_id : CollectionId, token_id : TokenId,
        ticket_image : String,ticket_type : Option<TicketType>, extra  : Option<String>){


        if !self.has_user(&env::signer_account_id().to_string()) {

            env::panic_str(format!("User {} does NOT exist!",
            env::signer_account_id()).as_str());
        }

        collections_contract::ext(Self::get_collections_contract_id(self.collections_contract_id.clone()))
        .with_static_gas(Gas(5*TGAS))
        .with_attached_deposit(env::attached_deposit())
        .ticket_mint(collection_id, token_id, ticket_image, ticket_type, extra, 
        None).as_return();
    
    }
}