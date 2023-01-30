extern crate rand;
extern crate ed25519_dalek;

use crate::helpers::{Storage, encrypt};

use rand::rngs::OsRng;
use ed25519_dalek::{Keypair};

#[derive(Debug)]
pub struct Account {
    pub_key : &'static str,
    sec_key : &'static str,
    storage : Option<Storage>
}

impl Account {
    pub fn create_account() -> Account {
        let mut csprng = OsRng{};
        let keypair : Keypair = Keypair::generate(&mut csprng);

        let pub_key_bytes : [u8; 32] = keypair.public.to_bytes();
        let sec_key_bytes : [u8; 32] = keypair.secret.to_bytes();

        Account {
            pub_key : encrypt(&pub_key_bytes).as_str(),
            sec_key : encrypt(&sec_key_bytes).as_str(),
            storage : None
        }
    }
}