use crate::*;


#[near_bindgen]
impl Contract {

    pub fn ticket_mint (&self, collection_id : CollectionId, token_id : TokenId,
    ticket_image : String,ticket_type : Option<TicketType>, extra  : Option<String>) {
    
        
        if !self.has_user(&env::signer_account_id().to_string()) {

            env::panic_str(format!("User {} does NOT exist!",
            env::signer_account_id()).as_str());
        }

        collections_contract::ext(Self::get_collections_contract_id(self.collections_contract_id.clone()))
        .with_static_gas(Gas(5*TGAS))
        .ticket_mint(collection_id, token_id, ticket_image, ticket_type, extra, 
        None).as_return();
    }

}
