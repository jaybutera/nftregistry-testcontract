#![feature(proc_macro_hygiene)]
#![cfg_attr(not(feature = "std"), no_std)]

use ink_prelude::{vec, vec::Vec};
use ink_core::env2::call::{
    Selector,
    CallData,
};
use ink_lang2 as ink;

#[ink::contract(version = "0.1.0")]
mod testcontract {
    #[ink(storage)]
    struct Testcontract {}

    #[ink(event)]
    struct Foo {
        #[ink(topic)]
        from: Option<AccountId>,
        value: Balance,
    }

    impl Testcontract {
        #[ink(constructor)]
        fn new(&mut self) {}

        #[ink(message)]
        fn validate(&self, nft_registry_id: AccountId) -> Vec<u8> {
            let mut call = CallData::new(
                Selector::from_str("finish_mint") );
            let caller = self.env().caller();
            self.env()
                .emit_event(
                    Foo {
                        from: Some(caller),
                        value: 100,
                    });

            // Origin is this contract, not the caller
            //call.push_arg( &self.env().address() );

            // Specify nft type to mint
            call.push_arg(&nft_registry_id);

            let x = self.env()
                .invoke_runtime(&call);

            call.to_bytes().to_vec()
        }
    }

    #[cfg(test)]
    mod tests {
        /// Imports all the definitions from the outer scope so we can use them here.
        use super::*;
        #[test]
        fn it_works() {
            let mut testcontract = Testcontract::new();
        }
        #[test]
        fn validate_encoding() {
            let mut testcontract = Testcontract::new();
            let account: AccountId = [0u8; 32].into();
            println!("{:?}", testcontract.validate(account));
        }
    }
}

pub use crate::testcontract::Testcontract;
