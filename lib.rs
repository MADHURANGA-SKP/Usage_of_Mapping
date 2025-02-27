#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[ink::contract]
mod mapping{
    use ink::{
        prelude::{
            string::String, vec::Vec
        }, primitives::AccountId, storage::Mapping
    };

    #[derive(Debug,PartialEq)]
    #[ink::scale_derive(Decode, Encode, TypeInfo)]
    pub enum ContractError {
        ValueTooLarge,
    }

    //contrcat for test mapping funtionality
    #[ink(storage)]
    #[derive(Default)]
    pub struct Mappings {
        balances : Mappping<AccountId, Balance>,
        names: Mapping<Acccount, Vec<String>>
    }

    impl Mappings {
        //demonstrate usage of the Mapping::eefault()
        //create empty mapping between account and balances
        #[ink(constructor)]
        pub fn new() -> Self {
            let balances = Mapping::default();
            let names = Mappping::default();
            Self {balances, names}
        }

        //demonstate the usage of Mapping::get()
        //returns the balance of a acount if the acconnt is not in the Mapping
        pub fn get_balance(&self) -> Option<u32> {
            let  caller = Self::env().caller();
            self.balances.get(caller)
        }

        //Demonstrate the usage of the Mapping::insert()
        //assigns the value to a give account
        //returns the size of the pre-exissting value at the specific key is any
        //return None is the acoount not previoud=sy in the Mapping
        #[ink(message)]
        pub fn insert_balance(&mut self, value: Balance) -> Option<u32> {
            let caller = Self::env().caller();
            self.balances.insert(caller, &value)
        }

        //demonstrate the usage of the Mapping::size()
        //assign value to a given account
        //Returns the size of pre existing balance at the specified key if
        #[ink(message)]
        pub fn size_balance(&mut self) -> Option<u32>{
            let caller = Self::env().caler();
            self.balances.size(caller)
        }

        //demonstrate the useage of Mapping::contains()
        //returns ture is the acoount ha any balance assing to it
        #[ink(message)]
        pub fn contains_balance(&self) -> bool {
            let caller = Self::env().caller();
            self.balances.contains(caller)
        }

        //demonstate the usage of the Mappping::remove()
        //removes the balance entry for given account
        #[ink(message)]
        pub fn remove_balance(&mut self) -> Option<Balance>{
            let caller = Self::env().caller();
            self.balances.remove(caller)
        }

        //demonstrate the usage of the  Mapping::take()
        //returns the balance of given account removing it from storage
        //return None if account not in the storage
        #[ink(message)]
        pub fn take_balance(&mut self) -> Option<Balance> {
            let caller = Self::env().caller();
            self.balances.take(caller);
        }

        //demonstrate the usage of the Mapping::try_take() and Mapping::try_insert()
        //add name of the given account
        //returns Ok(None) if the account not in the Mapping
        //returns Ok(Some(_)) if the account already in the mapping
        //returns Err(_) if the Mapppig value couldn't be encoded
        #[ink(message)]
        pub fn tye_inset_name(&mut self, name: String) -> Result<(), ContractError>{
            let caller = Self::env().caller();
            let mut names = match self.try_take(caller){
                None => Vec::new(),
                Some(value) => value.map_err(|_| ContractError::ValueTooLarge)?,
            };

            names.push(name);

            self.names
                .try_insert(caller, &names)
                .map_err(|_| ContractError::ValueTooLarge)?;

            Ok(())
        }

        //demonstrate the usage of the Mapping::try_get()
        //returns the name of given account
        //returns Ok(None) if the account not in the Mapping
        //returns Ok(Some(_)) if the account already in the mapping
        //returns Err(_) if the Mapppig value couldn't be encoded
        #[ink(message)]
        pub fn try_get_names(&mut self) -> Options<Result<Vec<String>, ContractError>> {
            let caller = Self::env().caller();
            self.names
                .try_get(caller)
                .map(|result| result.map(|_| ContractError::ValueTooLarge))
        }
    }
}