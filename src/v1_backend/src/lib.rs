#[macro_use]
extern crate serde;
// use serde::{Deserialize, Serialize}; // Import Deserialize and Serialize traits
use candid::{Decode, Encode};
// use ic_cdk::api::time;
use ic_stable_structures::memory_manager::{MemoryId, MemoryManager, VirtualMemory};
use ic_stable_structures::{BoundedStorable, Cell, DefaultMemoryImpl, StableBTreeMap, Storable};
use std::{borrow::Cow, cell::RefCell};


// defining our memory and id cell types
//we will use it to store the canister statet and generate some unique id  for each Vendor 
type Memory = VirtualMemory<DefaultMemoryImpl>;
type IdCell = Cell<u64, Memory>;
// type PhoneCell = Cell<String, Memory>;
//end of memory definistions 

#[derive(candid::CandidType, Clone, Serialize, Deserialize, Default)]
struct Vendor {
    id: u64,
    name: String,
    phone: String,
    current_location: String,
}

#[derive(candid::CandidType, Clone, Serialize, Deserialize, Default)]
struct Excess {
    id: u64,
    vendor_id:  u64,
    name: String,
    amount: String, //amount in kg they have in excess 
    date: String,  //date of excess
  
}


/*
|| end of data structure definitions ..
*/

/*
| Implementation of the Storable and BoundedStorable traits.
| These traits will be used to store the struct in a table.
*/
impl Storable for Vendor {
    // Convert the struct to bytes
    //Cow::borrow retursn a copy of data which cant be alterd 
    fn to_bytes(&self) -> std::borrow::Cow<[u8]> {
        // Encoding the struct into bytes and owning the result
        Cow::Owned(Encode!(self).unwrap()) // returns a copy of data which can be modifird 
    }

    // Convert bytes back to the struct
    fn from_bytes(bytes: std::borrow::Cow<[u8]>) -> Self {
        // Decoding bytes into the struct
        Decode!(bytes.as_ref(), Self).unwrap()
    }
}
//excess storable 
impl Storable for Excess {
    // Convert the struct to bytes
    //Cow::borrow retursn a copy of data which cant be alterd 
    fn to_bytes(&self) -> std::borrow::Cow<[u8]> {
        // Encoding the struct into bytes and owning the result
        Cow::Owned(Encode!(self).unwrap()) // returns a copy of data which can be modifird 
    }

    // Convert bytes back to the struct
    fn from_bytes(bytes: std::borrow::Cow<[u8]>) -> Self {
        // Decoding bytes into the struct
        Decode!(bytes.as_ref(), Self).unwrap()
    }
}

// Define the maximum size of the struct to make it flexible, not fixed in size
impl BoundedStorable for Vendor {
    // Maximum size of the struct
    const MAX_SIZE: u32 = 1024;
    // Indicate that the struct size is not fixed
    const IS_FIXED_SIZE: bool = false;
}
//memory manage for excess 
impl BoundedStorable for Excess {
    // Maximum size of the struct
    const MAX_SIZE: u32 = 1024;
    // Indicate that the struct size is not fixed
    const IS_FIXED_SIZE: bool = false;
}

/*
|| End of Storable and BoundedStorable implementations.
*/
/*
|| Start setting threadslocal variables 
|| the thread local are use to manage the memor, maintain counter and store data in rust 
*/


thread_local! {
    //intialise the memory manager using the default memory default configuaration 
    static MEMORY_MANAGER: RefCell<MemoryManager<DefaultMemoryImpl>> = RefCell::new(
        MemoryManager::init(DefaultMemoryImpl::default())
    );

    //initialise the idcell with zero 
    static ID_COUNTER: RefCell<IdCell> = RefCell::new(
        IdCell::init(MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(0))), 0)
            .expect("cannot create a counter")
    );

    // will store Vendor objects indexed by u64 keys 
    static VENDOR_STORAGE: RefCell<StableBTreeMap<u64, Vendor, Memory>> = RefCell::new(
        StableBTreeMap::init(MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(1))))
    );
    // will store excess details 
      static EXCESS_STORAGE: RefCell<StableBTreeMap<u64, Excess, Memory>> = RefCell::new(
        StableBTreeMap::init(MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(1))))
    );
}


/*
|| end  setting threadslocal variables 
*/


/*
|| setting the Vendor payload 
|| it is used when adding or updatind Vendor adn includes fields for 
*/

#[derive(candid::CandidType, Serialize, Deserialize, Default)]
struct VendorPayload {
    name: String,
    phone: String,
    current_location: String,

}

//EXCESS PAYLOAD 


#[derive(candid::CandidType, Serialize, Deserialize, Default)]
struct ExcessPayload {
    vendor_id:  u64,
    name: String,
    amount: String, //amount in kg they have in excess 
    date: String,  //date of excess
}


/*
|| Managing the Vendor 
|| this section contains core Vendor function to intereact with the canister 
*/

// get Vendors from the canister

// #[ic_cdk::query]
// fn get_Vendor(phone: String) -> String {
   
