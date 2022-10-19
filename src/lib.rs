pub mod models;
pub mod ext;
pub mod users_view;
pub mod nft_deploy;

use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{near_bindgen, env, ext_contract, BorshStorageKey,AccountId, Balance, Promise, Gas, PromiseError};
use near_sdk::collections::{UnorderedMap};
use near_sdk::serde::{Serialize, Deserialize};
use crate::models::*;
use ext::*;


const NEAR: u128 = 1000_000_000_000_000_000_000_000;

const MARKUP_PERC : f64 = 1.02;

// Define the contract structure
#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct Contract {
    
    users: UnorderedMap<String, User>,

    events_contract_id : Option<AccountId>,

    date_updated : Option<u64>, 
}

#[derive(BorshStorageKey, BorshSerialize)]
enum StorageKey {

    UserStorageKey,
}

// Define the default, which automatically initializes the contract
impl Default for Contract{
    fn default() -> Self{
        Self{users: UnorderedMap::new(StorageKey::UserStorageKey), 
            events_contract_id : None, 
            date_updated : Some(env::block_timestamp()) }
    }
}

#[near_bindgen]
impl Contract {

    #[init]
    #[private] // Public - but only callable by env::current_account_id()
    #[allow(dead_code)]
    pub (crate) fn init() -> Self {
        assert!(!env::state_exists(), "Already initialized");
        Self{users: UnorderedMap::new(StorageKey::UserStorageKey), 
        events_contract_id : None, 
        date_updated : Some(env::block_timestamp()) }
    }

}


#[near_bindgen]
impl Contract {

    #[init]
    #[private] // Public - but only callable by env::current_account_id()
    #[allow(dead_code)]
    pub fn init_with(_events_contract_id : AccountId) -> Self {
        assert!(!env::state_exists(), "Already initialized");
        Self{users: UnorderedMap::new(StorageKey::UserStorageKey), 
        events_contract_id : Some(_events_contract_id), 
        date_updated : Some(env::block_timestamp()) }
    }

}

#[near_bindgen]
impl Contract {

    fn is_signer_registered(&self){

        if !self.has_user(&env::signer_account_id().as_str().to_string()) {

            env::panic_str(format!("User [{}] does NOT exist!", env::signer_account_id()).as_str());
      
        }
    }
}