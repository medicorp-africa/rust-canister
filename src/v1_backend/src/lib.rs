#[macro_use]
extern crate serde;
// use serde::{Deserialize, Serialize}; // Import Deserialize and Serialize traits
use candid::{Decode, Encode};
// use ic_cdk::api::time;
use ic_stable_structures::memory_manager::{MemoryId, MemoryManager, VirtualMemory};
use ic_stable_structures::{BoundedStorable, Cell, DefaultMemoryImpl, StableBTreeMap, Storable};
use std::{borrow::Cow, cell::RefCell};


// defining our memory and id cell types
//we will use it to store the canister statet and generate some unique id  for each user 
type Memory = VirtualMemory<DefaultMemoryImpl>;
type IdCell = Cell<u64, Memory>;
type PhoneCell = Cell<String, Memory>;
//end of memory definistions 

#[derive(candid::CandidType, Clone, Serialize, Deserialize, Default)]
struct User {
    id: u64,
    name: String,
    age: String,
    password: String,
    phone: String,
    date_of_birth: String,
    current_location: String,
}


/*
|| end of data structure definitions ..
*/

/*
| Implementation of the Storable and BoundedStorable traits.
| These traits will be used to store the struct in a table.
*/
impl Storable for User {
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
impl BoundedStorable for User {
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

    // will store user objects indexed by u64 keys 
    static STORAGE: RefCell<StableBTreeMap<u64, User, Memory>> = RefCell::new(
        StableBTreeMap::init(MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(1))))
    );
}


/*
|| end  setting threadslocal variables 
*/


/*
|| setting the user payload 
|| it is used when adding or updatind user adn includes fields for 
*/

#[derive(candid::CandidType, Serialize, Deserialize, Default)]
struct UserPayload {
    name: String,
    age: String,
    password: String,
    phone: String,
    date_of_birth: String,
    current_location: String,

}


/*
|| Managing the user 
|| this section contains core user function to intereact with the canister 
*/

// get users from the canister

// #[ic_cdk::query]
// fn get_user(phone: String) -> String {
   
// }


// get users by 
#[ic_cdk::query]
fn get_user_from_id(id: u64) -> Result<User, Error> {
    match _get_user(&id){
        Some(user) =>Ok(user),
        None =>Err(Error::NotFound{
            msg: format!("user with the id ={}  does not exist ", id),
        }),
    }

   
}

// add  a new user function 
#[ic_cdk::update]
fn add_user(user: UserPayload) ->Option<User>{
    //generat a unique id for the user ..
    let id = ID_COUNTER
        //this another definition of anonymus function with a variabe known as  the counter 
        .with(|counter| {
            let current_value = *counter.borrow().get();
            counter.borrow_mut().set(current_value +1)

        })
        //handle the exception accordingly 
        .expect("can increment the user id");

    let user = User{
        id,
        name: user.name,
        age: user.age,
        password: user.password,
        phone: user.phone,
        date_of_birth: user.date_of_birth,
        current_location: user.current_location,

    };
    do_insert(&user);
    Some(user)
}

//update a user function 
//on;y allow user to update his/her phone number and the current location ..
#[ic_cdk::update]
fn  update_user(id: u64, payload: UserPayload) -> Result<User, Error>{
    match STORAGE.with(|service| service.borrow().get(&id)){
        Some(mut user) =>{
            user.phone            = payload.phone;
            user.name             = payload.name;
            user.password         = payload.password;
            user.date_of_birth    = payload.date_of_birth;
            user.current_location = payload.current_location;
            do_insert(&user);
            Ok(user)
        }
        //no user with such an id exists 
        None => Err(Error::NotFound{
            msg: format!(
                "could not update the user with the id {}, usr not found", id
            ),
        }),
    }
}

//delete user function 
//deletes user based on his/her id 
#[ic_cdk::update]
fn delete_user(id: u64) ->Result <User, Error>{
    match STORAGE.with(|service| service.borrow_mut().remove(&id)){
        Some(user) =>Ok(user),
        None => Err(Error::NotFound{
            msg: format!(
                "could not delete the user withe the id {}  user not found",
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





/*
|| User Helper functions section  
|| 
*/


//defining the helper function to help in the insertion process ..
fn do_insert(user: &User){
    STORAGE.with(|service| service.borrow_mut().insert(user.id, user.clone()));
}
//definign the get message helper function 
//defining a private function to ne use within ht canister hene the leading _
fn _get_user(id: &u64) -> Option<User> {
    //some anony,us function over here
    STORAGE.with(|s| s.borrow().get(id))
}
//enim function to handle our errors 




#[ic_cdk::query]
fn register(details: User) -> User {
    details
}

 // lets genereate the candid file 

 ic_cdk::export_candid!();
