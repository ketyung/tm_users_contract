use crate::*;


#[near_bindgen]
impl Contract {


    fn get_sub_account_id (sub_account_id : String ) -> AccountId{

        let acc_id : AccountId = format!("{}_{}.{}", 
        sub_account_id.to_lowercase(),
        env::signer_account_id().as_str().to_string().replace(".","_"),
        env::current_account_id()).parse().unwrap();

        acc_id 
    }

    fn get_collections_contract_id(collections_contract_id : Option<AccountId>) -> AccountId {

        if collections_contract_id.is_some(){

            return collections_contract_id.unwrap();
        }

        COLLECTIONS_CONTRACT_ID.parse().unwrap()
    }

}


const NFT_CONTRACT_CODE: &[u8] = include_bytes!("./wasm/non_fungible_token.wasm");

const DATA_IMAGE_SVG_NEAR_ICON: &str = "data:image/svg+xml,%3Csvg xmlns='http://www.w3.org/2000/svg' viewBox='0 0 288 288'%3E%3Cg id='l' data-name='l'%3E%3Cpath d='M187.58,79.81l-30.1,44.69a3.2,3.2,0,0,0,4.75,4.2L191.86,103a1.2,1.2,0,0,1,2,.91v80.46a1.2,1.2,0,0,1-2.12.77L102.18,77.93A15.35,15.35,0,0,0,90.47,72.5H87.34A15.34,15.34,0,0,0,72,87.84V201.16A15.34,15.34,0,0,0,87.34,216.5h0a15.35,15.35,0,0,0,13.08-7.31l30.1-44.69a3.2,3.2,0,0,0-4.75-4.2L96.14,186a1.2,1.2,0,0,1-2-.91V104.61a1.2,1.2,0,0,1,2.12-.77l89.55,107.23a15.35,15.35,0,0,0,11.71,5.43h3.13A15.34,15.34,0,0,0,216,201.16V87.84A15.34,15.34,0,0,0,200.66,72.5h0A15.35,15.35,0,0,0,187.58,79.81Z'/%3E%3C/g%3E%3C/svg%3E";

const COLLECTIONS_CONTRACT_ID :  &'static str = "tm_collections_contract.testnet";

#[near_bindgen]
impl Contract {


    pub (crate) fn to_actual_amount(init_balance : u32) -> f64 {
        
        init_balance as f64 / 1000.00
    }

    #[payable]
    pub fn create_collection_and_deploy(
        &mut self, 
        init_balance : u32, 
        collection : Collection ){


        self.is_signer_registered();

        
        let contract_id = Self::get_sub_account_id(collection.symbol.clone());

        // allow for 3 decimals, which means
        let actual_init_bal = Self::to_actual_amount(init_balance);

        // sending amount 1000 = 1 Near, 1020 = 1.020 Near, 2031 = 2.031 NEAR etc
        let bal : Balance = (actual_init_bal * (NEAR as f64)) as u128 ;

        // mark up 2 percent
        let req_deposit = (actual_init_bal * MARKUP_PERC) * (NEAR as f64);

        if env::attached_deposit() < (req_deposit as u128) {

            // refund deposit
            Promise::new(env::signer_account_id())
            .transfer(bal).as_return();

            env::panic_str(format!("Deposit is less than the required amount {} NEAR", req_deposit).as_str());
        }

    
        Promise::new(contract_id.clone())
        .create_account()
        .transfer(bal)
        .add_full_access_key(env::signer_account_pk())
        .deploy_contract(NFT_CONTRACT_CODE.to_vec())
        .then(

            Self::ext(env::current_account_id())
            .with_static_gas(Gas(CALLBACK_TGAS))
            .nft_contract_deploy_callback(contract_id, bal, collection, self.collections_contract_id.clone())
        );
    
    }
}


#[near_bindgen]
impl Contract {

    #[private]
    pub fn nft_contract_deploy_callback(
        contract_id : AccountId,
        init_balance : Balance,
        collection : Collection , 
        collections_contract_id : Option<AccountId>,
        #[callback_result] call_result: Result<(), PromiseError>) {


        if call_result.is_err() {

            // transfer back the init balance to the signer
            Promise::new(env::signer_account_id())
            .transfer(init_balance).as_return();
            
            env::log_str(
            format!("return at nft_contract_deploy_callback & tx back init_balanace {}", 
            init_balance).as_str());
            return;
        }

        let mut icon = collection.icon.clone();
        if icon.is_none() {

            icon = Some(DATA_IMAGE_SVG_NEAR_ICON.to_string());
        }

    
        env::log_str(format!("Going to init contract : {}", contract_id).as_str());

        nft_contract::ext(contract_id.clone())
        .with_static_gas(Gas(5*TGAS))
        .init_with_metadata(env::signer_account_id(), 
        collection.title.clone(), collection.symbol.clone(), icon, 
        collection.base_uri.clone())
        .then(

            Self::ext(env::current_account_id())
            .with_static_gas(Gas(CALLBACK_TGAS))
            .nft_contract_init_callback(contract_id, collection, collections_contract_id)
        );
    


    }



    #[private]
    pub fn nft_contract_init_callback(
        contract_id : AccountId,
        collection : Collection, 
        collections_contract_id : Option<AccountId>,
        #[callback_result] call_result: Result<(), PromiseError>) {

        if call_result.is_err() {

            env::panic_str(format!("Failed to initialize contract {}!", contract_id.clone()).as_str());
        }

        env::log_str(format!("NFT contract {} has been initialized!", contract_id.clone()).as_str());

        collections_contract::ext(Self::get_collections_contract_id(collections_contract_id))
        .with_static_gas(Gas(5*TGAS))
        .create_collection(env::signer_account_id(), collection.title.clone(),
        collection.symbol, 
        collection.icon, collection.base_uri,
        collection.description, 
        collection.category,
        collection.total_tickets,
        collection.tickets_sold,
        collection.ticket_types,
        collection.attributes,
        collection.ticket_template_type, 
        Some(contract_id))
        .then(

            Self::ext(env::current_account_id())
            .with_static_gas(Gas(CALLBACK_TGAS))
            .nft_collection_create_callback(collection.title)
        );


    }


    #[private]
    pub fn nft_collection_create_callback(
        collection_title : String, 
        #[callback_result] call_result: Result<(), PromiseError>) 
    {

        if call_result.is_err() {

            env::panic_str(format!("Failed to create collection '{}' in NFT collection store!", 
            collection_title).as_str());
        }


        env::log_str(format!("Collection {} has been created in NFT collection store!", 
        collection_title).as_str());

    }


}



#[near_bindgen]
impl Contract {

    // A wrapper function for cross-contract calling to
    // update the collection
    pub fn update_collection (&self, collection_id : CollectionId,
        update_collection_data : crate::models::CollectionDataForUpdate){

        collections_contract::ext(Self::get_collections_contract_id(self.collections_contract_id.clone()))
        .with_static_gas(Gas(5*TGAS))
        .update_collection(collection_id,update_collection_data).as_return();
    
    }
}