// }


// get Vendors by 
#[ic_cdk::query]
fn get_vendor_from_id(id: u64) -> Result<Vendor, Error> {
    match _get_vendor(&id){
        Some(Vendor) =>Ok(Vendor),
        None =>Err(Error::NotFound{
            msg: format!("Vendor with the id ={}  does not exist ", id),
        }),
    }

   
}
//get excess supplies by id 
#[ic_cdk::query]
fn get_excess_from_id(id: u64) -> Result<Excess, Error> {
    match _get_excess(&id){
        Some(Excess) =>Ok(Excess),
        None =>Err(Error::NotFound{
            msg: format!("Excess with the id ={}  does not exist ", id),
        }),
    }

   
}



// add  a new Vendor function 
#[ic_cdk::update]
fn add_Vendor(Vendor: VendorPayload) ->Option<Vendor>{
    //generat a unique id for the Vendor ..
    let id = ID_COUNTER
        //this another definition of anonymus function with a variabe known as  the counter 
        .with(|counter| {
            let current_value = *counter.borrow().get();
            counter.borrow_mut().set(current_value +1)

        })
        //handle the exception accordingly 
        .expect("can increment the Vendor id");

    let Vendor = Vendor{
        id,
        name: Vendor.name,
        phone: Vendor.phone,
        current_location: Vendor.current_location,

    };
    do_insert(&Vendor);
    Some(Vendor)
}
//add a new excess 
// add  a new Vendor function 
#[ic_cdk::update]
fn add_excess(Excess: ExcessPayload) ->Option<Excess>{
    //generat a unique id for the Vendor ..
    let id = ID_COUNTER
        //this another definition of anonymus function with a variabe known as  the counter 
        .with(|counter| {
            let current_value = *counter.borrow().get();
            counter.borrow_mut().set(current_value +1)

        })
        //handle the exception accordingly 
        .expect("can increment the excess id");
     

    let Excess = Excess{
        id,
        vendor_id: Excess.vendor_id,
        name: Excess.name,
        amount: Excess.amount,
        date: Excess.date,


    };
    do_insert_excess(&Excess);
    Some(Excess)
}

//update a Vendor function 
//on;y allow Vendor to update his/her phone number and the current location ..
#[ic_cdk::update]
fn  update_Vendor(id: u64, payload: VendorPayload) -> Result<Vendor, Error>{
    match VENDOR_STORAGE.with(|service| service.borrow().get(&id)){
        Some(mut Vendor) =>{
            Vendor.phone            = payload.phone;
            Vendor.name             = payload.name;
            Vendor.current_location = payload.current_location;
            do_insert(&Vendor);
            Ok(Vendor)
        }
        //no Vendor with such an id exists 
        None => Err(Error::NotFound{
            msg: format!(
                "could not update the Vendor with the id {}, usr not found", id
            ),
        }),
    }
}

//delete Vendor function 
//deletes Vendor based on his/her id 
#[ic_cdk::update]
fn delete_Vendor(id: u64) ->Result <Vendor, Error>{
    match VENDOR_STORAGE.with(|service| service.borrow_mut().remove(&id)){
        Some(Vendor) =>Ok(Vendor),
        None => Err(Error::NotFound{
            msg: format!(
                "could not delete the Vendor withe the id {}  Vendor not found",
                id
            ),
        })
    }
}

//error handling functions ..

#[derive(candid::CandidType, Deserialize, Serialize)]
enum Error {

    NotFound { msg: String },

}


#[ic_cdk::update]
fn delete_excess(id: u64) ->Result <Excess, Error>{
    match EXCESS_STORAGE.with(|service| service.borrow_mut().remove(&id)){
        Some(Excess) =>Ok(Excess),
        None => Err(Error::NotFound{
            msg: format!(
                "could not delete the excess withe the id {}  Vendor not found",
                id
            ),
        })
    }
}





/*
|| Vendor Helper functions section  
|| 
*/


//defining the helper function to help in the insertion process ..
fn do_insert(Vendor: &Vendor){
    VENDOR_STORAGE.with(|service| service.borrow_mut().insert(Vendor.id, Vendor.clone()));
}
//excess 
fn do_insert_excess(Excess: &Excess){
    EXCESS_STORAGE.with(|service| service.borrow_mut().insert(Excess.id, Excess.clone()));
}
//definign the get message helper function 
//defining a private function to ne use within ht canister hene the leading _
fn _get_vendor(id: &u64) -> Option<Vendor> {
    //some anony,us function over here
    VENDOR_STORAGE.with(|s| s.borrow().get(id))
}
//excess aux function 
fn _get_excess(id: &u64) -> Option<Excess> {
    //some anony,us function over here
    EXCESS_STORAGE.with(|s| s.borrow().get(id))
}
//enim function to handle our errors 




#[ic_cdk::query]
fn register(details: Vendor) -> Vendor {
    details
}

 // lets genereate the candid file 

 ic_cdk::export_candid!();
