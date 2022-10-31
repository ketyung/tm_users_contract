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



#[derive(Debug, Clone, Serialize, Deserialize, BorshDeserialize, BorshSerialize)]
#[serde(crate = "near_sdk::serde")]
pub enum TicketTemplateType {

    Fixed,

    Custom,
}

#[derive(Debug, Clone, Serialize, Deserialize, BorshDeserialize, BorshSerialize)]
#[serde(crate = "near_sdk::serde")]
pub struct TicketTemplate {

    pub value : String, 

    pub template_type : TicketTemplateType,

}


#[derive(Debug, Clone, Serialize, Deserialize, BorshDeserialize, BorshSerialize)]
#[serde(crate = "near_sdk::serde")]
pub enum AttributeType {

    StartDate,

    EndDate, 
    
    MaxTicketPerWallet,

    Venue,

    LocationCoord,

    SalesPageTemplate,
}



#[derive(BorshDeserialize, BorshSerialize,Serialize, Deserialize, Debug)]
#[serde(crate = "near_sdk::serde")]
pub struct Attribute{

    pub name : AttributeType,

    pub value : String, 
}



#[derive(BorshDeserialize, BorshSerialize,Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct TicketType{

    pub ticket_type : String,

    // ticket price is stored as u32 
    // the stored value is always divided by 1000
    // e.g. 3.2 Near token is stored as 3200 
    pub price : u32, 
}



#[derive(BorshDeserialize, BorshSerialize,Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct Collection{

    pub title : String, 

    pub symbol : String,

    pub icon : Option<String>,

    pub base_uri : Option<String>,
    
    pub description : Option<String>, 

    pub category : Option<String>, 

    pub ticket_types : Option<Vec<TicketType>>,

    pub total_tickets : Option<u64>,
    
    pub tickets_sold : Option<u64>,

    pub attributes : Option<Vec<Attribute>>,

    pub ticket_template_type : Option<TicketTemplate>,
    
}


#[derive(BorshDeserialize, BorshSerialize,Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct CollectionId {

    pub title : String, 

    pub symbol : String, 
    
    pub owner : AccountId, 
}



#[derive(BorshDeserialize, BorshSerialize,Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct CollectionDataForUpdate{

    pub description : Option<String>, 
    // the icon/logo of the collection
    pub icon : Option<String>,

    pub base_uri : Option<String>,
    
    pub ticket_types : Option<Vec<TicketType>>,

    pub total_tickets : Option<u64>,
    
    pub attributes : Option<Vec<Attribute>>,

    pub ticket_template_type : Option<TicketTemplate>,

    pub category : Option<String>,

}
