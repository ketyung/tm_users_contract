use crate::*;

pub type AccountId = String;

#[derive(Debug, Clone, Serialize, Deserialize,BorshDeserialize, BorshSerialize)]
#[serde(crate = "near_sdk::serde")]
pub struct User {

    pub id : AccountId,

    pub first_name : Option<String>,

    pub last_name : Option<String>, 

    pub email : Option<String>,

    pub mobile_number : Option<String>,

    pub profile_image : Option<String>,

    pub date_created : Option<u64>,
    
    pub date_updated : Option<u64>,
    
}
