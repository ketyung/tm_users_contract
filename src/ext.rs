use crate::*;

pub const TGAS: u64 = 1_000_000_000_000;
pub const NO_DEPOSIT: u128 = 0;
pub const XCC_SUCCESS: u64 = 1;


// For cross-contract calls
#[ext_contract(nft_contract)]
trait NftContract {
  
    fn init_with_metadata(owner_id: AccountId, name : String, symbol : String,
        icon : Option<String>, base_uri : Option<String> );    
}


// For cross-contract calls to create the collection
// in the collection store contract
#[ext_contract(collections_contract)]
trait CollectionsContract {
  
    fn create_collection (
        acc_id : AccountId,     
        title : String, 
        symbol : String,
        icon : Option<String>,
        base_uri : Option<String>,
        description : Option<String>,
        category : Option<String>,
        total_tickets : Option<u64>,
        tickets_sold : Option<u64>,
        ticket_types : Option<Vec<TicketType>>,
        attributes : Option<Vec<Attribute>>,
        template_type : Option<TicketTemplate>,
        contract_id : Option<AccountId>);

}



